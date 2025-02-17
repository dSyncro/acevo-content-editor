use acevo_content_editor::models::Benchmarked;
use clap::Parser;
use colored::Colorize;
use commands::{ListCommand, UnpackCommand};
use functions::{format_duration_ms, format_size_decimal, init_logging};
use spdlog::{info, trace};

mod args;
mod commands;
mod functions;

#[tokio::main]
async fn main() -> Result<(), ()> {
	let args = args::CliArgs::parse();

	init_logging(args.global_opts.verbose).unwrap();

	match args.command {
		args::Command::Unpack(unpack_args) => {
			let glob = unpack_args.glob.clone();

			let Benchmarked {
				execution_time,
				data: responses,
			} = UnpackCommand::new(args.global_opts, unpack_args)
				.run()
				.await;

			let matching_files = responses.len();

			let mut skipped_files = 0usize;
			let mut written_bytes = 0usize;
			for response in responses.iter() {
				if response.has_been_skipped {
					trace!("Skipping {}. It already exists", response.path.magenta());
					skipped_files += 1;
				} else {
					info!(
						"Unpacked {} with size {}",
						response.path.magenta(),
						format_size_decimal(response.written_bytes).cyan()
					);
					written_bytes += response.written_bytes as usize;
				}
			}

			info!(
				"{}! Written {} of unpacked data using query {}. Skipped {} files out of {} matching query. Took {} to execute.",
				"Unpack complete".green(),
				format_size_decimal(written_bytes).cyan(),
				glob.as_str().yellow(),
				skipped_files.to_string().bright_purple(),
				matching_files.to_string().bright_purple(),
				format_duration_ms(execution_time).bright_blue()
			);
		},
		args::Command::List(list_args) => {
			let pattern = list_args.glob.to_string();
			let Benchmarked {
				execution_time,
				data: entries,
			} = ListCommand::new(args.global_opts, list_args).run();
			for entry in entries.iter() {
				info!(
					"Found file {} at {} with size {}",
					entry.path.magenta(),
					format!("0x{:x}", entry.address).cyan(),
					format_size_decimal(entry.size).cyan()
				)
			}

			info!(
				"{}! Found {} entries matching query {}. Took {} to execute.",
				"Query complete".green(),
				entries.len().to_string().cyan(),
				pattern.yellow(),
				format_duration_ms(execution_time).bright_blue()
			);
		},
	}

	Ok(())
}
