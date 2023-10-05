use crate::asdf::{ast, byte_code, tc, Result};

#[derive(Debug)]
pub struct Scope {
	nodes: Vec<ast::BoxedNode>,
}

impl Scope {
	pub fn new(nodes: Vec<ast::BoxedNode>) -> Self {
		return Self { nodes };
	}
}

impl ast::Node for Scope {
	fn emit(&self, ctx: &mut byte_code::Context) {
		for i in &self.nodes {
			i.emit(ctx);
		}
	}

	fn pre_type_check(&self, ctx: &mut tc::Context) {
		for i in &self.nodes {
			i.pre_type_check(ctx);
		}
	}

	fn type_check(&self, ctx: &mut tc::Context) -> Result<tc::Type> {
		for i in &self.nodes {
			i.type_check(ctx)?;
		}

		return Ok(tc::Type::None);
	}
}
