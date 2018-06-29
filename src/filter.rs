use std::cmp::Ordering;
use std::ops::Bound;
use std::{f32, f64, i16, i32, i64, i8};

use ieee754::Ieee754;

use bounds::IntoBounds;
use pb::ColumnPredicatePb;
use Column;
use Error;
use PhysicalType;
use Result;
use Value;

#[derive(Clone, Debug, PartialEq)]
pub enum Filter {
    All,
    None,
    IsNull,
    IsNotNull,
    Equals {
        physical_type: PhysicalType,
        value: Vec<u8>,
    },
    Range {
        physical_type: PhysicalType,
        /// Inclusive lower bound.
        lower_bound: Option<Vec<u8>>,
        /// Exclusive upper bound.
        upper_bound: Option<Vec<u8>>,
    },
    In {
        physical_type: PhysicalType,
        values: Vec<Vec<u8>>,
    },
}

impl Filter {
    pub(crate) fn all() -> Filter {
        Filter::All
    }

    pub(crate) fn none() -> Filter {
        Filter::None
    }

    pub fn is_null() -> Filter {
        Filter::IsNull
    }

    pub fn is_not_null() -> Filter {
        Filter::IsNotNull
    }

    pub fn equals<V>(value: V) -> Filter
    where
        for<'data> V: Value<'data>,
    {
        Filter::Equals {
            physical_type: V::PHYSICAL_TYPE,
            value: value.encode(),
        }.simplify()
    }

    pub fn range<'data, V, R>(range: R) -> Filter
    where
        V: Value<'data>,
        R: IntoBounds<V>,
    {
        let physical_type = V::PHYSICAL_TYPE;
        let (start, end) = range.into_bounds();
        let lower_bound = match start {
            Bound::Included(value) => Some(value.encode()),
            Bound::Excluded(value) => value.encode_next(),
            Bound::Unbounded => None,
        };
        let upper_bound = match end {
            Bound::Included(value) => value.encode_next(),
            Bound::Excluded(value) => Some(value.encode()),
            Bound::Unbounded => None,
        };

        Filter::Range {
            physical_type,
            lower_bound,
            upper_bound,
        }.simplify()
    }

    pub fn in_list<'data, V, I>(values: I) -> Filter
    where
        V: Value<'data>,
        I: IntoIterator<Item = V>,
    {
        let physical_type = V::PHYSICAL_TYPE;
        let mut values = values
            .into_iter()
            .filter(|value| value.is_comparable())
            .map(Value::encode)
            .collect::<Vec<_>>();
        values.sort_unstable_by(TypeInfo::new(physical_type).cmp);
        values.dedup();
        Filter::In {
            physical_type,
            values,
        }.simplify()
    }

    #[cfg_attr(rustfmt, rustfmt_skip)]
    fn simplify(self) -> Filter {
        match self {
            // UNBOUNDED
            Filter::Range { lower_bound: None, upper_bound: None, .. } => Filter::IsNotNull,

            // _ <= VALUE < _
            Filter::Range { physical_type, lower_bound: Some(lower), upper_bound: Some(upper) } => {
                let TypeInfo { cmp, are_consecutive, .. } = TypeInfo::new(physical_type);
                if cmp(&lower, &upper) != Ordering::Less {
                    // If the bounds are inverted then no results can be returned.
                    Filter::None
                } else if are_consecutive(&lower, &upper) {
                    // If the bounds are consecutive then the range is equivalent to an equals.
                    Filter::Equals { physical_type, value: lower }
                } else {
                    // No further simplifications.
                    Filter::Range { physical_type, lower_bound: Some(lower), upper_bound: Some(upper) }
                }
            }

            // VALUE >= _
            Filter::Range { physical_type, lower_bound: Some(lower), upper_bound } => {
                let TypeInfo { is_min, is_max, .. } = TypeInfo::new(physical_type);
                if is_min(&lower) {
                    Filter::IsNotNull
                } else if is_max(&lower) {
                    Filter::Equals { physical_type, value: lower }
                } else {
                    Filter::Range { physical_type, lower_bound: Some(lower), upper_bound }
                }
            }

            // VALUE < _
            Filter::Range { physical_type, upper_bound: Some(ref upper), .. } if (TypeInfo::new(physical_type).is_min)(upper) => {
                Filter::None
            }

            Filter::In { physical_type, mut values } => {
                if values.is_empty() {
                    Filter::None
                } else if values.len() == 1 {
                    Filter::Equals { physical_type, value: values.pop().unwrap() }
                } else if physical_type == PhysicalType::Bool {
                    // If this is a boolean IN list with both true and false in the list, then we
                    // can just convert it to IS NOT NULL. This same simplification could be done
                    // for other integer types, but it's probably not as common (and hard to test).
                    Filter::IsNotNull
                } else {
                    Filter::In { physical_type, values }
                }
            }

            other => other,
        }
    }

