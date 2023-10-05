use crate::asdf::Result;
use std::collections::VecDeque;

use super::{
	ast, expression_parser::ExpressionParser, location::Location, operators::Operators,
	parsing_context::ParsingContext, tc, tokens::Tokens,
};

pub struct Parser {
	tokens: VecDeque<(Tokens, Location)>,
	nodes: Vec<ast::BoxedNode>,
	location: Location,
}

impl Parser {
	pub fn new(tokens: VecDeque<(Tokens, Location)>) -> Self {
		return Self {
			tokens,
			nodes: vec![],
			location: Location::default(),
		};
	}

	pub fn parse(mut self, ctx: &mut ParsingContext) -> Result<Vec<ast::BoxedNode>> {
		while !self.tokens.is_empty() {
			if self.return_statement(ctx)? {
				continue;
			}

			if self.if_statement(ctx)? {
				continue;
			}

			if self.while_statement(ctx)? {
				continue;
			}

			if self.variable_declaration(ctx)? {
				continue;
			}

			if self.function_definition(ctx)? {
				continue;
			}

			if self.expression(ctx)? {
				continue;
			}

			return self
				.location
				.error(format!("cannot parse {:?}", self.tokens));
		}

		return Ok(self.nodes);
	}

	fn return_statement(&mut self, ctx: &mut ParsingContext) -> Result<bool> {
		let Some(Tokens::Return) = self.peek() else {
			return Ok(false);
		};

		self.pop();

		let tokens = self.pop_until(Tokens::SemiColon)?;
		let node = ExpressionParser::new(tokens).parse(ctx)?;

		self.pop();
		self.push(ast::ReturnStatement::new(node).into());

		return Ok(true);
	}

	fn if_statement(&mut self, ctx: &mut ParsingContext) -> Result<bool> {
		let Some(Tokens::If) = self.peek() else {
			return Ok(false);
		};

		self.pop();

		let tokens = self.pop_until(Tokens::CurlyOpen)?;
		let node = ExpressionParser::new(tokens).parse(ctx)?;

		let scope_tokens = self.pop_scope(Tokens::CurlyOpen, Tokens::CurlyClose)?;

		let nodes = Self::new(scope_tokens).parse(ctx)?;
		let scope = ast::Scope::new(nodes);

		self.push(ast::IfStatement::new(node, scope).into());
		return Ok(true);
	}

	fn while_statement(&mut self, ctx: &mut ParsingContext) -> Result<bool> {
		let Some(Tokens::While) = self.peek() else {
			return Ok(false);
		};

		self.pop();

		let tokens = self.pop_until(Tokens::CurlyOpen)?;
		let condition = ExpressionParser::new(tokens).parse(ctx)?;

		let scope_tokens = self.pop_scope(Tokens::CurlyOpen, Tokens::CurlyClose)?;

		let nodes = Self::new(scope_tokens).parse(ctx)?;
		let scope = ast::Scope::new(nodes);

		self.push(ast::WhileStatement::new(condition, scope).into());
		return Ok(true);
	}

	fn variable_declaration(&mut self, ctx: &mut ParsingContext) -> Result<bool> {
		let _mutable = match self.peek() {
			Some(Tokens::Imm) => false,
			Some(Tokens::Mut) => true,
			_ => return Ok(false),
		};

		self.pop();

		let name = match self.pop() {
			Some((Tokens::Identifier(name), _)) => name,
			x => {
				return self
					.location
					.error(format!("expected Some(Identifier), got {:?}", x))
			}
		};

		self.pop_checked(Tokens::Equals)?;

		let tokens = self.pop_until(Tokens::SemiColon)?;
		let value = ExpressionParser::new(tokens).parse(ctx)?;

		self.pop();

		// TODO: this is very temporary, type checking shouldn't be needed for getting the type
		let value_type = value.type_check(ctx.get_type_checking_context())?;

		let size = value_type.get_size() as u32;
		let offset = ctx.push_variable(value_type.clone(), name, size);

		self.push(
			ast::BinaryOperation::new(
				Operators::Assignment,
				ast::Stack::new(value_type, offset, size).into(),
				value,
			)
			.into(),
		);

		return Ok(true);
	}

