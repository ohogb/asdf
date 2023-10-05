use crate::asdf::{
	ast::{self, Node},
	byte_code,
	lexer::Lexer,
	parser::Parser,
	parsing_context::ParsingContext,
	tc, Result,
};

pub struct Instance {
	instructions: Vec<byte_code::Instruction>,
	main_offset: usize,
}

impl Instance {
	pub fn new() -> Self {
		return Self {
			instructions: vec![],
			main_offset: usize::default(),
		};
	}

	pub fn parse(&mut self, string: &str) -> Result<()> {
		assert!(self.instructions.is_empty());

		let tokens = Lexer::new(string.into(), "file_name.ext".into()).lex()?;

		let mut tcc = tc::Context::new();
		let mut ctx = ParsingContext::new(&mut tcc);

		extern "C" fn print(x: *const u8) {
			let str = unsafe { std::ffi::CStr::from_ptr(x as *const i8) }
				.to_str()
				.unwrap();

			print!("{}", str);
		}

		extern "C" fn to_string(x: i64) -> *const u8 {
			let ptr = Box::leak(Box::new(x.to_string() + "\x00")).as_ptr();
			return ptr;
		}

		#[naked]
		extern "C" fn bp() {
			unsafe {
				std::arch::asm!("int3", "ret", options(noreturn));
			}
		}

		ctx.extern_function(
			"print",
			print as u64,
			tc::Type::None,
			vec![tc::Type::Pointer(Box::new(tc::Type::Char))],
		);

		ctx.extern_function(
			"to_string",
			to_string as u64,
			tc::Type::Pointer(Box::new(tc::Type::Char)),
			vec![tc::Type::I64],
		);

		ctx.extern_function("bp", bp as u64, tc::Type::None, vec![]);

		let nodes = Parser::new(tokens.into()).parse(&mut ctx)?;
		let global_scope = ast::Scope::new(nodes);

		global_scope.pre_type_check(&mut tcc);
		global_scope.type_check(&mut tcc)?;

		let mut ctx = byte_code::Context::new();
		global_scope.emit(&mut ctx);

		assert!(ctx.get_stack_size() == 0);
		self.instructions = ctx.instructions();

		let Some(main_offset) = ctx.get_function_offset("main") else {
			return Err(format!("cannot find 'main' function"));
		};

		self.main_offset = main_offset;
		return Ok(());
	}

	pub fn execute(&self) -> i64 {
		assert!(!self.instructions.is_empty());
		let mut ins = vec![];

		for i in &self.instructions {
			ins.extend(i.get_instruction());
		}

		let size = ins.len();
		let ptr = Self::mmap(size);

		let ret = unsafe {
			std::ptr::copy(ins.as_ptr(), ptr, size);

			let function: unsafe extern "C" fn() -> i64 =
				std::mem::transmute(ptr.add(self.main_offset));

			function()
		};

		Self::munmap(ptr, size);
		return ret;
	}

	extern "C" fn mmap(size: usize) -> *mut u8 {
		let mut ret;

		unsafe {
			std::arch::asm!(
				"syscall",
				inlateout("rax") 9u64 => ret,
				inout("rdi") 0 => _,
				inout("rsi") size => _,
				inout("rdx") 7 => _,
				inout("r10") 0x22 => _,
				inout("r8") 0 => _,
				inout("r9") 0 => _,
				out("rcx") _,
				out("r11") _,
			);
		}

		return ret;
	}

	extern "C" fn munmap(ptr: *const u8, size: usize) {
		unsafe {
			std::arch::asm!(
				"syscall",
				inlateout("rax") 11u64 => _,
				inout("rdi") ptr => _,
				inout("rsi") size => _,
				out("rdx") _,
				out("r10") _,
				out("r8") _,
				out("r9") _,
				out("rcx") _,
				out("r11") _,
			);
		}
	}
}
