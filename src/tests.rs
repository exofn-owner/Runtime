use std::time::Duration;

use crate::{Runtime, system_metrics};

#[test]
fn test_refresh_ne_1sec() {
    let mut rt = Runtime::default();
    let first = rt.to_string();
    std::thread::sleep(Duration::from_secs(1));
    rt.refresh();
    let second = rt.to_string();
    assert_ne!(first, second)
}

#[test]
fn test_system_refresh_ne_1sec() {
    let mut system = system_metrics::SystemMetrics::new();
    let first = system.clone();
    std::thread::sleep(Duration::from_secs(1));
    system.refresh();
    assert_ne!(first, system)
}
