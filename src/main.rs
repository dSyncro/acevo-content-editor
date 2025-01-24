use std::{env, io};

use acevo_content_editor::{
	args::{self},
	models::{ListCommand, PureRunnableAsync, UnpackCommand, UnpackCommandAsync},
	traits::PureRunnable,
};
use clap::Parser;

fn init_logging() {
	if env::var_os("RUST_LOG").is_none() {
		env::set_var("RUST_LOG", "info");
	}

	env_logger::builder().format_target(false).init();
}

#[tokio::main]
async fn main() -> io::Result<()> {
	init_logging();

	let args = args::CliArgs::parse();
	match args.command {
		args::Command::Unpack(unpack_args) => {
			UnpackCommand::new(args.global_opts, unpack_args).run()
		},
		args::Command::List(list_args) => ListCommand::new(args.global_opts, list_args).run(),
		args::Command::UnpackAsync(unpack_args) => {
			UnpackCommandAsync::new(args.global_opts, unpack_args)
				.run()
				.await
		},
	}

	Ok(())
}
