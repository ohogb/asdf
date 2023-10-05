use std::num::Wrapping;

use crate::asdf::{ast, byte_code, tc, Result};

#[derive(Debug)]
pub struct Stack {
	value_type: tc::Type,
	offset: u32,
	size: u32,
}

impl Stack {
	pub fn new(value_type: tc::Type, offset: u32, size: u32) -> Self {
		return Self {
			value_type,
			offset,
			size,
		};
	}
}

impl ast::Node for Stack {
	fn emit(&self, ctx: &mut byte_code::Context) {
		let offset = (Wrapping(0) - Wrapping(self.offset + self.size)).0;

		// lea rax, [rbp - offset]
		let bytes = [vec![0x48, 0x8D, 0x85], offset.to_ne_bytes().into()].concat();

		ctx.emit(bytes.as_slice());
	}

	fn type_check(&self, _: &mut tc::Context) -> Result<tc::Type> {
		return Ok(tc::Type::Reference(Box::new(self.value_type.clone())));
	}

	fn is_reference(&self) -> bool {
		return true;
	}
}
