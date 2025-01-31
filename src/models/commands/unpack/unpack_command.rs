use std::{io::SeekFrom, ops::Deref, sync::Arc, time::Instant};

use spdlog::error;
use tokio::{
	fs::File,
	io::{self, AsyncWriteExt},
	sync::Mutex,
	task::JoinSet,
};

use crate::{
	args::{GlobalOpts, UnpackArgs},
	models::{Benchmarked, PackageFileTable, PackedPackageBuffer},
	traits::SeekReadAsync,
};

use super::{UnpackTaskData, UnpackTaskResponse};

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
			let data = UnpackTaskData {
				content_package: content_package.clone(),
				entry,
				force: self.args.force.clone(),
				key: file_table.key,
				output_path: output_path.to_owned(),
			};

			tasks.spawn(Self::unpack_entry(data));
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

/// Private members
impl UnpackCommand {
	async fn unpack_entry(data: UnpackTaskData) -> io::Result<UnpackTaskResponse> {
		let seek_position = SeekFrom::Start(data.entry.address);
		let mut buffer = vec![0u8; data.entry.size as usize];

		// Read the file
		let mut write_handle = data.content_package.lock().await;
		write_handle.seek_read(seek_position, &mut buffer).await?;
		drop(write_handle);

		// Prepare output
		let unpacked_buffer = PackedPackageBuffer::new(buffer).unpacked(data.key);
		let destination = data.output_path.join(&data.entry.path);
		let output_parent_path = destination.parent();
		if let Some(parent) = output_parent_path {
			tokio::fs::create_dir_all(parent).await?;
		}

		// Skip entry if we can
		if destination.exists() {
			match data.force {
				Some(pattern) if pattern.matches_path(&destination) => {},
				_ => {
					let response = UnpackTaskResponse {
						written_bytes: 0,
						has_been_skipped: true,
						path: data.entry.path,
					};
					return Ok(response);
				},
			}
		}

		// Write output
		let mut output_file = File::create(&destination).await?;
		output_file.write_all(unpacked_buffer.deref()).await?;

		let response = UnpackTaskResponse {
			written_bytes: data.entry.size,
			has_been_skipped: false,
			path: data.entry.path,
		};

		Ok(response)
	}
}
