use std::{ops::Deref, sync::Arc};

#[derive(Debug, Default, Clone)]
pub struct PackedPackageBuffer {
	pub buffer: Arc<[u8]>,
}

impl PackedPackageBuffer {
	pub fn new(buffer: impl Into<Arc<[u8]>>) -> Self {
		Self {
			buffer: buffer.into(),
		}
	}

	pub fn unpacked(&self, key: u64) -> Arc<[u8]> {
		let mut work_buffer = self.get_work_buffer();
		let wbuffer: &mut [u64] = bytemuck::cast_slice_mut(work_buffer.as_mut_slice());
		wbuffer.iter_mut().for_each(|word| *word ^= key);
		bytemuck::cast_slice(wbuffer)[..self.buffer.len()].into()
	}

	fn get_work_buffer(&self) -> Vec<u8> {
		let padding = 8 - self.buffer.len() % 8;
		let mut output = self.buffer.deref().to_vec();
		output.resize(self.buffer.len() + padding, 0);
		output
	}
}