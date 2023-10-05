use crate::asdf::{byte_code, tc, Result};

pub trait Node: std::fmt::Debug {
	fn emit(&self, ctx: &mut byte_code::Context);

	fn pre_type_check(&self, _: &mut tc::Context) {}
	fn type_check(&self, ctx: &mut tc::Context) -> Result<tc::Type>;

	fn is_reference(&self) -> bool {
		return false;
	}
}
