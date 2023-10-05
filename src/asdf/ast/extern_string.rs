use crate::asdf::{ast, byte_code, tc, Result};

#[derive(Debug)]
pub struct ExternString {
	value: String,
}

impl ExternString {
	pub fn new(value: String) -> Self {
		return Self { value };
	}
}

impl ast::Node for ExternString {
	fn emit(&self, ctx: &mut byte_code::Context) {
		// TODO: make an actual string struct that frees memory
		let ptr = Box::leak(Box::new(self.value.clone() + "\x00")).as_ptr() as u64;

		// mov rax, ptr
		ctx.emit(
			[[0x48, 0xB8].as_slice(), ptr.to_ne_bytes().as_slice()]
				.concat()
				.as_slice(),
		);
	}

	fn type_check(&self, _: &mut tc::Context) -> Result<tc::Type> {
		return Ok(tc::Type::Pointer(Box::new(tc::Type::Char)));
	}
}
