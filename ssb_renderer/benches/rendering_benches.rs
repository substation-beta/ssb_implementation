// Imports
use microbench::{bench, Options};
use std::time::Duration;

// Benchmark
fn main() {
    bench(&Options::default().time(Duration::from_secs(2)), "Basic rendering.", || {
        
        // TODO: add basic rendering
        std::thread::sleep(Duration::from_millis(100));

    });
}