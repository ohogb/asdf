use crate::asdf::{
	ast, location::Location, operators::Operators, parsing_context::ParsingContext, tokens::Tokens,
	Result,
};

use std::collections::VecDeque;

pub struct ExpressionParser {
	tokens: VecDeque<(Tokens, Location)>,
}

impl ExpressionParser {
	pub fn new(tokens: VecDeque<(Tokens, Location)>) -> Self {
		Self { tokens }
	}

	pub fn parse(mut self, ctx: &mut ParsingContext) -> Result<ast::BoxedNode> {
		// TODO: errors with location

		assert!(!self.tokens.is_empty());

		let Some(mut lhs) = self.parse_value(ctx, 0)? else {
			return Err(format!("cannot expression parse: {:?}", self.tokens));
		};

		while !self.tokens.is_empty() {
			if let Some(op) = self.parse_binary_operator(0) {
				lhs = self.parse_binary_operation(ctx, op, lhs)?;
			} else {
				return Err(format!("cannot expression parse: {:?}", self.tokens));
			}
		}

		Ok(Self::ensure_value(lhs))
	}

	fn parse_value(
		&mut self,
		ctx: &mut ParsingContext,
		current_precedence: u8,
	) -> Result<Option<ast::BoxedNode>> {
		let value = self
			.parse_scope(ctx)?
			.or_else(|| self.parse_identifier(ctx))
			.or_else(|| self.parse_integer())
			.or_else(|| self.parse_string());

		let Some(mut value) = value else {
			return Ok(None);
		};

		value = self.parse_function_call(ctx, value)?;

		if let Some(op) = self.parse_binary_operator(current_precedence) {
			Ok(Some(self.parse_binary_operation(ctx, op, value)?))
		} else {
			Ok(Some(value))
		}
	}

	fn parse_binary_operator(&mut self, current_precedence: u8) -> Option<Operators> {
		let (op, amount) = match self.peek() {
			Some(Tokens::Plus) => (Operators::Addition, 1),
			Some(Tokens::Minus) => (Operators::Subtraction, 1),
			Some(Tokens::Star) => (Operators::Multiplication, 1),
			Some(Tokens::Slash) => (Operators::Division, 1),
			Some(Tokens::Percent) => (Operators::Modulo, 1),
			Some(Tokens::Equals) => match self.peek_nth(1) {
				Some(Tokens::Equals) => (Operators::Equals, 2),
				_ => (Operators::Assignment, 1),
			},
			Some(Tokens::ExclamationMark) => match self.peek_nth(1) {
				Some(Tokens::Equals) => (Operators::NotEquals, 2),
				_ => return None,
			},
			Some(Tokens::And) => match self.peek_nth(1) {
				Some(Tokens::And) => (Operators::LogicalAnd, 2),
				_ => return None,
			},
			Some(Tokens::Pipe) => match self.peek_nth(1) {
				Some(Tokens::Pipe) => (Operators::LogicalOr, 2),
				_ => return None,
			},
			_ => return None,
		};

		if op.get_precedence() < current_precedence {
			return None;
		}

		for _ in 0..amount {
			self.pop();
		}

		Some(op)
	}

	fn parse_binary_operation(
		&mut self,
		ctx: &mut ParsingContext,
		op: Operators,
		lhs: ast::BoxedNode,
	) -> Result<ast::BoxedNode> {
		let Some(rhs) = self.parse_value(ctx, op.get_precedence())? else {
			return Err(format!("expected a value for '{op:?}', found nothing"));
		};

		let lhs = match op {
			Operators::Assignment => lhs,
			_ => Self::ensure_value(lhs),
		};

		Ok(ast::BinaryOperation::new(op, lhs, Self::ensure_value(rhs)).into())
	}

	fn parse_arguments(
		ctx: &mut ParsingContext,
		tokens: Vec<(Tokens, Location)>,
	) -> Result<Vec<ast::BoxedNode>> {
		if tokens.is_empty() {
			return Ok(vec![]);
		}

		let mut ret = vec![];

		let mut current_tokens = vec![];
		let mut diff = 0;

		for i in tokens {
			let typ = i.0.clone();

			if typ == Tokens::ParenOpen {
				diff += 1;
			} else if typ == Tokens::ParenClose {
				assert!(diff > 0);
				diff -= 1;
			} else if typ == Tokens::Comma && diff == 0 {
				ret.push(Self::new(current_tokens.into()).parse(ctx)?);
				current_tokens = Vec::new();

				continue;
			}

			current_tokens.push(i);
		}

		assert!(diff == 0);

		if current_tokens.is_empty() {
			return Err(format!("expected a value, found nothing"));
		}

		ret.push(Self::new(current_tokens.into()).parse(ctx)?);
		Ok(ret)
	}