    #[cfg_attr(rustfmt, rustfmt_skip)]
    pub(crate) fn and(self, other: Filter) -> Filter {
        match (self, other) {
            | (Filter::Range { lower_bound: None, upper_bound: None, .. }, _)
            | (_, Filter::Range { lower_bound: None, upper_bound: None, .. }) => unreachable!(),

            | (Filter::Equals { physical_type: a, .. }, Filter::Equals { physical_type: b, ..})
            | (Filter::Equals { physical_type: a, .. }, Filter::Range { physical_type: b, ..})
            | (Filter::Equals { physical_type: a, .. }, Filter::In { physical_type: b, ..})
            | (Filter::Range { physical_type: a, .. }, Filter::Equals { physical_type: b, ..})
            | (Filter::Range { physical_type: a, .. }, Filter::Range { physical_type: b, ..})
            | (Filter::Range { physical_type: a, .. }, Filter::In { physical_type: b, ..})
            | (Filter::In { physical_type: a, .. }, Filter::Equals { physical_type: b, ..})
            | (Filter::In { physical_type: a, .. }, Filter::Range { physical_type: b, ..})
            | (Filter::In { physical_type: a, .. }, Filter::In { physical_type: b, ..}) if a != b => unreachable!(),

            (Filter::All, other) | (other, Filter::All) => other,
            (Filter::None, _) | (_, Filter::None) => Filter::None,
            (Filter::IsNull, _) | (_, Filter::IsNull) => Filter::None,
            (Filter::IsNotNull, other) | (other, Filter::IsNotNull) => other,
            (Filter::Equals { physical_type, value: a, .. }, Filter::Equals { value: b, .. }) => {
                if a == b {
                    Filter::Equals{ physical_type, value: a }
                } else {
                    Filter::None
                }
            }

            | (Filter::Equals { physical_type, value, .. }, Filter::Range { lower_bound, upper_bound, .. })
            | (Filter::Range { lower_bound, upper_bound, .. }, Filter::Equals{ physical_type, value, .. }) => {
                let cmp = TypeInfo::new(physical_type).cmp;
                match (lower_bound, upper_bound) {
                    (Some(ref lower), _) if cmp(&value, lower) == Ordering::Less => Filter::None,
                    (_, Some(ref upper)) if cmp(&value, upper) != Ordering::Less => Filter::None,
                    _ => Filter::Equals { physical_type, value },
                }
            }

            | (Filter::Equals { physical_type, value, .. }, Filter::In { values, .. })
            | (Filter::In { values, .. }, Filter::Equals { physical_type, value, .. }) => {
                let cmp = TypeInfo::new(physical_type).cmp;
                match values.binary_search_by(|probe| cmp(probe, &value)) {
                    Ok(_) => Filter::Equals { physical_type, value },
                    Err(_) => Filter::None,
                }
            }

            (Filter::Range { physical_type, lower_bound: a_lower, upper_bound: a_upper, .. },
             Filter::Range { lower_bound: b_lower, upper_bound: b_upper, .. }) => {
                let cmp = TypeInfo::new(physical_type).cmp;
                let lower_bound = match (a_lower, b_lower) {
                    (Some(a), Some(b)) => match cmp(&a, &b) {
                        Ordering::Less => Some(b),
                        _ => Some(a),
                    },
                    (None, other) | (other, None) => other,
                };
                let upper_bound = match (a_upper, b_upper) {
                    (Some(a), Some(b)) => match cmp(&a, &b) {
                        Ordering::Greater => Some(b),
                        _ => Some(a),
                    },
                    (None, other) | (other, None) => other,
                };

                Filter::Range { physical_type, lower_bound, upper_bound }.simplify()
            }

            | (Filter::Range { physical_type, lower_bound, upper_bound, .. }, Filter::In { mut values, .. })
            | (Filter::In { mut values, .. }, Filter::Range { physical_type, lower_bound, upper_bound, .. }) => {
                let cmp = TypeInfo::new(physical_type).cmp;
                if let Some(upper) = upper_bound {
                    match values.binary_search_by(|probe| cmp(&upper, probe)) {
                        Ok(idx) => values.truncate(idx),
                        Err(idx) => values.truncate(idx),
                    };
                }
                if let Some(lower) = lower_bound {
                    match values.binary_search_by(|probe| cmp(&lower, probe)) {
                        Ok(idx) => values.drain(..idx),
                        Err(idx) => values.drain(..idx),
                    };
                }

                Filter::In { physical_type, values }.simplify()
            }

            (Filter::In { physical_type, values: a, .. }, Filter::In { values: b, .. }) => {
                let cmp = TypeInfo::new(physical_type).cmp;
                let (mut smaller, larger) = if a.len() < b.len() { (a, b) } else { (b, a) };

                // TODO: this should be using retain
                let mut i = 0;
                let mut j = 0;
                while i != smaller.len() {
                    match larger[j..].binary_search_by(|probe| cmp(&smaller[i], probe)) {
                        Ok(idx) => {
                            i += 1;
                            j = idx;
                        }
                        Err(idx) => {
                            smaller.remove(i);
                            j = idx;
                        }
                    }
                }

                Filter::In { physical_type, values: larger }.simplify()
            }
        }
    }

