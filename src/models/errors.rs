use std::{fs::File, io::SeekFrom, path::PathBuf};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum UnpackCommandError {
	#[error("could not open package at {0:?}")]
	CouldNotOpenContentPackage(PathBuf),
	#[error("could not seek to position {position:?} in file {file:?}")]
	CouldNotSeek { position: SeekFrom, file: File },
}
