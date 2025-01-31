use std::time::Duration;

use crate::models::FileEntry;

#[derive(Debug, Default)]
pub struct ListResponse {
	pub execution_time: Duration,
	pub entries: Vec<FileEntry>,
}
