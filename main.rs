mod engine {
    pub mod event_flow;
    pub mod correlation_core;
    pub mod execution_map;
}

mod collectors {
    pub mod process_lineage;
    pub mod io_surface;
}

mod analysis {
    pub mod anomaly_vector;
    pub mod pattern_resolver;
}

mod memory {
    pub mod temporal_index;
    pub mod state_cache;
}

mod output {
    pub mod signal_dispatch;
    pub mod graph_encoder;
}

mod control {
    pub mod runtime;
    pub mod error_guard;
}

use control::runtime::Runtime;

fn main() {
    let mut runtime = Runtime::new();

    println!("[helixtrace] starting runtime...");

    for i in 0..10 {
        println!("[cycle] {}", i);

        runtime.tick();
    }

    println!("[helixtrace] shutdown complete");
}
