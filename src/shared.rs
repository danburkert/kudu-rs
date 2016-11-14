use std::cell::RefCell;
use std::rc::Rc;

use futures::{Future, Poll, Sink, StartSend};

pub struct Shared<T> {
    inner: Rc<RefCell<T>>,
}

impl <T> Shared<T>  {
    #[inline]
    pub fn new(value: T) -> Shared<T> {
        Shared {
            inner: Rc::new(RefCell::new(value)),
        }
    }
}

impl <T> AsRef<RefCell<T>> for Shared<T> {
    #[inline]
    fn as_ref(&self) -> &RefCell<T> {
        self.inner.as_ref()
    }
}


impl <T> Clone for Shared<T> {
    #[inline]
    fn clone(&self) -> Shared<T> {
        Shared {
            inner: self.inner.clone(),
        }
    }
}

impl <F> Future for Shared<F> where F: Future {
    type Item = F::Item;
    type Error = F::Error;

    #[inline]
    fn poll(&mut self) -> Poll<F::Item, F::Error> {
        self.inner.borrow_mut().poll()
    }
}

impl <S> Sink for Shared<S> where S: Sink {
    type SinkItem = S::SinkItem;
    type SinkError = S::SinkError;

    #[inline]
    fn start_send(&mut self, item: S::SinkItem) -> StartSend<S::SinkItem, S::SinkError> {
        self.inner.borrow_mut().start_send(item)
    }

    #[inline]
    fn poll_complete(&mut self) -> Poll<(), S::SinkError> {
        self.inner.borrow_mut().poll_complete()
    }
}
