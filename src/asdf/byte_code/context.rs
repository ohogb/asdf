use std::collections::HashMap;

use crate::asdf::byte_code;

#[derive(Debug)]
pub struct Context {
	instructions: Vec<byte_code::Instruction>,
	stack_size: usize,
	late_init: Vec<(String, usize)>,
	functions: HashMap<String, usize>,
}

impl Context {
	pub fn new() -> Self {
		return Self {
			instructions: vec![],
			stack_size: usize::default(),
			late_init: Vec::new(),
			functions: HashMap::new(),
		};
	}

	pub fn new_label(&mut self) -> usize {
		return self.instructions.len();
	}

	pub fn emit(&mut self, instruction: &[u8]) -> usize {
		self.instructions
			.push(byte_code::Instruction::new(instruction));
		return self.get_current_position() - 1;
	}

	pub fn get(&self, label: usize) -> Option<&byte_code::Instruction> {
		return self.instructions.get(label);
	}

	pub fn get_mut(&mut self, label: usize) -> Option<&mut byte_code::Instruction> {
		return self.instructions.get_mut(label);
	}

	pub fn get_current_position(&self) -> usize {
		return self.instructions.len();
	}

	pub fn instructions(&mut self) -> Vec<byte_code::Instruction> {
		let mut position = usize::default();

		for i in &mut self.instructions {
			position = i.store_relative_position(position);
		}

		for (name, ins) in std::mem::take(&mut self.late_init) {
			let fun = self.functions.get(&name).unwrap().clone();
			self.get_mut(ins).unwrap().set_target(fun, 0x4);
		}

		// this is to get around the borrow checker
		let mut instructions = self.instructions.clone();

		for i in &mut instructions {
			i.calculate_relatives(self);
		}

		self.instructions = instructions;
		return self.instructions.clone();
	}

	// a temporary solution to keep track of the stack alignment
	pub fn push(&mut self, size: usize) {
		self.stack_size += size;
	}

	pub fn pop(&mut self, size: usize) {
		self.stack_size -= size;
	}

	pub fn get_stack_size(&self) -> usize {
		return self.stack_size.clone();
	}

	pub fn late_initialize_relative(&mut self, instruction: usize, function_name: String) {
		// TODO: this is to set the size, make a better way
		self.get_mut(instruction).unwrap().set_target(0, 0x4);

		self.late_init.push((function_name, instruction));
	}

	pub fn define_function(&mut self, function_name: String, position: usize) {
		self.functions.insert(function_name, position);
	}

	pub fn get_function_offset(&self, function_name: &str) -> Option<usize> {
		let Some(ins) = self.functions.get(function_name) else {
			return None;
		};

		return self.get(*ins)?.get_position();
	}
}
