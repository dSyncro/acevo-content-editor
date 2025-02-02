use std::{sync::Arc, time::Instant};

use acevo_content_editor::{
	functions::unpack_entry_async,
	models::{Benchmarked, PackageFileTable, UnpackTaskRequest, UnpackTaskResponse},
};
use spdlog::error;
use tokio::{fs::File, sync::Mutex, task::JoinSet};

use crate::args::{GlobalOpts, UnpackArgs};

#[derive(Debug)]
pub struct UnpackCommand {
	pub global: GlobalOpts,
	pub args: UnpackArgs,
}

/// Public API
impl UnpackCommand {
	pub fn new(global: GlobalOpts, args: UnpackArgs) -> Self {
		Self { global, args }
	}

	pub async fn run(&self) -> Benchmarked<Vec<UnpackTaskResponse>> {
		let start = Instant::now();

		let content_path = self.global.content_path.as_path();
		let output_path = self.global.content_output.as_path();

		let content_package = File::open(content_path)
			.await
			.expect("Could not open content package.");
		let content_package = Arc::new(Mutex::new(content_package));

		let file_table = PackageFileTable::read_unpacked_from(content_path);
		let files = file_table.query(&self.args.glob);

		let mut tasks = JoinSet::new();
		for entry in files {
			let request = UnpackTaskRequest {
				content_package: content_package.clone(),
				entry,
				force: self.args.force.clone(),
				key: file_table.key,
				output_path: output_path.to_owned(),
			};

			tasks.spawn(unpack_entry_async(request));
		}

		let responses = tasks.join_all().await;
		let responses = responses.into_iter().filter_map(|res| match res {
			Err(unpack_error) => {
				error!("{unpack_error}");
				None
			},
			_ => Some(res.unwrap()),
		});

		Benchmarked {
			execution_time: start.elapsed(),
			data: responses.collect(),
		}
	}
}
