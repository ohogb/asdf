use crate::asdf::{ast, byte_code, tc, Result};

#[derive(Debug)]
pub struct ReturnStatement {
	value: Box<ast::BoxedNode>,
}

impl ReturnStatement {
	pub fn new(value: ast::BoxedNode) -> Self {
		return Self {
			value: Box::new(value),
		};
	}
}

impl ast::Node for ReturnStatement {
	fn emit(&self, ctx: &mut byte_code::Context) {
		self.value.emit(ctx);

		// mov rsp, rbp
		ctx.emit(&[0x48, 0x89, 0xEC]);

		// pop rbp
		ctx.emit(&[0x5D]);

		// ret
		ctx.emit(&[0xC3]);
	}

	fn pre_type_check(&self, ctx: &mut tc::Context) {
		self.value.pre_type_check(ctx);
	}

	fn type_check(&self, ctx: &mut tc::Context) -> Result<tc::Type> {
		let tc::Type::I64 = self.value.type_check(ctx)? else {
			return Err(format!(""));
		};

		return Ok(tc::Type::None);
	}
}
