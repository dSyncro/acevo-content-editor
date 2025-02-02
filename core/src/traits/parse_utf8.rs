use std::ffi::{CStr, FromBytesUntilNulError};

pub trait ParseUtf8 {
	fn parse_utf8(&self) -> Result<String, FromBytesUntilNulError>;
}

impl ParseUtf8 for [u8] {
	fn parse_utf8(&self) -> Result<String, FromBytesUntilNulError> {
		let string = CStr::from_bytes_until_nul(self)?
			.to_string_lossy()
			.to_string();
		Ok(string)
	}
}
