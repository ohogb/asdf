#[derive(Debug, Clone, PartialEq)]
pub enum Type {
	None,
	I64,
	Char,
	Reference(Box<Self>),
	Pointer(Box<Self>),
	Function(Box<Self>, Vec<Self>),
}

impl Type {
	pub fn get_size(&self) -> usize {
		match *self {
			Type::I64 => 8,
			Type::Char => 1,
			Type::Reference(_) => 8,
			Type::Pointer(_) => 8,
			Type::Function(_, _) => 8,
			Type::None => unreachable!(),
		}
	}
}
