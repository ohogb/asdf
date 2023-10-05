use std::collections::HashMap;

use crate::asdf::tc;

struct FunctionData {
	variables: HashMap<String, (tc::Type, u32, u32)>,
	current_offset: u32,
}

pub struct ParsingContext<'a> {
	extern_functions: HashMap<String, (u64, tc::Type, Vec<tc::Type>)>,
	type_checking_context: &'a mut tc::Context,
	functions: Vec<FunctionData>,
}

impl<'a> ParsingContext<'a> {
	pub fn new(type_checking_context: &'a mut tc::Context) -> Self {
		return Self {
			extern_functions: HashMap::new(),
			type_checking_context,
			functions: Vec::new(),
		};
	}

	// TODO: error handling
	pub fn push_variable(&mut self, value_type: tc::Type, name: String, size: u32) -> u32 {
		let func = self.get_fn_mut();
		let current_offset = func.current_offset.clone();

		func.variables
			.insert(name, (value_type, current_offset, size));
		func.current_offset += size;

		return current_offset;
	}

	pub fn find_variable(&self, name: &str) -> Option<(tc::Type, u32, u32)> {
		let Some(ret) = self.get_fn().variables.get(name) else {
			return None;
		};

		return Some(ret.clone());
	}

	// TODO: error handling
	pub fn extern_function<T: Into<String>>(
		&mut self,
		name: T,
		function: u64,
		return_type: tc::Type,
		argument_types: Vec<tc::Type>,
	) {
		self.extern_functions
			.insert(name.into(), (function, return_type, argument_types));
	}

	pub fn find_extern_function(&self, name: &str) -> Option<(u64, tc::Type, Vec<tc::Type>)> {
		let Some(ret) = self.extern_functions.get(name) else {
			return None;
		};

		return Some(ret.clone());
	}

	pub fn get_type_checking_context(&mut self) -> &mut tc::Context {
		return self.type_checking_context;
	}

	pub fn push(&mut self) {
		self.functions.push(FunctionData {
			variables: HashMap::new(),
			current_offset: u32::default(),
		});
	}

	pub fn pop(&mut self) -> u32 {
		return self.functions.pop().unwrap().current_offset;
	}

	fn get_fn(&self) -> &FunctionData {
		return self.functions.last().unwrap();
	}

	fn get_fn_mut(&mut self) -> &mut FunctionData {
		return self.functions.last_mut().unwrap();
	}
}
