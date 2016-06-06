use std::time::Duration;

pub fn duration_to_ms(duration: &Duration) -> u64 {
    duration.as_secs() * 1000 + duration.subsec_nanos() as u64 / 1000_000
}
