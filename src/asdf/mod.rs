pub mod instance;

mod ast;
mod byte_code;
mod expression_parser;
mod lexer;
mod location;
mod operators;
mod parser;
mod parsing_context;
mod tc;
mod tokens;

pub type Result<T> = core::result::Result<T, String>;
