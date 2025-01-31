use std::time::Duration;

use humansize::{format_size, ToF64, Unsigned, DECIMAL};
use humantime::format_duration;

pub fn format_duration_ms(duration: Duration) -> String {
	let elapsed_ms = duration.as_millis();
	format_duration(Duration::from_millis(elapsed_ms as u64)).to_string()
}

pub fn format_size_decimal(size: impl ToF64 + Unsigned) -> String {
	format_size(size, DECIMAL)
}
