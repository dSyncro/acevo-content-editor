use std::time::Instant;

use colored::Colorize;
use humansize::{format_size, DECIMAL};

use crate::{
	args::{GlobalOpts, ListArgs}, functions::format_duration_ms, models::PackageFileTable, traits::PureRunnable
};

#[derive(Debug)]
pub struct ListCommand {
	pub global: GlobalOpts,
	pub args: ListArgs,
}

impl ListCommand {
	pub fn new(global: GlobalOpts, args: ListArgs) -> Self {
		Self { global, args }
	}
}

impl PureRunnable for ListCommand {
	fn run(&self) {
		let start = Instant::now();

		let file_table = PackageFileTable::read_unpacked_from(&self.global.content_path);

		let matches = file_table.query(&self.args.glob);
		matches.iter().for_each(|entry| {
			println!(
				"Found file {} at {} with size {}",
				entry.path.magenta(),
				format!("0x{:x}", entry.address).cyan(),
				format_size(entry.size, DECIMAL).cyan()
			)
		});

		let elapsed = start.elapsed();

		println!(
			"{}! Found {} entries matching query {}. Took {} to execute.",
			"Query complete".green(),
			matches.len().to_string().cyan(),
			self.args.glob.as_str().yellow(),
			format_duration_ms(elapsed).bright_blue()
		);
	}
}
