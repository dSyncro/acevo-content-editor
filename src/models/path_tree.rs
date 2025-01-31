use std::collections::BTreeMap;

pub struct PathTree {
	pub name: String,
	pub children: BTreeMap<String, Self>,
}

impl PathTree {
	pub fn from_paths(paths: Vec<impl Into<String>>) -> Self {
		let mut root = Self::default();

		for path in paths {
			let path: String = path.into();
			let path = path.replace("\\", "/");
			let segments: Vec<_> = path
				.split("/")
				.filter(|segment| !segment.is_empty())
				.collect();
			root.add(segments);
		}

		root
	}

	pub fn add(&mut self, segments: Vec<impl Into<String>>) {
		let mut current_node = self;

		for segment in segments {
			let segment: String = segment.into();

			current_node = current_node
				.children
				.entry(segment.clone())
				.or_insert_with(|| PathTree {
					name: segment,
					children: BTreeMap::new(),
				});
		}
	}

	pub fn is_leaf(&self) -> bool {
		self.children.is_empty()
	}
}

impl Default for PathTree {
	fn default() -> Self {
		Self {
			name: "root".into(),
			children: Default::default(),
		}
	}
}
