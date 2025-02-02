use std::{path::PathBuf, sync::Arc};

use acevo_content_editor::models::FileEntry;
use tokio::{fs::File, sync::Mutex};

pub struct UnpackTaskData {
	pub entry: FileEntry,
	pub content_package: Arc<Mutex<File>>,
	pub output_path: PathBuf,
	pub force: Option<glob::Pattern>,
	pub key: u64,
}
