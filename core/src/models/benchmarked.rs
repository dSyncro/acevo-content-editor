use std::time::{Duration, Instant};

pub struct Benchmarked<T> {
	pub execution_time: Duration,
	pub data: T,
}

pub struct LazyBenchmarked<T> {
	pub start: Instant,
	pub data: T,
}

impl<T> LazyBenchmarked<T> {
	pub fn elapsed(&self) -> Duration {
		self.start.elapsed()
	}
}
