use std::path::PathBuf;

use crate::models::{FileEntry, PackageFileTable};

pub fn list_query(path: impl Into<PathBuf>, pattern: &glob::Pattern) -> Vec<FileEntry> {
	let path = path.into();
	let file_table = PackageFileTable::read_unpacked_from(path);
	file_table.query(pattern)
}

#[cfg(feature = "async")]
pub use async_feature::*;

#[cfg(feature = "async")]
mod async_feature {
	use std::{io::SeekFrom, ops::Deref};

	use tokio::{fs::File, io::AsyncWriteExt};

	use crate::{
		models::{PackedPackageBuffer, UnpackTaskData, UnpackTaskResponse},
		traits::SeekReadAsync,
	};

	pub async fn unpack_entry(data: UnpackTaskData) -> tokio::io::Result<UnpackTaskResponse> {
		let seek_position = SeekFrom::Start(data.entry.address);
		let mut buffer = vec![0u8; data.entry.size as usize];

		// Read the file
		let mut write_handle = data.content_package.lock().await;
		write_handle.seek_read(seek_position, &mut buffer).await?;
		drop(write_handle);

		// Prepare output
		let unpacked_buffer = PackedPackageBuffer::new(buffer).unpacked(data.key);
		let destination = data.output_path.join(&data.entry.path);
		let output_parent_path = destination.parent();
		if let Some(parent) = output_parent_path {
			tokio::fs::create_dir_all(parent).await?;
		}

		// Skip entry if we can
		if destination.exists() {
			match data.force {
				Some(pattern) if pattern.matches_path(&destination) => {},
				_ => {
					let response = UnpackTaskResponse {
						written_bytes: 0,
						has_been_skipped: true,
						path: data.entry.path,
					};
					return Ok(response);
				},
			}
		}

		// Write output
		let mut output_file = File::create(&destination).await?;
		output_file.write_all(unpacked_buffer.deref()).await?;

		let response = UnpackTaskResponse {
			written_bytes: data.entry.size,
			has_been_skipped: false,
			path: data.entry.path,
		};

		Ok(response)
	}
}