	fn parse_function_call(
		&mut self,
		ctx: &mut ParsingContext,
		node: ast::BoxedNode,
	) -> Result<ast::BoxedNode> {
		let Some(Tokens::ParenOpen) = self.peek() else {
			return Ok(node);
		};

		let argument_tokens = self.pop_scope(Tokens::ParenOpen, Tokens::ParenClose)?;
		let args = Self::parse_arguments(ctx, argument_tokens)?;

		Ok(ast::CallStatement::new(node, args).into())
	}

	fn parse_scope(&mut self, ctx: &mut ParsingContext) -> Result<Option<ast::BoxedNode>> {
		let Some(Tokens::ParenOpen) = self.peek() else {
			return Ok(None);
		};

		let scope_tokens = self.pop_scope(Tokens::ParenOpen, Tokens::ParenClose)?;
		let ret = Self::new(scope_tokens.into()).parse(ctx)?;

		Ok(Some(ret))
	}

	fn parse_identifier(&mut self, ctx: &mut ParsingContext) -> Option<ast::BoxedNode> {
		let Some(Tokens::Identifier(_)) = self.peek() else {
			return None;
		};

		let Some((Tokens::Identifier(name), _)) = self.pop() else {
			unreachable!();
		};

		if let Some((function, return_type, argument_types)) = ctx.find_extern_function(&name) {
			Some(ast::ExternFunction::new(function, return_type, argument_types).into())
		} else if let Some((value_type, offset, size)) = ctx.find_variable(&name) {
			Some(ast::Stack::new(value_type, offset, size).into())
		} else {
			Some(ast::Relative::new(name).into())
		}
	}

	fn parse_integer(&mut self) -> Option<ast::BoxedNode> {
		let Some(Tokens::Integer(_)) = self.peek() else {
			return None;
		};

		let Some((Tokens::Integer(value), _)) = self.pop() else {
			unreachable!();
		};

		Some(ast::Integer::new(value).into())
	}

	fn parse_string(&mut self) -> Option<ast::BoxedNode> {
		let Some(Tokens::String(_)) = self.peek() else {
			return None;
		};

		let Some((Tokens::String(value), _)) = self.pop() else {
			unreachable!();
		};

		Some(ast::ExternString::new(value).into())
	}

	fn peek(&self) -> Option<Tokens> {
		let Some((ret, _)) = self.tokens.front() else {
			return None;
		};

		Some(ret.clone())
	}

	fn peek_nth(&self, index: usize) -> Option<Tokens> {
		let Some((ret, _)) = self.tokens.get(index) else {
			return None;
		};

		Some(ret.clone())
	}

	fn pop(&mut self) -> Option<(Tokens, Location)> {
		self.tokens.pop_front()
	}

	fn pop_while(
		&mut self,
		mut predicate: impl FnMut(Option<Tokens>) -> Result<bool>,
	) -> Result<Vec<(Tokens, Location)>> {
		let mut ret = vec![];

		while predicate(self.peek())? {
			ret.push(self.pop().unwrap());
		}

		Ok(ret)
	}

	fn pop_scope(&mut self, open: Tokens, close: Tokens) -> Result<Vec<(Tokens, Location)>> {
		assert!(self.peek() == Some(open.clone()));
		self.pop();

		let mut diff = 1;
		let ret = self.pop_while(|x| {
			let Some(x) = x else {
				return Err(format!("expected '{close:?}', got nothing"));
			};

			if x == open {
				diff += 1;
			} else if x == close {
				diff -= 1;
			}

			Ok(diff > 0)
		})?;

		let last = self.pop();
		assert!(last.unwrap().0 == close);

		Ok(ret)
	}

	fn ensure_value(x: ast::BoxedNode) -> ast::BoxedNode {
		if x.is_reference() {
			ast::Dereference::new(x).into()
		} else {
			x
		}
	}
}
