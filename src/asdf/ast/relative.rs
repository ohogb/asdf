use crate::asdf::{ast, byte_code, tc, Result};

#[derive(Debug)]
pub struct Relative {
	function_name: String,
}

impl Relative {
	pub fn new(function_name: String) -> Self {
		return Self { function_name };
	}
}

impl ast::Node for Relative {
	fn emit(&self, ctx: &mut byte_code::Context) {
		// lea rax, [rip + target]
		let ins = ctx.emit(&[0x48, 0x8D, 0x05]);
		ctx.late_initialize_relative(ins, self.function_name.clone());
	}

	fn type_check(&self, ctx: &mut tc::Context) -> Result<tc::Type> {
		let Some((return_type, argument_types)) = ctx.find_function(&self.function_name) else {
			return Err(format!("cannot find function '{}'", self.function_name));
		};

		return Ok(tc::Type::Function(Box::new(return_type), argument_types));
	}
}
