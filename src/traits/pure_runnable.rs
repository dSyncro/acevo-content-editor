pub trait PureRunnable {
	fn run(&self);
}

pub trait PureRunnableAsync {
	fn run(&self) -> impl std::future::Future<Output = ()> + Send;
}
