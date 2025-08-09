use std::time::Duration;
use runtime::{Runtime, system_metrics::SystemMetrics};

#[test]
fn test_refresh_ne_1sec() {
    let mut rt = Runtime::default();
    let first = rt.to_string();
    std::thread::sleep(Duration::from_secs(1));
    rt.refresh();
    let second = rt.to_string();
    assert_ne!(first, second, "Runtime output should change after refresh");
}

#[test]
fn test_system_refresh_ne_1sec() {
    let mut system = SystemMetrics::new().expect("Failed to create SystemMetrics");
    let first = system.clone();
    std::thread::sleep(Duration::from_secs(1));
    system.refresh().expect("Failed to refresh SystemMetrics");
    assert_ne!(first, system, "SystemMetrics should change after refresh");
}

#[test]
fn test_system_metrics_creation() {
    let metrics = SystemMetrics::new().expect("Should be able to create SystemMetrics");
    assert!(metrics.uptime_seconds() > 0.0, "Uptime should be positive");
    assert!(metrics.user_count() > 0, "Should have at least one user");
}

#[test]
fn test_load_averages() {
    let metrics = SystemMetrics::new().expect("Should be able to create SystemMetrics");
    let (load1, load5, load15) = metrics.load_averages();
    assert!(load1 >= 0.0, "Load average should be non-negative");
    assert!(load5 >= 0.0, "Load average should be non-negative");
    assert!(load15 >= 0.0, "Load average should be non-negative");
}

#[test]
fn test_boot_time() {
    let metrics = SystemMetrics::new().expect("Should be able to create SystemMetrics");
    let boot_time = metrics.boot_time();
    assert!(boot_time > 0, "Boot time should be positive");

    // Boot time should be reasonable (not in the future, not too old)
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    assert!(boot_time <= now, "Boot time should not be in the future");
    assert!(boot_time > now - (365 * 24 * 3600), "Boot time should not be more than a year ago");
}
