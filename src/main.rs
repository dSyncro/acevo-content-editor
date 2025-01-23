use acevo_content_editor::{
	args::{self},
	models::{ListCommand, UnpackCommand},
	traits::PureRunnable,
};
use clap::Parser;

fn main() {
	let args = args::CliArgs::parse();
	match args.command {
		args::Command::Unpack(unpack_args) => {
			UnpackCommand::new(args.global_opts, unpack_args).run()
		},
		args::Command::List(list_args) => ListCommand::new(args.global_opts, list_args).run(),
	}
}
