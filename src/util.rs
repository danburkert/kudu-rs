use std::cmp::Ordering;
use std::collections::HashSet;
use std::fmt;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::sync::Arc;
use std::time::Duration;
use std::time::{UNIX_EPOCH, SystemTime};

use chrono;
use crossbeam::sync::SegQueue;
use futures::task::{EventSet, UnparkEvent, with_unpark_event};
use futures::{Async, Future, IntoFuture, Poll, Stream};
use ifaces;
use slab::Slab;
use tokio_timer;

use DataType;
use Row;
use backoff::Backoff;

pub fn duration_to_ms(duration: &Duration) -> u64 {
    duration.as_secs() * 1000 + duration.subsec_nanos() as u64 / 1000_000
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
            (duration.as_secs() * 1000_000 + duration.subsec_nanos() as u64 / 1000) as i64
        },
        Err(error) => {
            let duration = error.duration();
            (- ((duration.as_secs() * 1000_000 + duration.subsec_nanos() as u64 / 1000) as i64))
        }
    }

}

pub fn us_to_time(us: i64) -> SystemTime {
    let abs = us.abs() as u64;

    let s = abs / 1000_000;
    let ns = (abs % 1000_000) as u32 * 1000;

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

/// Creates a new stream from a collection of futures, yielding items in order
/// of completion.
// TODO: wat is this type signature?
pub fn select_stream<I>(futures: I) -> SelectStream<<<I as IntoIterator>::Item as IntoFuture>::Future>
where I: IntoIterator,
      I::Item: IntoFuture,
      I::IntoIter: ExactSizeIterator {
    let iter = futures.into_iter();
    let mut slab = Slab::with_capacity(iter.len());
    let event_set = SegQueueEventSet::new();
    for f in iter {
        let idx = match slab.insert(f.into_future()) {
            Ok(idx) => idx,
            _ => unreachable!(),
        };
        event_set.insert(idx);
    }
    SelectStream {
        slab: slab,
        event_set: Arc::new(event_set),
    }
}

/// Stream which yields items from a collection of futures in completion order.
#[derive(Debug)]
pub struct SelectStream<F> where F: Future {
    slab: Slab<F>,
    event_set: Arc<SegQueueEventSet>,
}

impl <F> SelectStream<F> where F: Future {
    fn add(&mut self, f: F) {
        if !self.slab.has_available() {
            let len = self.slab.len();
            self.slab.reserve_exact(if len < 4 { 8 - len } else { len / 2 });
        }
        let idx = match self.slab.insert(f) {
            Ok(idx) => idx,
            _ => unreachable!(),
        };
        self.event_set.insert(idx);
    }
}

impl<F> Stream for SelectStream<F> where F: Future {
    type Item = F::Item;
    type Error = F::Error;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        if self.slab.is_empty() {
            assert!(self.event_set.0.try_pop().is_none());
            return Ok(Async::Ready(None));
        }
        let SelectStream { ref mut slab, ref event_set } = *self;
        while let Some(idx) = event_set.0.try_pop() {
            let event = UnparkEvent::new(event_set.clone(), idx);
            let mut entry = slab.entry(idx).unwrap();
            let poll = with_unpark_event(event, || entry.get_mut().poll());
            let result = match poll {
                Ok(Async::NotReady) => continue,
                Ok(Async::Ready(item)) => Ok(Async::Ready(Some(item))),
                Err(error) => Err(error),
            };
            entry.remove();
            return result;
        }
        Ok(Async::NotReady)
    }
}

#[derive(Debug)]
pub struct MergeStream<S> where S: Stream {
    slab: Slab<S>,
    event_set: Arc<SegQueueEventSet>,
}

impl <S> MergeStream<S> where S: Stream {
    fn add(&mut self, stream: S) {
        if !self.slab.has_available() {
            let len = self.slab.len();
            self.slab.reserve_exact(if len < 4 { 8 - len } else { len / 2 });
        }
        let idx = match self.slab.insert(stream) {
            Ok(idx) => idx,
            _ => unreachable!(),
        };
        self.event_set.insert(idx);
    }
}

impl<S> Stream for MergeStream<S> where S: Stream {
    type Item = S::Item;
    type Error = S::Error;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        if self.slab.is_empty() {
            debug_assert!(self.event_set.0.try_pop().is_none());
            return Ok(Async::Ready(None));
        }
        let MergeStream { ref mut slab, ref event_set } = *self;
        while let Some(idx) = event_set.0.try_pop() {
            let event = UnparkEvent::new(event_set.clone(), idx);
            let mut entry = slab.entry(idx).unwrap();
            let poll = with_unpark_event(event, || entry.get_mut().poll());
            match poll {
                Ok(Async::NotReady) => continue,
                Ok(Async::Ready(Some(item))) => return Ok(Async::Ready(Some(item))),
                Ok(Async::Ready(None)) => { entry.remove(); },
                Err(error) => return Err(error),
            }
        }
        if slab.is_empty() {
            debug_assert!(self.event_set.0.try_pop().is_none());
            Ok(Async::Ready(None))
        } else {
            Ok(Async::NotReady)
        }
    }
}

