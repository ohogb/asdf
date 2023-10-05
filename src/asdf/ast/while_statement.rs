use crate::asdf::{ast, byte_code, tc, Result};

#[derive(Debug)]
pub struct WhileStatement {
	condition: Box<ast::BoxedNode>,
	scope: ast::Scope,
}

impl WhileStatement {
	pub fn new(condition: ast::BoxedNode, scope: ast::Scope) -> Self {
		return Self {
			condition: Box::new(condition),
			scope,
		};
	}
}

impl ast::Node for WhileStatement {
	fn emit(&self, ctx: &mut byte_code::Context) {
		let condition = ctx.new_label();
		self.condition.emit(ctx);

		// test rax, rax
		ctx.emit(&[0x48, 0x85, 0xC0]);

		// jz exit
		let jmp_exit = ctx.emit(&[0x0F, 0x84]);

		self.scope.emit(ctx);

		// jmp condition
		let jmp_condition = ctx.emit(&[0xE9]);

		let exit = ctx.new_label();

		ctx.get_mut(jmp_exit).unwrap().set_target(exit, 0x4);
		ctx.get_mut(jmp_condition)
			.unwrap()
			.set_target(condition, 0x4);
	}

	fn pre_type_check(&self, ctx: &mut tc::Context) {
		self.condition.pre_type_check(ctx);
		self.scope.pre_type_check(ctx);
	}

	fn type_check(&self, ctx: &mut tc::Context) -> Result<tc::Type> {
		let tc::Type::I64 = self.condition.type_check(ctx)? else {
			return Err(format!(""));
		};

		self.scope.type_check(ctx)?;
		return Ok(tc::Type::None);
	}
}
