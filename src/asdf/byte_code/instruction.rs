use std::num::Wrapping;

use crate::asdf::byte_code;

#[derive(Debug, Clone)]
pub struct Instruction {
	instruction: Vec<u8>,
	target: Option<(usize, usize)>,
	position: Option<usize>,
}

impl Instruction {
	pub fn new(instruction: &[u8]) -> Self {
		return Self {
			instruction: instruction.to_vec(),
			target: None,
			position: None,
		};
	}

	pub fn set_target(&mut self, target: usize, size: usize) {
		self.target = Some((target, size));
	}

	pub fn store_relative_position(&mut self, mut position: usize) -> usize {
		self.position = Some(position);
		position += self.get_size();

		return position;
	}

	pub fn calculate_relatives(&mut self, ctx: &byte_code::Context) {
		if self.target.is_none() {
			return;
		}

		let (label, size) = self.target.as_ref().unwrap();

		// TODO: this can panic if there isn't instructions after the label
		let instruction = ctx.get(*label).unwrap();

		let lhs = Wrapping(instruction.position.unwrap());
		let rhs = Wrapping(self.position.unwrap() + self.get_size());

		let diff = (lhs - rhs).0;

		// TODO: this is temporary
		assert!(*size == 4);
		let tmp = (diff & 0xFFFFFFFF) as u32;

		self.instruction.extend(tmp.to_ne_bytes());
	}

	pub fn get_instruction(&self) -> &Vec<u8> {
		return &self.instruction;
	}

	pub fn get_position(&self) -> Option<usize> {
		return self.position.clone();
	}

	fn get_size(&self) -> usize {
		let mut ret = self.instruction.len();

		if let Some((_, size)) = self.target {
			ret += size;
		}

		return ret;
	}
}
