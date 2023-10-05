use crate::asdf::Result;

use super::{location::Location, tokens::Tokens};

pub struct Lexer {
	string: String,
	file_name: String,
	line_number: u64,
	tokens: Vec<(Tokens, Location)>,
}

impl Lexer {
	pub fn new(string: String, file_name: String) -> Self {
		return Self {
			string,
			file_name,
			line_number: 1,
			tokens: vec![],
		};
	}

	pub fn lex(mut self) -> Result<Vec<(Tokens, Location)>> {
		loop {
			self.remove_white_space()?;

			if self.string.is_empty() {
				break;
			}

			if self.key_words() {
				continue;
			}

			if self.symbols() {
				continue;
			}

			if self.numbers()? {
				continue;
			}

			if self.strings()? {
				continue;
			}

			if self.identifiers()? {
				continue;
			}

			Location::new(self.file_name.clone(), self.line_number)
				.error(format!("cannot tokenize: {}", self.string))?;
		}

		return Ok(self.tokens);
	}

	fn remove_white_space(&mut self) -> Result<()> {
		let chars = self.pop_while(|x, _| {
			let Some(c) = x else {
				return Ok(false);
			};

			return Ok(c.is_whitespace());
		})?;

		for c in chars.chars() {
			if c == '\n' {
				self.line_number += 1;
			}
		}

		return Ok(());
	}

	fn key_words(&mut self) -> bool {
		let (token, size) = match self.peek_word() {
			"if" => (Tokens::If, 2),
			"else" => (Tokens::Else, 4),
			"return" => (Tokens::Return, 6),
			"mut" => (Tokens::Mut, 3),
			"imm" => (Tokens::Imm, 3),
			"while" => (Tokens::While, 5),
			"fn" => (Tokens::Fn, 2),
			"i64" => (Tokens::I64, 3),
			_ => return false,
		};

		for _ in 0..size {
			self.pop();
		}

		self.push(token);
		return true;
	}

	fn symbols(&mut self) -> bool {
		let token = match self.peek().unwrap() {
			';' => Tokens::SemiColon,
			':' => Tokens::Colon,
			',' => Tokens::Comma,
			'+' => Tokens::Plus,
			'-' => Tokens::Minus,
			'*' => Tokens::Star,
			'/' => Tokens::Slash,
			'=' => Tokens::Equals,
			'!' => Tokens::ExclamationMark,
			'%' => Tokens::Percent,
			'&' => Tokens::And,
			'|' => Tokens::Pipe,
			'(' => Tokens::ParenOpen,
			')' => Tokens::ParenClose,
			'{' => Tokens::CurlyOpen,
			'}' => Tokens::CurlyClose,
			_ => return false,
		};

		self.pop();
		self.push(token);

		return true;
	}

	fn numbers(&mut self) -> Result<bool> {
		let value = self.pop_while(|x, _| {
			let Some(c) = x else {
				return Ok(false);
			};

			return Ok(c.is_ascii_digit());
		})?;

		if value.is_empty() {
			return Ok(false);
		}

		self.push(Tokens::Integer(value.parse().unwrap()));
		return Ok(true);
	}

	fn strings(&mut self) -> Result<bool> {
		match self.peek() {
			Some('"') => {}
			_ => return Ok(false),
		}

		self.pop();

		let value = self.pop_while(|x, _| {
			let Some(c) = x else {
				return Err(format!("expected '\"', got nothing"));
			};

			return Ok(c != '"');
		})?;

		// TODO: actual escaped characters
		let value = value.replace("\\n", "\n");

		self.pop();
		self.push(Tokens::String(value));

		return Ok(true);
	}

	fn identifiers(&mut self) -> Result<bool> {
		let value = self.pop_while(|x, is_first| {
			let Some(c) = x else {
				return Ok(false);
			};

			if c.is_ascii_alphabetic() {
				return Ok(true);
			}

			if !is_first && c.is_ascii_digit() {
				return Ok(true);
			}

			if c == '_' {
				return Ok(true);
			}

			return Ok(false);
		})?;

		if value.is_empty() {
			return Ok(false);
		}

		self.push(Tokens::Identifier(value));
		return Ok(true);
	}

	fn peek(&self) -> Option<char> {
		return self.string.chars().next();
	}

	fn peek_word(&self) -> &str {
		let mut size = usize::default();

		for c in self.string.chars() {
			// TODO: is this good?
			if c.is_ascii_alphanumeric() || c == '_' {
				size += 1;
				continue;
			}

			break;
		}

		return &self.string[0..size];
	}

	fn pop(&mut self) -> char {
		return self.string.remove(0);
	}

	fn pop_while<T: Fn(Option<char>, bool) -> Result<bool>>(
		&mut self,
		predicate: T,
	) -> Result<String> {
		let mut ret = String::new();

		while predicate(self.string.chars().next(), ret.is_empty())? {
			ret.push(self.pop());
		}

		return Ok(ret);
	}

	fn push(&mut self, token: Tokens) {
		self.tokens.push((
			token,
			Location::new(self.file_name.clone(), self.line_number),
		));
	}
}
