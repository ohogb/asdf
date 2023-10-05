use crate::asdf::Result;

#[derive(Debug, Default, Clone)]
pub struct Location {
	file_name: String,
	line_number: u64,
}

impl Location {
	pub fn new<T: Into<String>>(file_name: T, line_number: u64) -> Self {
		return Self {
			file_name: file_name.into(),
			line_number,
		};
	}

	pub fn error<T, U: Into<String>>(&self, message: U) -> Result<T> {
		return Err(format!(
			"{}:{}: {}",
			self.file_name,
			self.line_number,
			message.into()
		));
	}
}
