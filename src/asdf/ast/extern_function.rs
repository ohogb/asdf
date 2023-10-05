use crate::asdf::{ast, byte_code, tc, Result};

#[derive(Debug)]
pub struct ExternFunction {
	address: u64,
	return_type: tc::Type,
	argument_types: Vec<tc::Type>,
}

impl ExternFunction {
	pub fn new(address: u64, return_type: tc::Type, argument_types: Vec<tc::Type>) -> Self {
		return Self {
			address,
			return_type,
			argument_types,
		};
	}
}

impl ast::Node for ExternFunction {
	fn emit(&self, ctx: &mut byte_code::Context) {
		// mov rax, self.address
		ctx.emit(
			[[0x48, 0xB8].to_vec(), self.address.to_ne_bytes().into()]
				.concat()
				.as_slice(),
		);
	}

	fn type_check(&self, _: &mut tc::Context) -> Result<tc::Type> {
		return Ok(tc::Type::Function(
			Box::new(self.return_type.clone()),
			self.argument_types.clone(),
		));
	}
}