    pub(crate) fn into_pb(self, column: &Column) -> ColumnPredicatePb {
        use pb::column_predicate_pb::*;
        let predicate = match self {
            Filter::All | Filter::None => unreachable!(),

            Filter::Equals { physical_type, .. }
            | Filter::Range { physical_type, .. }
            | Filter::In { physical_type, .. }
                if physical_type != column.data_type().physical_type() =>
            {
                unreachable!()
            }

            Filter::IsNull => Predicate::IsNull(IsNull {}),
            Filter::IsNotNull => Predicate::IsNotNull(IsNotNull {}),
            Filter::Equals { value, .. } => Predicate::Equality(Equality { value: Some(value) }),
            Filter::Range {
                lower_bound,
                upper_bound,
                ..
            } => Predicate::Range(Range {
                lower: lower_bound,
                upper: upper_bound,
            }),
            Filter::In { values, .. } => Predicate::InList(InList { values }),
        };
        ColumnPredicatePb {
            column: Some(column.name().to_owned()),
            predicate: Some(predicate),
        }
    }

    /// Checks that the filter is compatible with the provided column.
    pub(crate) fn check_type(&self, column: &Column) -> Result<()> {
        match *self {
            Filter::Equals { physical_type, .. }
            | Filter::Range { physical_type, .. }
            | Filter::In { physical_type, .. }
                if physical_type != column.data_type().physical_type() =>
            {
                Err(Error::InvalidArgument(format!(
                    "filter of type {:?} is invalid for column {:?}",
                    physical_type, column
                )))
            }
            _ => Ok(()),
        }
    }
}

