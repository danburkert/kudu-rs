use std::collections::{self, VecDeque};
use std::fmt;
use std::mem;
use std::ops::{Index, IndexMut};
use std::usize;

/// `QueueMap` is a container with some of the properties of a FIFO queue, and some of the
/// properties of a an associative map.
///
/// Elements in a `QueueMap` are associated with a `usize` key, which is automatically assigned
/// when using the queue-like API. Individual elements can also be inserted or removed by key, just
/// like a map. `QueueMap` is space efficient when all elements have keys which are close together.
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct QueueMap<T> {
    queue: VecDeque<Option<T>>,
    next_key: usize,
    len: usize,
}

impl <T> QueueMap<T> {

    /// Creates a new `QueueMap`.
    pub fn new() -> QueueMap<T> {
        QueueMap {
            queue: VecDeque::new(),
            next_key: 0,
            len: 0,
        }
    }

    /// Creates a new `QueueMap` with the provided capacity.
    pub fn with_capacity(capacity: usize) -> QueueMap<T> {
        QueueMap {
            queue: VecDeque::with_capacity(capacity),
            next_key: 0,
            len: 0,
        }
    }

    /// Enqueues an element and returns the key.
    pub fn push(&mut self, value: T) -> usize {
        self.queue.push_back(Some(value));
        let key = self.next_key;
        self.next_key += 1;
        self.len += 1;
        key
    }

    /// Enqueues an element produced by a function.
    ///
    /// The function argument is the element's key. This is useful for enqueing elements which must
    /// know their own key.
    pub fn push_with<F>(&mut self, f: F) where F: FnOnce(usize) -> T {
        self.queue.push_back(Some(f(self.next_key)));
        self.next_key += 1;
        self.len += 1;
    }

    /// Dequeues an element, returning the key and value.
    pub fn pop(&mut self) -> Option<(usize, T)> {
        let key = self.key_offset();
        let val = self.queue.pop_front().map(Option::unwrap);
        self.trim_front();
        if val.is_some() {
            self.len -= 1;
        }
        val.map(|val| (key, val))
    }

    /// Removes an element by key from the `QueueMap`.
    pub fn remove(&mut self, key: usize) -> Option<T> {
        if key >= self.next_key || key < self.key_offset() {
            None
        } else {
            let offset = self.queue.len() - (self.next_key - key);
            let val = self.queue[offset].take();
            if val.is_some() {
                self.len -= 1;
            }
            self.trim_front();
            val
        }
    }

    /// Inserts an element into the `QueueMap`, returning the previous element at the key.
    pub fn insert(&mut self, key: usize, value: T) -> Option<T> {
        if key >= self.next_key {
            for _ in self.next_key..key {
                self.queue.push_back(None);
            }
            self.queue.push_back(Some(value));
            self.next_key = key + 1;
            self.len += 1;
            None
        } else if key < self.key_offset() {
            for k in (key+1)..self.key_offset() {
                println!("pushing empty key: {}", k);
                self.queue.push_front(None);
            }
            self.queue.push_front(Some(value));
            self.len += 1;
            None
        } else {
            let offset = self.queue.len() - (self.next_key - key);
            let val = mem::replace(&mut self.queue[offset], Some(value));
            if val.is_none() {
                self.len += 1;
            }
            val
        }
    }

    /// Returns the number of elements in the `QueueMap`.
    pub fn len(&self) -> usize {
        self.len
    }

    /// Returns `true` if the `QueueMap` contains no elements.
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Returns an iterator of the `QueueMap`s elements in FIFO order.
    pub fn iter(&self) -> Iter<T> {
        Iter {
            key: self.key_offset(),
            itr: self.queue.iter(),
        }
    }

    /// Returns a mutable iterator of the `QueueMap`s elements in FIFO order.
    pub fn iter_mut(&mut self) -> IterMut<T> {
        IterMut {
            key: self.key_offset(),
            itr: self.queue.iter_mut(),
        }
    }

    /// TODO: add range argument once it lands in stable.
    pub fn drain(&mut self) -> Drain<T> {
        self.len = 0;
        Drain {
            key: self.key_offset(),
            itr: self.queue.drain(..),
        }
    }

