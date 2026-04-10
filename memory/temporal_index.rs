use std::collections::BTreeMap;

pub struct TemporalIndex {
    map: BTreeMap<u128, String>,
}

impl TemporalIndex {
    pub fn new() -> Self {
        Self {
            map: BTreeMap::new(),
        }
    }

    pub fn insert(&mut self, ts: u128, value: String) {
        self.map.insert(ts, value);
    }

    pub fn range(&self, start: u128, end: u128) -> Vec<String> {
        self.map.range(start..end)
            .map(|(_, v)| v.clone())
            .collect()
    }
}
