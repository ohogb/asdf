use crate::asdf::{ast, byte_code, operators::Operators, tc, Result};

#[derive(Debug)]
pub struct BinaryOperation {
	operator: Operators,
	lhs: Box<ast::BoxedNode>,
	rhs: Box<ast::BoxedNode>,
}

impl BinaryOperation {
	pub fn new(operator: Operators, lhs: ast::BoxedNode, rhs: ast::BoxedNode) -> Self {
		return Self {
			operator,
			lhs: Box::new(lhs),
			rhs: Box::new(rhs),
		};
	}

	fn assignment(&self, ctx: &mut byte_code::Context) {
		// push rbx
		ctx.emit(&[0x53]);
		ctx.push(0x8);

		self.lhs.emit(ctx);

		// mov rbx, rax
		ctx.emit(&[0x48, 0x89, 0xC3]);

		self.rhs.emit(ctx);

		// mov [rbx], rax
		ctx.emit(&[0x48, 0x89, 0x03]);

		// pop rbx
		ctx.emit(&[0x5B]);
		ctx.pop(0x8);
	}

	fn addition(&self, ctx: &mut byte_code::Context) {
		// push rbx
		ctx.emit(&[0x53]);
		ctx.push(0x8);

		self.lhs.emit(ctx);

		// mov rbx, rax
		ctx.emit(&[0x48, 0x89, 0xC3]);

		self.rhs.emit(ctx);

		// add rax, rbx
		ctx.emit(&[0x48, 0x01, 0xD8]);

		// pop rbx
		ctx.emit(&[0x5B]);
		ctx.pop(0x8);
	}

	fn subtraction(&self, ctx: &mut byte_code::Context) {
		// mov rax, rhs
		self.rhs.emit(ctx);

		// push rbx
		ctx.emit(&[0x53]);
		ctx.push(0x8);

		// mov rbx, rax
		ctx.emit(&[0x48, 0x89, 0xC3]);

		// mov rax, lhs
		self.lhs.emit(ctx);

		// sub rax, rbx
		ctx.emit(&[0x48, 0x29, 0xD8]);

		// pop rbx
		ctx.emit(&[0x5B]);
		ctx.pop(0x8);
	}

	fn multiplication(&self, ctx: &mut byte_code::Context) {
		// push rbx
		ctx.emit(&[0x53]);
		ctx.push(0x8);

		self.lhs.emit(ctx);

		// mov rbx, rax
		ctx.emit(&[0x48, 0x89, 0xC3]);

		self.rhs.emit(ctx);

		// imul rax, rbx
		ctx.emit(&[0x48, 0x0F, 0xAF, 0xC3]);

		// pop rbx
		ctx.emit(&[0x5B]);
		ctx.pop(0x8);
	}

	fn division(&self, ctx: &mut byte_code::Context) {
		// push rdx
		ctx.emit(&[0x52]);
		ctx.push(0x8);

		// xor rdx, rdx
		ctx.emit(&[0x48, 0x31, 0xD2]);

		// push rbx
		ctx.emit(&[0x53]);
		ctx.push(0x8);

		// mov rax, rhs
		self.rhs.emit(ctx);

		// mov rbx, rax
		ctx.emit(&[0x48, 0x89, 0xC3]);

		// mov rax, lhs
		self.lhs.emit(ctx);

		// idiv rbx
		ctx.emit(&[0x48, 0xF7, 0xFB]);

		// pop rbx
		ctx.emit(&[0x5B]);
		ctx.pop(0x8);

		// pop rdx
		ctx.emit(&[0x5A]);
		ctx.pop(0x8);
	}

	fn modulo(&self, ctx: &mut byte_code::Context) {
		// push rdx
		ctx.emit(&[0x52]);
		ctx.push(0x8);

		// xor rdx, rdx
		ctx.emit(&[0x48, 0x31, 0xD2]);

		// push rbx
		ctx.emit(&[0x53]);
		ctx.push(0x8);

		// mov rax, rhs
		self.rhs.emit(ctx);

		// mov rbx, rax
		ctx.emit(&[0x48, 0x89, 0xC3]);

		// mov rax, lhs
		self.lhs.emit(ctx);

		// idiv rbx
		ctx.emit(&[0x48, 0xF7, 0xFB]);

		// pop rbx
		ctx.emit(&[0x5B]);
		ctx.pop(0x8);

		// mov rax, rdx
		ctx.emit(&[0x48, 0x89, 0xD0]);

		// pop rdx
		ctx.emit(&[0x5A]);
		ctx.pop(0x8);
	}

