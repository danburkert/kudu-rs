use std::time::Duration;

pub fn duration_to_ms(duration: &Duration) -> u64 {
    duration.as_secs() + duration.subsec_nanos() as u64 / 1000_000
}
