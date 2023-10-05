use std::collections::HashMap;

use crate::asdf::tc;

pub struct Context {
	defined_functions: HashMap<String, (tc::Type, Vec<tc::Type>)>,
}

impl Context {
	pub fn new() -> Self {
		return Self {
			defined_functions: HashMap::new(),
		};
	}

	pub fn define_function(
		&mut self,
		function_name: String,
		return_type: tc::Type,
		argument_types: Vec<tc::Type>,
	) {
		self.defined_functions
			.insert(function_name, (return_type, argument_types));
	}

	pub fn find_function(&self, function_name: &str) -> Option<(tc::Type, Vec<tc::Type>)> {
		let Some(ret) = self.defined_functions.get(function_name) else {
			return None;
		};

		return Some(ret.clone());
	}
}
