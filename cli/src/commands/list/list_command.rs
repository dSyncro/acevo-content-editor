use std::{path::PathBuf, time::Instant};

use acevo_content_editor::models::{Benchmarked, FileEntry, PackageFileTable};
use glob::Pattern;

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

pub fn list_query(path: impl Into<PathBuf>, pattern: &Pattern) -> Vec<FileEntry> {
	let path = path.into();
	let file_table = PackageFileTable::read_unpacked_from(path);
	file_table.query(pattern)
}
