use std::{cell::Cell, path::PathBuf, sync::Arc};

use tokio::sync::Mutex;

use crate::models::FileEntry;

pub struct UnpackRequest {
	pub entry: FileEntry,
	pub content_package: Cell<std::fs::File>,
	pub output_path: PathBuf,
	pub force: Option<glob::Pattern>,
	pub key: u64,
}

#[cfg(feature = "async")]
pub struct UnpackTaskRequest {
	pub entry: FileEntry,
	pub content_package: Arc<Mutex<tokio::fs::File>>,
	pub output_path: PathBuf,
	pub force: Option<glob::Pattern>,
	pub key: u64,
}