    fn key_offset(&self) -> usize {
        self.next_key - self.queue.len() as usize
    }

    /// The key of the least recently queued element, or `None` if the `QueueMap` is empty.
    pub fn front_key(&self) -> Option<usize> {
        if self.is_empty() {
            None
        } else {
            Some(self.key_offset())
        }
    }

    /// The key of the most recently queued element, or `None` if the `QueueMap` is empty.
    pub fn back_key(&self) -> Option<usize> {
        if self.is_empty() {
            None
        } else {
            Some(self.next_key - 1)
        }
    }

    fn trim_front(&mut self) {
        while let Some(&None) = self.queue.front() {
            self.queue.pop_front();
        }
    }
}

impl <T> Index<usize> for QueueMap<T> {
    type Output = T;
    fn index(&self, key: usize) -> &T {
        let offset = self.queue.len() - (self.next_key - key);
        self.queue[offset].as_ref().unwrap()
    }
}

impl <T> IndexMut<usize> for QueueMap<T> {
    fn index_mut(&mut self, key: usize) -> &mut T {
        let offset = self.queue.len() - (self.next_key - key);
        self.queue[offset].as_mut().unwrap()
    }
}

impl <'a, T> IntoIterator for &'a QueueMap<T> where T: 'a {
    type Item = (usize, &'a T);
    type IntoIter = Iter<'a, T>;
    fn into_iter(self) -> Iter<'a, T> {
        self.iter()
    }
}
impl <'a, T> IntoIterator for &'a mut QueueMap<T> where T: 'a {
    type Item = (usize, &'a mut T);
    type IntoIter = IterMut<'a, T>;
    fn into_iter(self) -> IterMut<'a, T> {
        self.iter_mut()
    }
}
impl <T> IntoIterator for QueueMap<T> {
    type Item = (usize, T);
    type IntoIter = IntoIter<T>;
    fn into_iter(self) -> IntoIter<T> {
        IntoIter {
            key: self.key_offset(),
            itr: self.queue.into_iter(),
        }
    }
}

/// `QueueMap` iterator.
pub struct Iter<'a, T> where T: 'a {
    key: usize,
    itr: collections::vec_deque::Iter<'a, Option<T>>,
}
impl <'a, T> Iterator for Iter<'a, T> {
    type Item = (usize, &'a T);
    fn next(&mut self) -> Option<(usize, &'a T)> {
        loop {
            self.key += 1;
            match self.itr.next() {
                Some(&Some(ref value)) => return Some((self.key - 1, value)),
                Some(&None) => (),
                None => return None,
            }
        }
    }
}
/// `QueueMap` mutable iterator.
pub struct IterMut<'a, T> where T: 'a {
    key: usize,
    itr: collections::vec_deque::IterMut<'a, Option<T>>,
}
impl <'a, T> Iterator for IterMut<'a, T> {
    type Item = (usize, &'a mut T);
    fn next(&mut self) -> Option<(usize, &'a mut T)> {
        loop {
            self.key += 1;
            match self.itr.next() {
                Some(&mut Some(ref mut value)) => return Some((self.key - 1, value)),
                Some(&mut None) => (),
                None => return None,
            }
        }
    }
}
/// `QueueMap` owned iterator.
pub struct IntoIter<T> {
    key: usize,
    itr: collections::vec_deque::IntoIter<Option<T>>,
}
impl <T> Iterator for IntoIter<T> {
    type Item = (usize, T);
    fn next(&mut self) -> Option<(usize, T)> {
        loop {
            self.key += 1;
            match self.itr.next() {
                Some(Some(value)) => return Some((self.key - 1, value)),
                Some(None) => (),
                None => return None,
            }
        }
    }
}
/// `QueueMap` draining iterator.
pub struct Drain<'a, T> where T: 'a {
    key: usize,
    itr: collections::vec_deque::Drain<'a, Option<T>>,
}
impl <'a, T> Iterator for Drain<'a, T> {
    type Item = (usize, T);
    fn next(&mut self) -> Option<(usize, T)> {
        loop {
            self.key += 1;
            match self.itr.next() {
                Some(Some(value)) => return Some((self.key - 1, value)),
                Some(None) => (),
                None => return None,
            }
        }
    }
}

impl <T> fmt::Debug for QueueMap<T> where T: fmt::Debug  {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "["));
        let mut first = true;
        for (k, v) in self {
            if first { first = false; }
            else { try!(write!(f, ", ")); }
            try!(write!(f, "({}, {:?})", k, v));
        }
        write!(f, "]")
    }
}

