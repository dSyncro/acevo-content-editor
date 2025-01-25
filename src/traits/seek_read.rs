use std::io::{self, Read, Seek, SeekFrom};

use tokio::io::{AsyncReadExt, AsyncSeekExt};

pub trait SeekRead {
	fn seek_read(&mut self, position: SeekFrom, buffer: &mut [u8]) -> io::Result<()>;
}

impl SeekRead for std::fs::File {
	fn seek_read(&mut self, position: SeekFrom, buffer: &mut [u8]) -> io::Result<()> {
		self.seek(position)?;
		self.read_exact(buffer)?;
		Ok(())
	}
}

pub trait SeekReadAsync {
	fn seek_read(
		&mut self,
		position: SeekFrom,
		buffer: &mut [u8],
	) -> impl std::future::Future<Output = tokio::io::Result<()>> + Send;
}

impl SeekReadAsync for tokio::fs::File {
	async fn seek_read(&mut self, position: SeekFrom, buffer: &mut [u8]) -> tokio::io::Result<()> {
		self.seek(position).await?;
		self.read_exact(buffer).await?;
		Ok(())
	}
}
