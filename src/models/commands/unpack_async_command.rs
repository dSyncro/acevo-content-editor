use std::{io::SeekFrom, ops::Deref, path::PathBuf, sync::Arc, time::Instant};

use colored::Colorize;
use humansize::{format_size, DECIMAL};
use tokio::{
	fs::File,
	io::{self, AsyncReadExt, AsyncSeekExt, AsyncWriteExt},
	sync::Mutex,
	task::JoinSet,
};

use crate::{
	args::{GlobalOpts, UnpackArgs},
	functions::format_duration_ms,
	models::{FileEntry, PackageFileTable, PackedPackageBuffer},
};

#[derive(Debug)]
pub struct UnpackCommandAsync {
	pub global: GlobalOpts,
	pub args: UnpackArgs,
}

impl UnpackCommandAsync {
	pub fn new(global: GlobalOpts, args: UnpackArgs) -> Self {
		Self { global, args }
	}
}

pub trait PureRunnableAsync {
	fn run(&self) -> impl std::future::Future<Output = ()> + Send;
}

impl PureRunnableAsync for UnpackCommandAsync {
	async fn run(&self) {
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

			tasks.spawn(unpack_entry(data));
		}

		let mut written_bytes = 0usize;
		let mut skipped_files = 0usize;
		while let Some(res) = tasks.join_next().await {
			if let Err(task_error) = res {
				log::error!("{task_error}");
				continue;
			}

			let res = res.unwrap();
			if let Err(unpack_error) = res {
				log::error!("{unpack_error}");
				continue;
			}

			let res = res.unwrap();

			written_bytes += res.written_bytes as usize;
			if res.has_been_skipped {
				log::info!("Skipping {}. It already exists", res.path.magenta());
				skipped_files += 1;
			} else {
				log::info!(
					"Unpacked {} with size {}",
					res.path.magenta(),
					format_size(res.written_bytes, DECIMAL).cyan()
				);
			}
		}

		let elapsed = start.elapsed();
		log::info!(
			"{}! Written {} of unpacked data using query {}. Skipped {} files. Took {} to execute.",
			"Unpack complete".green(),
			format_size(written_bytes, DECIMAL).cyan(),
			self.args.glob.as_str().yellow(),
			skipped_files.to_string().bright_purple(),
			format_duration_ms(elapsed).bright_blue()
		);
	}
}

struct UnpackTaskData {
	entry: FileEntry,
	content_package: Arc<Mutex<File>>,
	output_path: PathBuf,
	force: Option<glob::Pattern>,
	key: u64,
}

struct UnpackTaskResponse {
	has_been_skipped: bool,
	written_bytes: u64,
	path: String,
}
trait SeekReadAsync {
	async fn seek_read(&mut self, position: SeekFrom, buf: &mut [u8]) -> tokio::io::Result<()>;
}

impl SeekReadAsync for File {
	async fn seek_read(&mut self, position: SeekFrom, buf: &mut [u8]) -> tokio::io::Result<()> {
		self.seek(position).await?;
		self.read_exact(buf).await?;
		Ok(())
	}
}

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
