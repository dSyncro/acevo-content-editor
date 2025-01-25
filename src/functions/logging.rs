pub fn init_logging(verbosity: u8) {
	let level = match verbosity {
		0 => spdlog::Level::Warn,
		1 => spdlog::Level::Info,
		2 => spdlog::Level::Debug,
		_ => spdlog::Level::Trace,
	};

	let filter = spdlog::LevelFilter::MoreSevereEqual(level);
	spdlog::default_logger().set_level_filter(filter);
}
