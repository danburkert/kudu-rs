use std::cmp::Ordering;
use std::collections::HashSet;
use std::fmt;
use std::mem;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::sync::Arc;
use std::time::Duration;
use std::time::{UNIX_EPOCH, Instant, SystemTime};

use chrono;
use crossbeam::sync::SegQueue;
use futures::task::{EventSet, UnparkEvent, with_unpark_event};
use futures::{Async, AsyncSink, Future, IntoFuture, Poll, Sink, Stream};
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
#[must_use = "streams do nothing unless polled"]
pub struct SelectStream<F> where F: Future {
    slab: Slab<F>,
    event_set: Arc<SegQueueEventSet>,
}

impl <F> SelectStream<F> where F: Future {
    pub fn add(&mut self, f: F) {
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
            if let Some(mut entry) = slab.entry(idx) {
                let event = UnparkEvent::new(event_set.clone(), idx);
                let poll = with_unpark_event(event, || entry.get_mut().poll());
                let result = match poll {
                    Ok(Async::NotReady) => continue,
                    Ok(Async::Ready(item)) => Ok(Async::Ready(Some(item))),
                    Err(error) => Err(error),
                };
                entry.remove();
                return result;
            }
        }
        Ok(Async::NotReady)
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
#[must_use = "streams do nothing unless polled"]
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

pub fn retry_with_backoff<R, F>(timer: tokio_timer::Timer,
                                mut backoff: Backoff,
                                mut retry: R)
                                -> RetryWithBackoff<R, F>
where R: FnMut(Instant, RetryCause<F::Error>) -> F,
      F: Future,
{
    let duration = backoff.next_backoff();
    let future = retry(Instant::now() + duration, RetryCause::Initial);
    let sleep = timer.sleep(duration);
    RetryWithBackoff {
        backoff: backoff,
        timer: timer,
        sleep: sleep,
        retry: retry,
        try: Try::Future(future),
    }
}

pub enum RetryCause<E> {
    Initial,
    TimedOut,
    Err(E),
}

enum Try<F> where F: Future {
    Future(F),
    Err(F::Error),
    None,
}
impl <F> Try<F> where F: Future {
    fn take(&mut self) -> Result<F, F::Error> {
        match mem::replace(self, Try::None) {
            Try::Future(f) => Ok(f),
            Try::Err(error) => Err(error),
            Try::None => unreachable!(),
        }
    }
}

#[must_use = "futures do nothing unless polled"]
pub struct RetryWithBackoff<R, F>
where R: FnMut(Instant, RetryCause<F::Error>) -> F,
      F: Future,
{
    backoff: Backoff,
    timer: tokio_timer::Timer,
    sleep: tokio_timer::Sleep,
    retry: R,
    try: Try<F>,
}

impl <R, F> Future for RetryWithBackoff<R, F>
where R: FnMut(Instant, RetryCause<F::Error>) -> F,
      F: Future,
{
    type Item = F::Item;
    type Error = !;

    fn poll(&mut self) -> Poll<F::Item, !> {
        loop {
            {
                let poll = if let Try::Future(ref mut f) = self.try {
                    f.poll()
                } else {
                    Ok(Async::NotReady)
                };
                match poll {
                    Ok(Async::Ready(item)) => return Ok(Async::Ready(item)),
                    Ok(Async::NotReady) => (),
                    Err(error) => self.try = Try::Err(error),
                }
            }

            // Unwrap here is unfortunate, but we really have no way to handle
            // the timer being out of capacity.
            match self.sleep.poll().unwrap() {
                Async::Ready(()) => {
                    let duration = self.backoff.next_backoff();

                    let cause = match self.try.take() {
                        Ok(_) => RetryCause::TimedOut,
                        Err(error) => RetryCause::Err(error),
                    };

                    self.try = Try::Future((self.retry)(Instant::now() + duration, cause));
                    self.sleep = self.timer.sleep(duration);
                },
                Async::NotReady => return Ok(Async::NotReady),
            }
        }
    }
}

/// The status of a `tail_fn` loop.
pub enum Tail<T, S> {
    /// Indicates that the loop has completed with output `T`.
    Done(T),

    /// Indicates that the loop function should be called again with input
    /// state `S`.
    Loop(S),
}

/// TODO: remove once `futures::TailFn` is released.
#[must_use = "futures do nothing unless polled"]
pub struct TailFn<A, F> {
    future: A,
    func: F,
}

/// TODO: use `futures::tail_fn` once it's released.
pub fn tail_fn<S, T, I, A, F>(initial_state: S, mut func: F) -> TailFn<A, F>
    where F: FnMut(S) -> I,
          A: Future<Item = Tail<T, S>>,
          I: IntoFuture<Future = A, Item = A::Item, Error = A::Error>
{
    TailFn {
        future: func(initial_state).into_future(),
        func: func,
    }
}

impl<S, T, I, A, F> Future for TailFn<A, F>
    where F: FnMut(S) -> I,
          A: Future<Item = Tail<T, S>>,
          I: IntoFuture<Future = A, Item = A::Item, Error = A::Error>
{
    type Item = T;
    type Error = A::Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        loop {
            match try_ready!(self.future.poll()) {
                Tail::Done(x) => return Ok(Async::Ready(x)),
                Tail::Loop(s) => self.future = (self.func)(s).into_future(),
            }
        }
    }
}

/// TODO: replace with `futures::PollFn`.
#[must_use = "futures do nothing unless polled"]
pub struct PollFn<F> {
    inner: F,
}

/// TODO: replace with `futures::poll_fn`.
pub fn poll_fn<T, E, F>(f: F) -> PollFn<F> where F: FnMut() -> Poll<T, E> {
    PollFn { inner: f }
}

impl<T, E, F> Future for PollFn<F> where F: FnMut() -> Poll<T, E> {
    type Item = T;
    type Error = E;

    fn poll(&mut self) -> Poll<T, E> {
        (self.inner)()
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
