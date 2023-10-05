#[derive(Debug, Clone)]
pub enum Operators {
	Assignment,
	Addition,
	Subtraction,
	Multiplication,
	Division,
	Modulo,
	Equals,
	NotEquals,
	LogicalAnd,
	LogicalOr,
}

impl Operators {
	pub fn get_precedence(&self) -> u8 {
		return match *self {
			Self::Assignment => 0,
			Self::Addition => 3,
			Self::Subtraction => 3,
			Self::Multiplication => 4,
			Self::Division => 4,
			Self::Modulo => 3,
			Self::Equals => 2,
			Self::NotEquals => 2,
			Self::LogicalAnd => 1,
			Self::LogicalOr => 1,
		};
	}
}
