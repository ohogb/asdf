use crate::asdf::{ast, byte_code, tc, Result};

#[derive(Debug)]
pub struct Dereference {
	node: Box<ast::BoxedNode>,
}

impl Dereference {
	pub fn new(node: ast::BoxedNode) -> Self {
		return Self {
			node: Box::new(node),
		};
	}
}

impl ast::Node for Dereference {
	fn emit(&self, ctx: &mut byte_code::Context) {
		self.node.emit(ctx);

		// mov rax, [rax]
		ctx.emit(&[0x48, 0x8B, 0x00]);
	}

	fn pre_type_check(&self, ctx: &mut tc::Context) {
		self.node.pre_type_check(ctx);
	}

	fn type_check(&self, ctx: &mut tc::Context) -> Result<tc::Type> {
		let tc::Type::Reference(typ) = self.node.type_check(ctx)? else {
			return Err(format!(""));
		};

		return Ok(*typ);
	}
}
