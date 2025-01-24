use std::path::PathBuf;

use clap::{ArgAction, Args, Parser, Subcommand};
use glob::Pattern;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct CliArgs {
	#[clap(flatten)]
	pub global_opts: GlobalOpts,

	#[clap(subcommand)]
	pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
	/// List content from package
	List(ListArgs),
	/// Unpack content from package
	Unpack(UnpackArgs),
	UnpackAsync(UnpackArgs)
}

#[derive(Debug, Args)]
pub struct UnpackArgs {
	/// Pattern of the elements to look for
	#[clap(value_name = "GLOB", index = 1, default_value = "*")]
	pub glob: Pattern,

	/// Pattern of elements to force extract, even if they are already present
	#[clap(long, short = 'F')]
	pub force: Option<glob::Pattern>,
}

#[derive(Debug, Args)]
pub struct ListArgs {
	/// Pattern of the elements to look for
	#[clap(value_name = "GLOB", index = 1, default_value = "*")]
	pub glob: Pattern,
}

#[derive(Debug, Args)]
pub struct GlobalOpts {
	/// Verbosity level (can be specified multiple times)
	#[clap(long, short, global = true, action = ArgAction::Count)]
	pub verbose: u8,

	/// The path of the content package
	#[clap(long, short, env, global = true, default_value = "content.kspkg")]
	pub content_path: PathBuf,

	/// The path where to extract content data
	#[clap(long, short = 'o', env, global = true, default_value = "./")]
	pub content_output: PathBuf,
}
