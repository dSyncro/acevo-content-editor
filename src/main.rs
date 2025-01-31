use acevo_content_editor::{
	args::{self},
	functions::{format_duration_ms, init_logging},
	models::{ListCommand, UnpackCommand},
};
use clap::Parser;
use colored::Colorize;
use humansize::{format_size, DECIMAL};
use spdlog::info;

#[tokio::main]
async fn main() -> Result<(), ()> {
	let args = args::CliArgs::parse();

	init_logging(args.global_opts.verbose).unwrap();

	match args.command {
		args::Command::Unpack(unpack_args) => {
			UnpackCommand::new(args.global_opts, unpack_args)
				.run()
				.await
		},
		args::Command::List(list_args) => {
			let pattern = list_args.glob.to_string();
			let response = ListCommand::new(args.global_opts, list_args).run();
			for entry in response.entries.iter() {
				info!(
					"Found file {} at {} with size {}",
					entry.path.magenta(),
					format!("0x{:x}", entry.address).cyan(),
					format_size(entry.size, DECIMAL).cyan()
				)
			}

			info!(
				"{}! Found {} entries matching query {}. Took {} to execute.",
				"Query complete".green(),
				response.entries.len().to_string().cyan(),
				pattern.yellow(),
				format_duration_ms(response.execution_time).bright_blue()
			);
		},
	}

	Ok(())
}
