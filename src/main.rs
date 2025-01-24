use acevo_content_editor::{
	args::{self},
	functions::init_logging,
	models::{ListCommand, UnpackCommand},
};
use clap::Parser;

#[tokio::main]
async fn main() -> Result<(), ()> {
	init_logging();

	let args = args::CliArgs::parse();
	match args.command {
		args::Command::Unpack(unpack_args) => {
			UnpackCommand::new(args.global_opts, unpack_args)
				.run()
				.await
		},
		args::Command::List(list_args) => ListCommand::new(args.global_opts, list_args).run(),
	}

	Ok(())
}
