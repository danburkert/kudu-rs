use std::collections::VecDeque;
use std::usize;

pub struct QueueMap<T> {
    queue: VecDeque<Option<T>>,
    next_key: usize,
    len: usize,
}

impl <T> QueueMap<T> {

    pub fn new() -> QueueMap<T> {
        QueueMap {
            queue: VecDeque::new(),
            next_key: 0,
            len: 0,
        }
    }

    pub fn push(&mut self, value: T) -> usize {
        self.queue.push_back(Some(value));
        let key = self.next_key;
        self.next_key += 1;
        self.len += 1;
        key
    }

    pub fn push_with<F>(&mut self, f: F) where F: FnOnce(usize) -> T {
        self.queue.push_back(Some(f(self.next_key)));
        self.next_key += 1;
        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<(usize, T)> {
        let key = self.next_key - self.queue.len() as usize;
        let val = self.queue.pop_front().map(Option::unwrap);
        self.trim_front();
        if val.is_some() {
            self.len -= 1;
        }
        val.map(|val| (key, val))
    }

    pub fn remove(&mut self, key: usize) -> Option<T> {
        if key >= self.next_key || key < self.next_key - self.queue.len() {
            None
        } else {
            let offset = self.queue.len() - (self.next_key - key);
            let val = self.queue[offset].take();
            if val.is_some() {
                if offset == 0 {
                    self.trim_front();
                }
                self.len -= 1;
            }
            val
        }
    }

    fn trim_front(&mut self) {
        while let Some(&None) = self.queue.front() {
            self.queue.pop_front();
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
}

#[cfg(test)]
mod test {
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
    }
}
