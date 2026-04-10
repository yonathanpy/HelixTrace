use std::collections::HashMap;

pub struct ProcessLineage {
    tree: HashMap<u32, u32>,
}

impl ProcessLineage {
    pub fn new() -> Self {
        Self {
            tree: HashMap::new(),
        }
    }

    pub fn record(&mut self, pid: u32, parent: u32) {
        self.tree.insert(pid, parent);
    }

    pub fn lineage(&self, pid: u32) -> Vec<u32> {
        let mut chain = Vec::new();
        let mut current = pid;

        while let Some(p) = self.tree.get(&current) {
            chain.push(*p);
            current = *p;
        }

        chain
    }
}