#[cfg(test)]
mod test {

    use std::collections::BTreeMap;

    use super::*;

    #[test]
    fn queue_map() {
        let mut queue = QueueMap::new();

        for i in 0..10 {
            assert_eq!(i, queue.push(i));
        }
        assert_eq!(10, queue.len());

        assert_eq!(Some((0, 0)), queue.pop());
        assert_eq!(9, queue.len());

        assert_eq!(Some(3), queue.remove(3));
        assert_eq!(8, queue.len());

        assert_eq!(None, queue.remove(0));
        assert_eq!(None, queue.remove(3));
        assert_eq!(None, queue.remove(15));
        assert_eq!(8, queue.len());

        assert_eq!(None, queue.insert(0, 0));
        assert_eq!(None, queue.insert(3, 3));
        assert_eq!(Some(3), queue.insert(3, 3));
        assert_eq!(10, queue.len());
        assert_eq!(Some(0), queue.remove(0));
        assert_eq!(Some(3), queue.remove(3));

        assert_eq!(Some((1, 1)), queue.pop());
        assert_eq!(Some((2, 2)), queue.pop());
        assert_eq!(Some((4, 4)), queue.pop());
        assert_eq!(5, queue.len());

        assert_eq!(Some(6), queue.remove(6));
        assert_eq!(Some(7), queue.remove(7));
        assert_eq!(Some(5), queue.remove(5));
        assert_eq!(2, queue.len());

        assert_eq!(10, queue.push(10));
        assert_eq!(3, queue.len());
        assert_eq!("[(8, 8), (9, 9), (10, 10)]", &format!("{:?}", queue));

        assert_eq!(Some((8, 8)), queue.pop());
        assert_eq!(Some((9, 9)), queue.pop());
        assert_eq!(Some((10, 10)), queue.pop());
        assert_eq!(0, queue.len());
        assert_eq!(None, queue.pop());

        assert_eq!(11, queue.push(11));
        assert_eq!(None, queue.remove(10));
        assert_eq!(None, queue.remove(12));
        assert_eq!(Some(11), queue.remove(11));
        assert_eq!(0, queue.len());
        assert_eq!("[]", &format!("{:?}", queue));

        assert_eq!(None, queue.insert(3, 3));
        assert_eq!(None, queue.insert(12, 12));
        assert_eq!(13, queue.push(13));
        assert_eq!(3, queue.len());
        assert_eq!("[(3, 3), (12, 12), (13, 13)]", &format!("{:?}", queue));

        assert_eq!(Some((3, 3)), queue.pop());
        assert_eq!(Some((12, 12)), queue.pop());
        assert_eq!(Some((13, 13)), queue.pop());
        assert_eq!(None, queue.pop());
        assert_eq!(0, queue.len());
        assert_eq!(14, queue.push(14));
        assert_eq!(1, queue.len());
    }

    #[test]
    fn check_map() {
        quickcheck! {
            fn map(elements: Vec<(usize, usize)>) -> bool {
                let mut qm = QueueMap::new();
                let mut bm = BTreeMap::new();
                for (k, v) in elements {
                    if qm.insert(k, v) != bm.insert(k, v) {
                        return false;
                    }
                }

                if qm.iter().collect::<Vec<_>>() != bm.iter().map(|(&k, v)| (k, v)).collect::<Vec<_>>() {
                    return false;
                }

                if qm.iter_mut().collect::<Vec<_>>() != bm.iter_mut().map(|(&k, v)| (k, v)).collect::<Vec<_>>() {
                    return false;
                }

                {
                    let mut qm = qm.clone();
                    if qm.drain().collect::<Vec<_>>() != bm.clone().into_iter().collect::<Vec<_>>() {
                        return false;
                    }
                }

                qm.into_iter().collect::<Vec<_>>() == bm.into_iter().collect::<Vec<_>>()
            }
        }
    }
}
