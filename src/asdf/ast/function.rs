use crate::asdf::{ast, byte_code, tc, Result};

#[derive(Debug)]
pub struct Function {
	name: String,
	return_type: tc::Type,
	params: Vec<tc::Type>,
	nodes: Vec<ast::BoxedNode>,
	stack_size: u32,
}

impl Function {
	pub fn new(
		name: String,
		return_type: tc::Type,
		params: Vec<tc::Type>,
		nodes: Vec<ast::BoxedNode>,
		stack_size: u32,
	) -> Self {
		return Self {
			name,
			return_type,
			params,
			nodes,
			stack_size,
		};
	}
}

impl ast::Node for Function {
	fn emit(&self, ctx: &mut byte_code::Context) {
		ctx.define_function(self.name.clone(), ctx.get_current_position());

		// push rbp
		ctx.emit(&[0x55]);

		// mov rbp, rsp
		ctx.emit(&[0x48, 0x89, 0xE5]);

		// sub rsp, self.stack_size
		ctx.emit(
			[
				[0x48, 0x81, 0xEC].to_vec(),
				self.stack_size.to_ne_bytes().into(),
			]
			.concat()
			.as_slice(),
		);

		let mut current_offset: u32 = 0;

		for (i, typ) in self.params.iter().enumerate() {
			match i {
				// mov rax, rdi
				0 => ctx.emit(&[0x48, 0x89, 0xF8]),
				// mov rax, rsi
				1 => ctx.emit(&[0x48, 0x89, 0xF0]),
				_ => unreachable!(),
			};

			let offset = (std::num::Wrapping(0)
				- std::num::Wrapping(current_offset + typ.get_size() as u32))
			.0;

			// mov [rbp - offset], rax
			let bytes = [vec![0x48, 0x89, 0x85], offset.to_ne_bytes().into()].concat();

			ctx.emit(bytes.as_slice());
			current_offset += typ.get_size() as u32;
		}

		for i in &self.nodes {
			i.emit(ctx);
		}
	}

	fn pre_type_check(&self, ctx: &mut tc::Context) {
		ctx.define_function(
			self.name.clone(),
			self.return_type.clone(),
			self.params.clone(),
		);
	}

	fn type_check(&self, ctx: &mut tc::Context) -> Result<tc::Type> {
		for i in &self.nodes {
			i.type_check(ctx)?;
		}

		return Ok(self.return_type.clone());
	}
}
