use std::collections::VecDeque;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone)]
pub struct Event {
    pub id: u64,
    pub ts: u128,
    pub payload: String,
}

pub struct EventFlow {
    queue: VecDeque<Event>,
    cap: usize,
}

impl EventFlow {
    pub fn new(cap: usize) -> Self {
        Self {
            queue: VecDeque::with_capacity(cap),
            cap,
        }
    }

    pub fn push(&mut self, payload: String) {
        let ev = Event {
            id: Self::gen_id(),
            ts: Self::now(),
            payload,
        };

        if self.queue.len() >= self.cap {
            self.queue.pop_front();
        }

        self.queue.push_back(ev);
    }

    pub fn pull_batch(&mut self) -> Vec<Event> {
        let mut out = Vec::new();
        while let Some(e) = self.queue.pop_front() {
            out.push(e);
        }
        out
    }

    fn gen_id() -> u64 {
        (Self::now() % u64::MAX as u128) as u64
    }

    fn now() -> u128 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis()
    }
}
