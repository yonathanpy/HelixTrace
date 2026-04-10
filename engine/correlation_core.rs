use std::collections::HashMap;
use crate::engine::event_flow::Event;

pub struct CorrelationCore {
    chains: HashMap<u64, Vec<Event>>,
}

impl CorrelationCore {
    pub fn new() -> Self {
        Self {
            chains: HashMap::new(),
        }
    }

    pub fn ingest(&mut self, events: Vec<Event>) {
        for ev in events {
            let key = ev.id % 16;
            self.chains.entry(key).or_insert(Vec::new()).push(ev);
        }
    }

    pub fn analyze(&self) -> Vec<(u64, usize)> {
        let mut result = Vec::new();

        for (k, v) in &self.chains {
            result.push((*k, v.len()));
        }

        result
    }
}
