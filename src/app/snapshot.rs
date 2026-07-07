use std::time::{SystemTime, UNIX_EPOCH};
use crate::domain::reading::Reading;
use crate::infra::sensors;

pub fn take() -> Reading{
    let cpu = sensors::read();
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
        .to_string();

    Reading{timestamp, cpu}
}
