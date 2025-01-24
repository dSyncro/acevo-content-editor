use std::{
	fs::File,
	io::{Read, Seek, SeekFrom, Write},
	ops::Deref, time::Instant,
};

use colored::Colorize;
use humansize::{make_format, DECIMAL};

use crate::{
	args::{GlobalOpts, UnpackArgs}, functions::format_duration_ms, models::{PackageFileTable, PackedPackageBuffer}, traits::PureRunnable
};

#[derive(Debug)]
pub struct UnpackCommand {
	pub global: GlobalOpts,
	pub args: UnpackArgs,
}

impl UnpackCommand {
	pub fn new(global: GlobalOpts, args: UnpackArgs) -> Self {
		Self { global, args }
	}
}

impl PureRunnable for UnpackCommand {
	fn run(&self) {
		let start = Instant::now();

		let content_path = self.global.content_path.as_path();
		let output_path = self.global.content_output.as_path();

		let mut content_package =
			File::open(content_path).expect("Could not open content package.");

		let file_table = PackageFileTable::read_unpacked_from(content_path);
		let files = file_table.query(&self.args.glob);

		let size_formatter = make_format(DECIMAL);
		let mut written_bytes = 0;
		let mut skipped_files = 0;
		files.iter().for_each(|entry| {
			let seek_position = SeekFrom::Start(entry.address);
			let mut buffer = vec![0u8; entry.size as usize];
			content_package
				.seek(seek_position)
				.expect("Could not seek position in content package.");
			content_package
				.read_exact(buffer.as_mut_slice())
				.expect("Could not read seeked content.");

			let unpacked_buffer = PackedPackageBuffer::new(buffer).unpacked(file_table.key);

			let destination = output_path.join(&entry.path);
			let output_parent_path = destination.parent();
			if let Some(parent) = output_parent_path {
				std::fs::create_dir_all(parent).unwrap();
			}

			if destination.exists() {
				match &self.args.force {
					Some(pattern) if pattern.matches_path(&destination) => {},
					_ => {
						println!("Skipping {}. It already exists.", &entry.path.magenta());
						skipped_files += 1;
						return;
					},
				}
			}

			let mut output_file =
				File::create(&destination).expect("Could not create destination file!");
			output_file
				.write_all(unpacked_buffer.deref())
				.expect("Could not write buffer to destination file!");

			println!(
				"Unpacked {} with size {}",
				&entry.path.magenta(),
				size_formatter(entry.size).cyan()
			);

			written_bytes += entry.size;
		});

		let elapsed = start.elapsed();

		println!(
			"{}! Written {} of unpacked data using query {}. Skipped {} files. Took {} to execute.",
			"Unpack complete".green(),
			size_formatter(written_bytes).cyan(),
			self.args.glob.as_str().yellow(),
			skipped_files.to_string().bright_purple(),
			format_duration_ms(elapsed).bright_blue()
		);
	}
}
