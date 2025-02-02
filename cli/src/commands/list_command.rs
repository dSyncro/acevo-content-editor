use std::time::Instant;

use acevo_content_editor::{
	functions::list_query,
	models::{Benchmarked, FileEntry},
};

use crate::args::{GlobalOpts, ListArgs};

#[derive(Debug)]
pub struct ListCommand {
	pub global: GlobalOpts,
	pub args: ListArgs,
}

/// Public API
impl ListCommand {
	pub fn new(global: GlobalOpts, args: ListArgs) -> Self {
		Self { global, args }
	}

	pub fn run(&self) -> Benchmarked<Vec<FileEntry>> {
		let start = Instant::now();
		let entries = list_query(&self.global.content_path, &self.args.glob);

		Benchmarked {
			execution_time: start.elapsed(),
			data: entries,
		}
	}
}
