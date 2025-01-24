use std::env;

pub fn init_logging() {
	if env::var_os("RUST_LOG").is_none() {
		env::set_var("RUST_LOG", "info");
	}

	env_logger::builder().format_target(false).init();
}