struct TypeInfo {
    cmp: fn(&Vec<u8>, &Vec<u8>) -> Ordering,
    are_consecutive: fn(&Vec<u8>, &Vec<u8>) -> bool,
    is_min: fn(&Vec<u8>) -> bool,
    is_max: fn(&Vec<u8>) -> bool,
}

impl TypeInfo {
    fn new(physical_type: PhysicalType) -> TypeInfo {
        fn bool_cmp(a: &Vec<u8>, b: &Vec<u8>) -> Ordering {
            unsafe { bool::decode(a).cmp(&bool::decode(b)) }
        }
        fn int8_cmp(a: &Vec<u8>, b: &Vec<u8>) -> Ordering {
            unsafe { i8::decode(a).cmp(&i8::decode(b)) }
        }
        fn int16_cmp(a: &Vec<u8>, b: &Vec<u8>) -> Ordering {
            unsafe { i16::decode(a).cmp(&i16::decode(b)) }
        }
        fn int32_cmp(a: &Vec<u8>, b: &Vec<u8>) -> Ordering {
            unsafe { i32::decode(a).cmp(&i32::decode(b)) }
        }
        fn int64_cmp(a: &Vec<u8>, b: &Vec<u8>) -> Ordering {
            unsafe { i64::decode(a).cmp(&i64::decode(b)) }
        }
        fn float_cmp(a: &Vec<u8>, b: &Vec<u8>) -> Ordering {
            unsafe { f32::decode(a).partial_cmp(&f32::decode(b)).unwrap() }
        }
        fn double_cmp(a: &Vec<u8>, b: &Vec<u8>) -> Ordering {
            unsafe { f64::decode(a).partial_cmp(&f64::decode(b)).unwrap() }
        }
        fn binary_cmp(a: &Vec<u8>, b: &Vec<u8>) -> Ordering {
            a.cmp(b)
        }

        fn bool_are_consecutive(a: &Vec<u8>, b: &Vec<u8>) -> bool {
            unsafe { bool::decode(a) as u8 + 1 == bool::decode(b) as u8 }
        }
        fn int8_are_consecutive(a: &Vec<u8>, b: &Vec<u8>) -> bool {
            unsafe {
                let a = i8::decode(a);
                let b = i8::decode(b);
                a != i8::MAX && a + 1 == b
            }
        }
        fn int16_are_consecutive(a: &Vec<u8>, b: &Vec<u8>) -> bool {
            unsafe {
                let a = i16::decode(a);
                let b = i16::decode(b);
                a != i16::MAX && a + 1 == b
            }
        }
        fn int32_are_consecutive(a: &Vec<u8>, b: &Vec<u8>) -> bool {
            unsafe {
                let a = i32::decode(a);
                let b = i32::decode(b);
                a != i32::MAX && a + 1 == b
            }
        }
        fn int64_are_consecutive(a: &Vec<u8>, b: &Vec<u8>) -> bool {
            unsafe {
                let a = i64::decode(a);
                let b = i64::decode(b);
                a != i64::MAX && a + 1 == b
            }
        }
        #[cfg_attr(feature = "cargo-clippy", allow(float_cmp))]
        fn float_are_consecutive(a: &Vec<u8>, b: &Vec<u8>) -> bool {
            unsafe {
                let a = f32::decode(a);
                let b = f32::decode(b);
                a != f32::INFINITY && a.next() == b
            }
        }
        #[cfg_attr(feature = "cargo-clippy", allow(float_cmp))]
        fn double_are_consecutive(a: &Vec<u8>, b: &Vec<u8>) -> bool {
            unsafe {
                let a = f64::decode(a);
                let b = f64::decode(b);
                a != f64::INFINITY && a.next() == b
            }
        }
        fn binary_are_consecutive(a: &Vec<u8>, b: &Vec<u8>) -> bool {
            unsafe {
                let a = <&[u8]>::decode(a);
                let b = <&[u8]>::decode(b);
                let a_len = a.len();
                let b_len = b.len();
                a_len + 1 == b_len && b[a_len] == 0 && a == &b[..a_len]
            }
        }

        fn bool_is_min(data: &Vec<u8>) -> bool {
            unsafe { !bool::decode(data) }
        }
        fn int8_is_min(data: &Vec<u8>) -> bool {
            unsafe { i8::decode(data) == i8::MIN }
        }
        fn int16_is_min(data: &Vec<u8>) -> bool {
            unsafe { i16::decode(data) == i16::MIN }
        }
        fn int32_is_min(data: &Vec<u8>) -> bool {
            unsafe { i32::decode(data) == i32::MIN }
        }
        fn int64_is_min(data: &Vec<u8>) -> bool {
            unsafe { i64::decode(data) == i64::MIN }
        }
        fn float_is_min(data: &Vec<u8>) -> bool {
            unsafe { f32::decode(data) == f32::NEG_INFINITY }
        }
        fn double_is_min(data: &Vec<u8>) -> bool {
            unsafe { f64::decode(data) == f64::NEG_INFINITY }
        }
        fn binary_is_min(data: &Vec<u8>) -> bool {
            data.is_empty()
        }

        fn bool_is_max(data: &Vec<u8>) -> bool {
            unsafe { bool::decode(data) }
        }
        fn int8_is_max(data: &Vec<u8>) -> bool {
            unsafe { i8::decode(data) == i8::MAX }
        }
        fn int16_is_max(data: &Vec<u8>) -> bool {
            unsafe { i16::decode(data) == i16::MAX }
        }
        fn int32_is_max(data: &Vec<u8>) -> bool {
            unsafe { i32::decode(data) == i32::MAX }
        }
        fn int64_is_max(data: &Vec<u8>) -> bool {
            unsafe { i64::decode(data) == i64::MAX }
        }
        fn float_is_max(data: &Vec<u8>) -> bool {
            unsafe { f32::decode(data) == f32::INFINITY }
        }
        fn double_is_max(data: &Vec<u8>) -> bool {
            unsafe { f64::decode(data) == f64::INFINITY }
        }
        fn binary_is_max(_data: &Vec<u8>) -> bool {
            false
        }

        match physical_type {
            PhysicalType::Bool => TypeInfo {
                cmp: bool_cmp,
                are_consecutive: bool_are_consecutive,
                is_min: bool_is_min,
                is_max: bool_is_max,
            },
            PhysicalType::Int8 => TypeInfo {
                cmp: int8_cmp,
                are_consecutive: int8_are_consecutive,
                is_min: int8_is_min,
                is_max: int8_is_max,
            },
            PhysicalType::Int16 => TypeInfo {
                cmp: int16_cmp,
                are_consecutive: int16_are_consecutive,
                is_min: int16_is_min,
                is_max: int16_is_max,
            },
            PhysicalType::Int32 => TypeInfo {
                cmp: int32_cmp,
                are_consecutive: int32_are_consecutive,
                is_min: int32_is_min,
                is_max: int32_is_max,
            },
            PhysicalType::Int64 => TypeInfo {
                cmp: int64_cmp,
                are_consecutive: int64_are_consecutive,
                is_min: int64_is_min,
                is_max: int64_is_max,
            },
            PhysicalType::Float => TypeInfo {
                cmp: float_cmp,
                are_consecutive: float_are_consecutive,
                is_min: float_is_min,
                is_max: float_is_max,
            },
            PhysicalType::Double => TypeInfo {
                cmp: double_cmp,
                are_consecutive: double_are_consecutive,
                is_min: double_is_min,
                is_max: double_is_max,
            },
            PhysicalType::Binary => TypeInfo {
                cmp: binary_cmp,
                are_consecutive: binary_are_consecutive,
                is_min: binary_is_min,
                is_max: binary_is_max,
            },
        }
    }
}