	fn equals(&self, ctx: &mut byte_code::Context) {
		// push rbx
		ctx.emit(&[0x53]);
		ctx.push(0x8);

		self.lhs.emit(ctx);

		// mov rbx, rax
		ctx.emit(&[0x48, 0x89, 0xC3]);

		self.rhs.emit(ctx);

		// cmp rax, rbx
		ctx.emit(&[0x48, 0x39, 0xD8]);

		// sete al
		ctx.emit(&[0x0F, 0x94, 0xC0]);

		// pop rbx
		ctx.emit(&[0x5B]);
		ctx.pop(0x8);
	}

	fn not_equals(&self, ctx: &mut byte_code::Context) {
		self.equals(ctx);

		// xor al, 1
		ctx.emit(&[0x34, 0x01]);
	}

	fn logical_and(&self, ctx: &mut byte_code::Context) {
		// mov rax, lhs
		self.lhs.emit(ctx);

		// test rax, rax
		ctx.emit(&[0x48, 0x85, 0xC0]);

		// push rbx
		ctx.emit(&[0x53]);
		ctx.push(0x8);

		// setne bl
		ctx.emit(&[0x0F, 0x95, 0xC3]);

		// mov rax, rhs
		self.rhs.emit(ctx);

		// test rax, rax
		ctx.emit(&[0x48, 0x85, 0xC0]);

		// setne al
		ctx.emit(&[0x0F, 0x95, 0xC0]);

		// and al, bl
		ctx.emit(&[0x20, 0xD8]);

		// pop rbx
		ctx.emit(&[0x5B]);
		ctx.pop(0x8);
	}

	fn logical_or(&self, ctx: &mut byte_code::Context) {
		// mov rax, lhs
		self.lhs.emit(ctx);

		// test rax, rax
		ctx.emit(&[0x48, 0x85, 0xC0]);

		// push rbx
		ctx.emit(&[0x53]);
		ctx.push(0x8);

		// setne bl
		ctx.emit(&[0x0F, 0x95, 0xC3]);

		// mov rax, rhs
		self.rhs.emit(ctx);

		// test rax, rax
		ctx.emit(&[0x48, 0x85, 0xC0]);

		// setne al
		ctx.emit(&[0x0F, 0x95, 0xC0]);

		// or al, bl
		ctx.emit(&[0x08, 0xD8]);

		// pop rbx
		ctx.emit(&[0x5B]);
		ctx.pop(0x8);
	}
}

impl ast::Node for BinaryOperation {
	fn emit(&self, ctx: &mut byte_code::Context) {
		match self.operator {
			Operators::Assignment => self.assignment(ctx),
			Operators::Addition => self.addition(ctx),
			Operators::Subtraction => self.subtraction(ctx),
			Operators::Multiplication => self.multiplication(ctx),
			Operators::Division => self.division(ctx),
			Operators::Modulo => self.modulo(ctx),
			Operators::Equals => self.equals(ctx),
			Operators::NotEquals => self.not_equals(ctx),
			Operators::LogicalAnd => self.logical_and(ctx),
			Operators::LogicalOr => self.logical_or(ctx),
		}
	}

	fn pre_type_check(&self, ctx: &mut tc::Context) {
		self.lhs.pre_type_check(ctx);
		self.rhs.pre_type_check(ctx);
	}

	fn type_check(&self, ctx: &mut tc::Context) -> Result<tc::Type> {
		let mut lhs = self.lhs.type_check(ctx)?;
		let rhs = self.rhs.type_check(ctx)?;

		if let Operators::Assignment = self.operator {
			let tc::Type::Reference(ptr_type) = lhs else {
				return Err(format!(""));
			};

			lhs = *ptr_type;
		}

		if lhs != rhs {
			return Err(format!(""));
		}

		if let Operators::Assignment = self.operator {
			return Ok(tc::Type::None);
		}

		return Ok(lhs);
	}
}
