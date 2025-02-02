use crate::{
	root::ks::{PackageFileFlags, PackageFileTableEntry},
	traits::ParseUtf8,
};

#[derive(Debug, Clone, Default)]
pub struct FileEntry {
	pub path: String,
	pub attributes: PackageFileFlags,
	pub size: u64,
	pub address: u64,
}

impl From<PackageFileTableEntry> for FileEntry {
	fn from(value: PackageFileTableEntry) -> Self {
		Self::from(&value)
	}
}

impl From<&PackageFileTableEntry> for FileEntry {
	fn from(value: &PackageFileTableEntry) -> Self {
		let path = value
			.file_path
			.parse_utf8()
			.expect("Could not read file path!");

		// Note: next line might be not really safe on UNIX or when using UNC paths
		// however AC Evo only supports Windows and we are only dealing with relative paths.
		let path = path.replace("\\", "/");

		Self {
			path,
			attributes: value.inf_flags,
			size: value.file_size,
			address: value.file_offs,
		}
	}
}
