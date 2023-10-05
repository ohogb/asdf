use crate::asdf::{ast, byte_code, tc, Result};

#[derive(Debug)]
pub struct Integer {
	value: i64,
}

impl Integer {
	pub fn new(value: i64) -> Self {
		return Self { value };
	}
}

impl ast::Node for Integer {
	fn emit(&self, ctx: &mut byte_code::Context) {
		// mov rax, self.value
		ctx.emit(
			[[0x48, 0xB8].to_vec(), self.value.to_ne_bytes().into()]
				.concat()
				.as_slice(),
		);
	}

	fn type_check(&self, _: &mut tc::Context) -> Result<tc::Type> {
		return Ok(tc::Type::I64);
	}
}
