use std::cmp::Ordering;
use std::collections::HashSet;
use std::fmt;
use std::mem;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::time::Duration;
use std::time::{UNIX_EPOCH, Instant, SystemTime};

use chrono;
use futures::{Async, Future, Poll, Stream};
use ifaces;
use timer;

use DataType;
use Row;
use backoff::Backoff;

pub fn duration_to_ms(duration: &Duration) -> u64 {
    duration.as_secs() * 1000 + u64::from(duration.subsec_nanos()) / 1_000_000
}

pub fn fmt_hex<T>(f: &mut fmt::Formatter, bytes: &[T]) -> fmt::Result where T: fmt::LowerHex {
    if bytes.is_empty() {
        return write!(f, "0x")
    }
    try!(write!(f, "{:#x}", bytes[0]));
    for b in &bytes[1..] {
        try!(write!(f, "{:x}", b));
    }
    Ok(())
}

pub fn time_to_us(time: &SystemTime) -> i64 {
    // TODO: do overflow checking
    match time.duration_since(UNIX_EPOCH) {
        Ok(duration) => {
            (duration.as_secs() * 1_000_000 + u64::from(duration.subsec_nanos()) / 1000) as i64
        },
        Err(error) => {
            let duration = error.duration();
            (- ((duration.as_secs() * 1_000_000 + u64::from(duration.subsec_nanos()) / 1000) as i64))
        }
    }
}

pub fn us_to_time(us: i64) -> SystemTime {
    let abs = us.abs() as u64;

    let s = abs / 1_000_000;
    let ns = (abs % 1_000_000) as u32 * 1000;

    if us.is_negative() {
        UNIX_EPOCH - Duration::new(s, ns)
    } else {
        UNIX_EPOCH + Duration::new(s, ns)
    }
}

pub fn fmt_timestamp(f: &mut fmt::Formatter, timestamp: SystemTime) -> fmt::Result {
    let datetime = if timestamp < UNIX_EPOCH {
        chrono::NaiveDateTime::from_timestamp(0, 0) -
            chrono::Duration::from_std(UNIX_EPOCH.duration_since(timestamp).unwrap()).unwrap()
    } else {
        chrono::NaiveDateTime::from_timestamp(0, 0) +
            chrono::Duration::from_std(timestamp.duration_since(UNIX_EPOCH).unwrap()).unwrap()
    };

    write!(f, "{}", datetime.format("%Y-%m-%dT%H:%M:%S%.6fZ"))
}

pub fn fmt_cell(f: &mut fmt::Formatter, row: &Row, idx: usize) -> fmt::Result {
    match row.schema().columns()[idx].data_type() {
        DataType::Bool => write!(f, "{}", row.get::<bool>(idx).unwrap()),
        DataType::Int8 => write!(f, "{}", row.get::<i8>(idx).unwrap()),
        DataType::Int16 => write!(f, "{}", row.get::<i16>(idx).unwrap()),
        DataType::Int32 => write!(f, "{}", row.get::<i32>(idx).unwrap()),
        DataType::Int64 => write!(f, "{}", row.get::<i64>(idx).unwrap()),
        DataType::Timestamp => fmt_timestamp(f, row.get::<SystemTime>(idx).unwrap()),
        DataType::Float => write!(f, "{}", row.get::<f32>(idx).unwrap()),
        DataType::Double => write!(f, "{}", row.get::<f64>(idx).unwrap()),
        DataType::Binary => fmt_hex(f, row.get::<&[u8]>(idx).unwrap()),
        DataType::String => write!(f, "{:?}", row.get::<&str>(idx).unwrap()),
    }
}

pub fn dummy_addr() -> SocketAddr {
    SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 0)
}

lazy_static! {
    static ref LOCAL_ADDRS: HashSet<IpAddr> = {
        let mut addrs = HashSet::new();
        match ifaces::Interface::get_all() {
            Ok(ifaces) => {
                for iface in ifaces {
                    if let Some(addr) = iface.addr {
                        addrs.insert(addr.ip());
                    }
                }
            },
            Err(error) => warn!("failed to resolve local interface addresses: {}", error),
        }
        addrs
    };
}

/// Returns `true` if socket addr is for a local interface.
pub fn is_local_addr(addr: &IpAddr) -> bool {
    LOCAL_ADDRS.contains(addr) || addr.is_loopback()
}

pub fn cmp_socket_addrs(a: &SocketAddr, b: &SocketAddr) -> Ordering {
    match (a, b) {
        (&SocketAddr::V4(ref a), &SocketAddr::V4(ref b)) => (a.ip(), a.port()).cmp(&(b.ip(), b.port())),
        (&SocketAddr::V6(ref a), &SocketAddr::V6(ref b)) => (a.ip(), a.port()).cmp(&(b.ip(), b.port())),
        (&SocketAddr::V4(_), &SocketAddr::V6(_)) => Ordering::Less,
        (&SocketAddr::V6(_), &SocketAddr::V4(_)) => Ordering::Greater,
    }
}

#[cfg(test)]
mod tests {

    use std::time::{Duration, UNIX_EPOCH};
    use std::net::ToSocketAddrs;

    use env_logger;
    use quickcheck::{quickcheck, TestResult};

    use schema;
    use super::*;

    #[test]
    fn timestamp_conversion() {
        let _ = env_logger::init();

        fn roundtrip(us: i64) -> TestResult {
            TestResult::from_bool(us == time_to_us(&us_to_time(us)))
        }

        quickcheck(roundtrip as fn(i64) -> TestResult);
    }

    #[test]
    fn test_format_timestamp() {
        let _ = env_logger::init();
        let schema = schema::tests::all_types_schema();
        let mut row = schema.new_row();

        row.set_by_name("timestamp", UNIX_EPOCH - Duration::from_millis(1234)).unwrap();
        assert_eq!("Timestamp \"timestamp\"=1969-12-31T23:59:58.766000Z",
                   &format!("{:?}", row));

        row.set_by_name("timestamp", UNIX_EPOCH + Duration::from_millis(1234)).unwrap();
        assert_eq!("Timestamp \"timestamp\"=1970-01-01T00:00:01.234000Z",
                   &format!("{:?}", row));
    }

    #[test]
    fn test_is_local_addr() {
        let _ = env_logger::init();
        let addr = "127.0.1.1:0".to_socket_addrs().unwrap().next().unwrap().ip();
        assert!(is_local_addr(&addr));
        let addr = "127.0.0.1:0".to_socket_addrs().unwrap().next().unwrap().ip();
        assert!(is_local_addr(&addr));
    }
}
