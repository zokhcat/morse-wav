use std::thread::sleep;

pub fn sleep_ms(ms: u64) {
    sleep(std::time::Duration::from_millis(ms));
}