/// Returns a stream which yields elements according to the backoff policy.
pub fn backoff_stream(mut backoff: Backoff, timer: tokio_timer::Timer) -> BackoffStream {
    let sleep = timer.sleep(backoff.next_backoff());
    BackoffStream {
        backoff: backoff,
        timer: timer,
        sleep: sleep,
    }
}
/// Stream which yields elements according to a backoff policy.
pub struct BackoffStream {
    backoff: Backoff,
    timer: tokio_timer::Timer,
    sleep: tokio_timer::Sleep
}
impl Stream for BackoffStream {
    type Item = ();
    type Error = ();
    fn poll(&mut self) -> Poll<Option<()>, ()> {
        let _ = try_ready!(self.sleep.poll());
        let backoff = self.backoff.next_backoff();
        self.sleep = self.timer.sleep(backoff);
        Ok(Async::Ready(Some(())))
    }
}

pub fn retry_with_backoff<F, R>(retry: R,
                                mut backoff: Backoff,
                                timer: tokio_timer::Timer)
                                -> RetryWithBackoff
where R: FnMut() -> F,
      F: Future
{
    let sleep = timer.sleep(backoff.next_backoff());
    let f = retry();
    RetryWithBackoff {
        backoff: Backoff,
        timer: timer,
        sleep: sleep,
        f: Some(f),
    }
}

struct RetryWithBackoff<R, F>
where R: FnMut() -> F,
      F: Future
{
    backoff: Backoff,
    timer: tokio_timer::Timer,
    sleep: tokio_timer::Sleep,
    f: Option<F>,
}

impl <R, F> Stream for RetryWithBackoff<R, F>
where R: FnMut() -> F,
      F: Future
{
    type Item = F::Item;
    type Error = F::Error;

    fn poll(&mut self) -> Poll<F::Item, F::Error> {
        unimplemented!()
    }
}



#[derive(Debug)]
struct SegQueueEventSet(SegQueue<usize>);
impl SegQueueEventSet {
    fn new() -> SegQueueEventSet {
        SegQueueEventSet(SegQueue::new())
    }
}
impl EventSet for SegQueueEventSet {
    fn insert(&self, id: usize) {
        self.0.push(id);
    }
}

pub fn retry_with_backoff<F, G, E>(mut g: G,
                                   e: E,
                                   backoff: Backoff,
                                   timer: tokio_timer::Timer)
                                   -> RetryWithBackoff<F, G, E>
where F: Future,
      G: FnMut() -> F,
      E: FnMut(F::Error) {
    let backoff = backoff_stream(backoff, timer);
    let f = g();
    RetryWithBackoff {
        g: g,
        f: f,
        e: e,
        backoff: backoff,
    }
}

pub struct RetryWithBackoff<F, G, E>
where F: Future,
      G: FnMut() -> F,
      E: FnMut(F::Error),
{
    g: G,
    f: F,
    e: E,
    backoff: BackoffStream,
}

impl <F, G, E> Future for RetryWithBackoff<F, G, E>
where F: Future,
      G: FnMut() -> F,
      E: FnMut(F::Error),
{
    type Item = F::Item;
    type Error = !;
    fn poll(&mut self) -> Poll<F::Item, !> {
        loop {
            match self.f.poll() {
                Ok(Async::Ready(value)) => return Ok(Async::Ready(value)),
                Ok(Async::NotReady) => (),
                Err(error) => (self.e)(error),
            }
            match self.backoff.poll() {
                Ok(Async::Ready(Some(()))) => self.f = (self.g)(),
                Ok(Async::Ready(None)) => panic!("backoff timer ended"),
                Ok(Async::NotReady) => return Ok(Async::NotReady),
                Err(()) => panic!("backoff timer failed"),
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use std::time::{Duration, UNIX_EPOCH};
    use std::net::ToSocketAddrs;

    use env_logger;
    use futures::sync::oneshot;
    use futures;
    use quickcheck::{quickcheck, TestResult};

    use schema;
    use super::*;

    #[test]
    fn timestamp_conversion() {

        fn roundtrip(us: i64) -> TestResult {
            TestResult::from_bool(us == time_to_us(&us_to_time(us)))
        }

        quickcheck(roundtrip as fn(i64) -> TestResult);
    }

    #[test]
    fn test_format_timestamp() {
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
    fn test_select_stream() {
        let _ = env_logger::init();
        let (a_tx, a_rx) = oneshot::channel::<u32>();
        let (b_tx, b_rx) = oneshot::channel::<u32>();
        let (c_tx, c_rx) = oneshot::channel::<u32>();

        let stream = select_stream(vec![a_rx, b_rx, c_rx]);

        let mut spawn = futures::executor::spawn(stream);
        b_tx.complete(99);
        assert_eq!(Some(Ok(99)), spawn.wait_stream());

        a_tx.complete(33);
        c_tx.complete(33);
        assert_eq!(Some(Ok(33)), spawn.wait_stream());
        assert_eq!(Some(Ok(33)), spawn.wait_stream());
        assert_eq!(None, spawn.wait_stream());
    }

    #[test]
    fn test_is_local_addr() {
        let addr = "127.0.1.1:0".to_socket_addrs().unwrap().next().unwrap().ip();
        assert!(is_local_addr(&addr));
        let addr = "127.0.0.1:0".to_socket_addrs().unwrap().next().unwrap().ip();
        assert!(is_local_addr(&addr));
    }
}
