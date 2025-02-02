use std::sync::Arc;

use spdlog::{
	sink::{AsyncPoolSink, OverflowPolicy},
	Error, Logger,
};

pub fn init_logging(verbosity: u8) -> Result<Arc<Logger>, Error> {
	let level = match verbosity {
		0 => spdlog::Level::Warn,
		1 => spdlog::Level::Info,
		2 => spdlog::Level::Debug,
		_ => spdlog::Level::Trace,
	};

	let filter = spdlog::LevelFilter::MoreSevereEqual(level);
	let logger = spdlog::default_logger().fork_with(|new| {
		let file_sink = Arc::new(
			AsyncPoolSink::builder()
				.overflow_policy(OverflowPolicy::Block)
				.level_filter(filter)
				.build()?,
		);
		new.sinks_mut().push(file_sink);
		Ok(())
	})?;

	spdlog::set_default_logger(logger.clone());

	Ok(logger)
}
