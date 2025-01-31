use std::time::Duration;

use crate::models::FileEntry;

pub struct Benchmarked<T> {
	pub execution_time: Duration,
	pub data: T,
}

#[derive(Debug, Default)]
pub struct ListResponse {
	pub execution_time: Duration,
	pub entries: Vec<FileEntry>,
}

#[derive(Debug, Default)]
pub struct UnpackResponse {
	pub execution_time: Duration,
	pub entries: Vec<FileEntry>,
}
