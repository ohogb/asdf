mod binary_operation;
mod call_statement;
mod dereference;
mod extern_function;
mod extern_string;
mod function;
mod if_statement;
mod integer;
mod node;
mod relative;
mod return_statement;
mod scope;
mod stack;
mod while_statement;

pub use binary_operation::*;
pub use call_statement::*;
pub use dereference::*;
pub use extern_function::*;
pub use extern_string::*;
pub use function::*;
pub use if_statement::*;
pub use integer::*;
pub use node::*;
pub use relative::*;
pub use return_statement::*;
pub use scope::*;
pub use stack::*;
pub use while_statement::*;

use crate::asdf::{byte_code, tc, Result};

macro_rules! helper {
	($name:ident, $($types:ident),*,) => {

		#[derive(Debug)]
		pub enum $name {
			$(
				$types($types),
			)*
		}

		impl $name {

			pub fn emit(&self, ctx: &mut byte_code::Context) {
				match self {
					$(
						Self::$types(x) => x.emit(ctx),
					)*
				}
			}

			pub fn pre_type_check(&self, ctx: &mut tc::Context) {
				match self {
					$(
						Self::$types(x) => x.pre_type_check(ctx),
					)*
				}
			}

			pub fn type_check(&self, ctx: &mut tc::Context) -> Result<tc::Type> {
				return match self {
					$(
						Self::$types(x) => x.type_check(ctx),
					)*
				};
			}

			pub fn is_reference(&self) -> bool {
				return match self {
					$(
						Self::$types(x) => x.is_reference(),
					)*
				};
			}
		}

		$(
			impl From<$types> for $name {
				fn from(x: $types) -> Self {
					return Self::$types(x);
				}
			}
		)*
	}
}

helper!(
	BoxedNode,
	BinaryOperation,
	CallStatement,
	Dereference,
	ExternFunction,
	ExternString,
	Function,
	IfStatement,
	Integer,
	Relative,
	ReturnStatement,
	Scope,
	Stack,
	WhileStatement,
);
