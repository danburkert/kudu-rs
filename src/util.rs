use std::collections::HashSet;
use std::fmt;
use std::net::IpAddr;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use chrono;
use futures::{Async, Future, Poll};
use ifaces;
use url::Url;

use pb::HostPortPb;
use DataType;
use Error;
use Row;

pub fn time_to_us(time: SystemTime) -> i64 {
    // TODO: do overflow checking
    match time.duration_since(UNIX_EPOCH) {
        Ok(duration) => {
            (duration.as_secs() * 1_000_000 + u64::from(duration.subsec_nanos()) / 1000) as i64
        }
        Err(error) => {
            let duration = error.duration();
            (-((duration.as_secs() * 1_000_000 + u64::from(duration.subsec_nanos()) / 1000) as i64))
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

pub fn fmt_hex<T>(f: &mut fmt::Formatter, bytes: &[T]) -> fmt::Result
where
    T: fmt::LowerHex,
{
    write!(f, "0x")?;
    for b in bytes {
        write!(f, "{:02x}", b)?;
    }
    Ok(())
}

fn fmt_timestamp(f: &mut fmt::Formatter, timestamp: SystemTime) -> fmt::Result {
    let datetime = if timestamp < UNIX_EPOCH {
        chrono::NaiveDateTime::from_timestamp(0, 0)
            - chrono::Duration::from_std(UNIX_EPOCH.duration_since(timestamp).unwrap()).unwrap()
    } else {
        chrono::NaiveDateTime::from_timestamp(0, 0)
            + chrono::Duration::from_std(timestamp.duration_since(UNIX_EPOCH).unwrap()).unwrap()
    };

    write!(f, "{}", datetime.format("%Y-%m-%dT%H:%M:%S%.6fZ"))
}

pub fn fmt_cell(f: &mut fmt::Formatter, row: &Row, idx: usize) -> fmt::Result {
    debug_assert!(row.is_set(idx).unwrap());
    if row.is_null(idx).unwrap() {
        return write!(f, "NULL");
    }

    match row.schema().columns()[idx].data_type() {
        DataType::Bool => write!(f, "{}", row.get::<_, bool>(idx).unwrap()),
        DataType::Int8 => write!(f, "{}", row.get::<_, i8>(idx).unwrap()),
        DataType::Int16 => write!(f, "{}", row.get::<_, i16>(idx).unwrap()),
        DataType::Int32 => write!(f, "{}", row.get::<_, i32>(idx).unwrap()),
        DataType::Int64 => write!(f, "{}", row.get::<_, i64>(idx).unwrap()),
        DataType::Timestamp => fmt_timestamp(f, row.get::<_, SystemTime>(idx).unwrap()),
        DataType::Float => write!(f, "{}", row.get::<_, f32>(idx).unwrap()),
        DataType::Double => write!(f, "{}", row.get::<_, f64>(idx).unwrap()),
        DataType::Binary => fmt_hex(f, row.get::<_, &[u8]>(idx).unwrap()),
        DataType::String => write!(f, "{:?}", row.get::<_, &str>(idx).unwrap()),
    }
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
            }
            Err(error) => warn!("failed to resolve local interface addresses: {}", error),
        }
        addrs
    };
}

/// Returns `true` if socket addr is for a local interface.
#[allow(dead_code)]
pub fn is_local_addr(addr: &IpAddr) -> bool {
    LOCAL_ADDRS.contains(addr) || addr.is_loopback()
}

pub(crate) fn urls_from_pb(
    hostports: &[HostPortPb],
    https_enabled: bool,
) -> Result<Vec<Url>, Error> {
    hostports
        .iter()
        .map(|hostport| {
            Url::parse(&format!(
                "{}://{}:{}",
                if https_enabled { "https" } else { "http" },
                hostport.host,
                hostport.port
            )).map_err(From::from)
        })
        .collect()
}

pub struct ContextFuture<F, C> {
    future: F,
    context: Option<C>,
}

impl<F, C> ContextFuture<F, C> {
    pub fn new(future: F, context: C) -> ContextFuture<F, C> {
        ContextFuture {
            future,
            context: Some(context),
        }
    }
}

impl<F, C> Future for ContextFuture<F, C>
where
    F: Future,
{
    type Item = (F::Item, C);
    type Error = (F::Error, C);

    fn poll(&mut self) -> Poll<(F::Item, C), (F::Error, C)> {
        match self.future.poll() {
            Ok(Async::Ready(item)) => Ok(Async::Ready((
                item,
                self.context.take().expect("future already complete"),
            ))),
            Ok(Async::NotReady) => Ok(Async::NotReady),
            Err(error) => Err((error, self.context.take().expect("future already complete"))),
        }
    }
}

#[cfg(test)]
mod tests {

    use std::net::ToSocketAddrs;
    use std::time::{Duration, UNIX_EPOCH};

    use env_logger;
    use quickcheck::{quickcheck, TestResult};

    use super::*;
    use schema;

    #[test]
    fn timestamp_conversion() {
        let _ = env_logger::try_init();

        fn roundtrip(us: i64) -> TestResult {
            TestResult::from_bool(us == time_to_us(us_to_time(us)))
        }

        quickcheck(roundtrip as fn(i64) -> TestResult);
    }

    #[test]
    fn test_format_timestamp() {
        let _ = env_logger::try_init();
        let schema = schema::tests::all_types_schema();
        let mut row = schema.new_row();

        row.set("timestamp", UNIX_EPOCH - Duration::from_millis(1234))
            .unwrap();
        assert_eq!(
            "{timestamp: 1969-12-31T23:59:58.766000Z}",
            &format!("{:?}", row)
        );

        row.set("timestamp", UNIX_EPOCH + Duration::from_millis(1234))
            .unwrap();
        assert_eq!(
            "{timestamp: 1970-01-01T00:00:01.234000Z}",
            &format!("{:?}", row)
        );
    }

    #[test]
    fn test_is_local_addr() {
        let _ = env_logger::try_init();
        let addr = "127.0.1.1:0"
            .to_socket_addrs()
            .unwrap()
            .next()
            .unwrap()
            .ip();
        assert!(is_local_addr(&addr));
        let addr = "127.0.0.1:0"
            .to_socket_addrs()
            .unwrap()
            .next()
            .unwrap()
            .ip();
        assert!(is_local_addr(&addr));
    }
}
