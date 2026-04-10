use std::collections::VecDeque;

pub struct BufferWindow<T> {
    buf: VecDeque<T>,
    cap: usize,
}

impl<T: Clone> BufferWindow<T> {
    pub fn new(cap: usize) -> Self {
        Self {
            buf: VecDeque::with_capacity(cap),
            cap,
        }
    }

    pub fn push(&mut self, item: T) {
        if self.buf.len() >= self.cap {
            self.buf.pop_front();
        }
        self.buf.push_back(item);
    }

    pub fn snapshot(&self) -> Vec<T> {
        self.buf.iter().cloned().collect()
    }
}
