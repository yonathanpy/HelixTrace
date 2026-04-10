use std::collections::VecDeque;

pub struct IoSurface {
    buffer: VecDeque<String>,
}

impl IoSurface {
    pub fn new() -> Self {
        Self {
            buffer: VecDeque::new(),
        }
    }

    pub fn capture(&mut self, data: String) {
        if self.buffer.len() > 100 {
            self.buffer.pop_front();
        }
        self.buffer.push_back(data);
    }

    pub fn snapshot(&self) -> Vec<String> {
        self.buffer.iter().cloned().collect()
    }
}
