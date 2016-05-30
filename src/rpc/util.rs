use eventual::{AsyncResult, Future};

pub fn collect<T, E>(futures: &[Future<T, E>]) -> Future<Vec<AsyncResult<T, E>>, ()>
where T: Send, E: Send {
    unimplemented!()
}
