use crate::asdf::{ast, byte_code, tc, Result};

#[derive(Debug)]
pub struct CallStatement {
	function: Box<ast::BoxedNode>,
	arguments: Vec<ast::BoxedNode>,
}

impl CallStatement {
	pub fn new(function: ast::BoxedNode, arguments: Vec<ast::BoxedNode>) -> Self {
		return Self {
			function: Box::new(function),
			arguments,
		};
	}
}

impl ast::Node for CallStatement {
	fn emit(&self, ctx: &mut byte_code::Context) {
		assert!(self.arguments.len() <= 2);

		// push rdi
		ctx.emit(&[0x57]);
		ctx.push(0x8);

		for (i, node) in self.arguments.iter().enumerate() {
			// mov rax, argument
			node.emit(ctx);

			match i {
				// mov rdi, rax
				0 => ctx.emit(&[0x48, 0x89, 0xC7]),
				// mov rsi, rax
				1 => ctx.emit(&[0x48, 0x89, 0xC6]),
				_ => unreachable!(),
			};
		}

		// mov rax, function
		self.function.emit(ctx);

		// TODO: this is a temporary alignment for rsp, needs cleaning up
		let bytes_to_align = 0x10 - (ctx.get_stack_size() % 0x10) as u8;

		// sub rsp, bytes_to_align
		ctx.emit(&[0x48, 0x83, 0xEC, bytes_to_align]);

		// call rax
		ctx.emit(&[0xFF, 0xD0]);

		// add rsp, bytes_to_align
		ctx.emit(&[0x48, 0x83, 0xC4, bytes_to_align]);

		// pop rdi
		ctx.emit(&[0x5F]);
		ctx.pop(0x8);
	}

	fn pre_type_check(&self, ctx: &mut tc::Context) {
		self.function.pre_type_check(ctx);

		for i in &self.arguments {
			i.pre_type_check(ctx);
		}
	}

	fn type_check(&self, ctx: &mut tc::Context) -> Result<tc::Type> {
		let tc::Type::Function(ret, args) = self.function.type_check(ctx)? else {
			return Err(format!(""));
		};

		if self.arguments.len() != args.len() {
			return Err(format!(""));
		}

		for (i, node) in self.arguments.iter().enumerate() {
			if node.type_check(ctx)? != args[i] {
				return Err(format!(""));
			}
		}

		return Ok(*ret);
	}
}