	fn function_definition(&mut self, ctx: &mut ParsingContext) -> Result<bool> {
		let Some(Tokens::Fn) = self.peek() else {
			return Ok(false);
		};

		self.pop();

		let function_name = match self.pop() {
			Some((Tokens::Identifier(name), _)) => name,
			x => {
				return self
					.location
					.error(format!("expected Some(Identifier), got {:?}", x))
			}
		};

		self.pop_checked(Tokens::ParenOpen)?;

		let params = self.parse_function_params()?;
		let types = params.iter().map(|(_, typ)| typ.clone()).collect();

		self.pop_checked(Tokens::ParenClose)?;
		let tokens = self.pop_scope(Tokens::CurlyOpen, Tokens::CurlyClose)?;

		ctx.push();

		for (name, typ) in params.clone() {
			let size = typ.get_size() as u32;
			ctx.push_variable(typ, name, size);
		}

		let nodes = Self::new(tokens).parse(ctx)?;
		let stack_size = ctx.pop();

		self.push(
			ast::Function::new(
				function_name,
				tc::Type::I64,
				types,
				nodes,
				(stack_size + 0x10 - 1) & !(0x10 - 1),
			)
			.into(),
		);

		return Ok(true);
	}

	fn expression(&mut self, ctx: &mut ParsingContext) -> Result<bool> {
		let tokens = self.pop_until(Tokens::SemiColon)?;
		let expression = ExpressionParser::new(tokens).parse(ctx)?;

		self.pop();
		self.push(expression);

		return Ok(true);
	}

	fn peek(&self) -> Option<Tokens> {
		let Some((ret, _)) = self.tokens.front() else {
			return None;
		};

		return Some(ret.clone());
	}

	fn push(&mut self, node: ast::BoxedNode) {
		self.nodes.push(node);
	}

	fn pop(&mut self) -> Option<(Tokens, Location)> {
		let Some((tok, loc)) = self.tokens.pop_front() else {
			return None;
		};

		self.location = loc.clone();
		return Some((tok, loc));
	}

	fn pop_checked(&mut self, expected: Tokens) -> Result<()> {
		let Some((tok, _)) = self.pop() else {
			return self
				.location
				.error(format!("expected {:?}, got nothing", expected));
		};

		if tok != expected {
			return self
				.location
				.error(format!("expected {:?}, got {:?}", expected, tok));
		}

		return Ok(());
	}

	fn pop_while<T: FnMut(&Self, Option<&(Tokens, Location)>) -> Result<bool>>(
		&mut self,
		mut predicate: T,
	) -> Result<VecDeque<(Tokens, Location)>> {
		let mut ret = VecDeque::new();

		while predicate(self, self.tokens.front())? {
			ret.push_back(self.pop().unwrap());
		}

		return Ok(ret);
	}

	fn pop_until(&mut self, token_type: Tokens) -> Result<VecDeque<(Tokens, Location)>> {
		let ret = self.pop_while(|this, x| {
			let Some((typ, _)) = x else {
				return this
					.location
					.error(format!("expected {:?}, got nothing", token_type));
			};

			return Ok(*typ != token_type);
		})?;

		return Ok(ret);
	}

	fn pop_scope(&mut self, open: Tokens, close: Tokens) -> Result<VecDeque<(Tokens, Location)>> {
		let mut diff = 1;
		self.pop_checked(open.clone())?;

		let ret = self.pop_while(|this, x| {
			let Some((typ, _)) = x else {
				return this
					.location
					.error(format!("expected {:?}, got nothing", close));
			};

			if *typ == open {
				diff += 1;
			} else if *typ == close {
				diff -= 1;
			}

			return Ok(diff > 0);
		})?;

		self.pop();
		return Ok(ret);
	}

	fn parse_function_params(&mut self) -> Result<Vec<(String, tc::Type)>> {
		let mut ret = vec![];

		// TODO: clean this up
		while !self.tokens.is_empty() {
			if self.peek() == Some(Tokens::ParenClose) {
				break;
			}

			let ident = match self.pop() {
				Some((Tokens::Identifier(x), _)) => x,
				Some((tok, _)) => {
					return self
						.location
						.error(format!("expected Identifier, got {:?}", tok))
				}
				_ => {
					return self
						.location
						.error(format!("expected Identifier, got nothing"))
				}
			};

			self.pop_checked(Tokens::Colon)?;

			let typ = match self.pop() {
				Some((Tokens::I64, _)) => tc::Type::I64,
				Some((tok, _)) => {
					return self
						.location
						.error(format!("expected a type, got {:?}", tok))
				}
				_ => return self.location.error(format!("expected a type, got nothing")),
			};

			match self.peek() {
				Some(Tokens::ParenClose) => {}
				None => {}
				_ => self.pop_checked(Tokens::Comma)?,
			}

			ret.push((ident, typ));
		}

		// TODO: should this check for if ParenClose is never reached?
		Ok(ret)
	}
}
