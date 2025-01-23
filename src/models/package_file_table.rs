use std::{
	fs::File,
	io::{Read, Seek, SeekFrom},
	ops::Deref,
	path::PathBuf,
	sync::Arc,
};

use glob::MatchOptions;

use crate::root::ks::{self, PackageFileTableEntry};

use super::{FileEntry, PackedPackageBuffer};

pub struct PackedPackageFileTable {
	data: PackedPackageBuffer,
}

impl PackedPackageFileTable {
	pub fn read_from(path: impl Into<PathBuf>) -> Self {
		let path = path.into();
		let mut file = File::open(path).expect("Failed to load content package!");

		// Move file cursor to the table position
		let table_position = SeekFrom::End(-(ks::PACKAGE_FILE_TABLE_SIZE as i64));
		file.seek(table_position)
			.expect("Could not find file table!");

		let mut buffer = vec![0u8; ks::PACKAGE_FILE_TABLE_SIZE];
		file.read_exact(&mut buffer)
			.expect("Failed to read package file table!");

		Self {
			data: PackedPackageBuffer::new(buffer),
		}
	}

	pub fn guess_key(&self) -> u64 {
		let look_at = self.data.buffer.len() - 8;
		let candidate: u64 = *bytemuck::from_bytes(&self.data.buffer[look_at..]);
		candidate
	}

	pub fn unpacked(&self) -> PackageFileTable {
		let key = self.guess_key();
		let unpacked = self.data.unpacked(key);
		let entries = bytemuck::cast_slice::<_, ks::PackageFileTableEntry>(unpacked.deref());
		PackageFileTable::new(entries, key)
	}
}

pub struct PackageFileTable {
	pub entries: Arc<[ks::PackageFileTableEntry]>,
	pub key: u64,
}

impl PackageFileTable {
	pub fn new(entries: impl Into<Arc<[PackageFileTableEntry]>>, key: u64) -> Self {
		Self {
			entries: entries.into(),
			key,
		}
	}

	pub fn read_unpacked_from(path: impl Into<PathBuf>) -> Self {
		let packed_table = PackedPackageFileTable::read_from(path);
		packed_table.unpacked()
	}

	pub fn query(&self, pattern: &glob::Pattern) -> Vec<FileEntry> {
		self.entries
			.iter()
			.filter_map(|entry| {
				let entry = FileEntry::from(entry);
				let options = MatchOptions {
					case_sensitive: false,
					..Default::default()
				};
				let matches_query = pattern.matches_with(&entry.path, options)
					|| entry.path.starts_with(pattern.as_str());
				matches_query.then_some(entry)
			})
			.filter(|entry| entry.address != 0)
			.collect()
	}
}
