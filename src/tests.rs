use std::time::Duration;

use crate::Runtime;

#[test]
fn test_refresh() {
    let mut rt = Runtime::default();
    let first = rt.to_string();
    std::thread::sleep(Duration::from_secs(1));
    rt.refresh();
    let second = rt.to_string();
    assert_ne!(first, second)

}