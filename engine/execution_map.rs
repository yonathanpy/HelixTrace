use std::collections::HashMap;

pub struct ExecutionMap {
    edges: HashMap<String, Vec<String>>,
}

impl ExecutionMap {
    pub fn new() -> Self {
        Self {
            edges: HashMap::new(),
        }
    }

    pub fn link(&mut self, a: String, b: String) {
        self.edges.entry(a).or_insert(Vec::new()).push(b);
    }

    pub fn trace(&self, start: &str) -> Vec<String> {
        let mut path = Vec::new();

        if let Some(next) = self.edges.get(start) {
            for n in next {
                path.push(n.clone());
            }
        }

        path
    }
}
