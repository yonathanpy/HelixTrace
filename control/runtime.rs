use crate::engine::event_flow::EventFlow;
use crate::engine::correlation_core::CorrelationCore;
use crate::output::signal_dispatch::SignalDispatch;

pub struct Runtime {
    flow: EventFlow,
    core: CorrelationCore,
}

impl Runtime {
    pub fn new() -> Self {
        Self {
            flow: EventFlow::new(256),
            core: CorrelationCore::new(),
        }
    }

    pub fn tick(&mut self) {
        self.flow.push("exec:process".into());
        self.flow.push("net:connect".into());

        let batch = self.flow.pull_batch();
        self.core.ingest(batch);

        let res = self.core.analyze();

        for (k, v) in res {
            if v > 5 {
                SignalDispatch::alert(2, &format!("cluster {} size {}", k, v));
            }
        }
    }
}
