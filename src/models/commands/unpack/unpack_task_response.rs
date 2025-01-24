#[derive(Debug, Clone)]
pub struct UnpackTaskResponse {
	pub has_been_skipped: bool,
	pub written_bytes: u64,
	pub path: String,
}

impl Default for UnpackTaskResponse {
	fn default() -> Self {
		Self {
			has_been_skipped: true,
			written_bytes: Default::default(),
			path: Default::default(),
		}
	}
}
