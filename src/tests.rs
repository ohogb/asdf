use crate::asdf::{instance::Instance, Result};

// parse and run
fn par(string: &str) -> Result<i64> {
	let mut instance = Instance::new();
	instance.parse(string)?;

	return Ok(instance.execute());
}

#[test]
fn binary_op() -> Result<()> {
	assert_eq!(
		par(r#"

		fn main() {
			return 1 + 2 * 4 + 5 * (6 - 7) * 8 + (9 + 10) * 11 + (12 % 13) * 14;
		}

	"#)?,
		346
	);

	Ok(())
}

#[test]
fn if_statement() -> Result<()> {
	assert_eq!(
		par(r#"

		fn main() {

			mut ret = 0;

			if 1 {
				ret = ret + 5;
			}

			if 0 {
				ret = ret + 4;
			}

			return ret;
		}

	"#)?,
		5
	);

	Ok(())
}

#[test]
fn while_statement() -> Result<()> {
	assert_eq!(
		par(r#"

		fn main() {

			mut i = 0;
			mut ret = 0;

			while i != 100 {
				ret = ret + 5;
				i = i + 1;
			}

			return ret;
		}

	"#)?,
		500
	);

	Ok(())
}

#[test]
fn function_call() -> Result<()> {
	assert_eq!(
		par(r#"

		fn main() {
			return function();
		}

		fn function() {
			return 123;
		}

	"#)?,
		123
	);

	assert_eq!(
		par(r#"

		fn function() {
			return 123;
		}

		fn main() {
			return function();
		}

	"#)?,
		123
	);

	Ok(())
}

#[test]
fn logical_and() -> Result<()> {
	assert_eq!(
		par(r#"

		fn main() {
			return 1 && 2;
		}

	"#)? != 0,
		true
	);

	assert_eq!(
		par(r#"

		fn main() {
			return 0 && 2;
		}

	"#)? != 0,
		false
	);

	assert_eq!(
		par(r#"

		fn main() {
			return 1 && 0;
		}

	"#)? != 0,
		false
	);

	assert_eq!(
		par(r#"

		fn main() {
			return 0 && 0;
		}

	"#)? != 0,
		false
	);

	Ok(())
}

#[test]
fn logical_or() -> Result<()> {
	assert_eq!(
		par(r#"

		fn main() {
			return 1 || 2;
		}

	"#)? != 0,
		true
	);

	assert_eq!(
		par(r#"

		fn main() {
			return 0 || 2;
		}

	"#)? != 0,
		true
	);

	assert_eq!(
		par(r#"

		fn main() {
			return 1 || 0;
		}

	"#)? != 0,
		true
	);

	assert_eq!(
		par(r#"

		fn main() {
			return 0 || 0;
		}

	"#)? != 0,
		false
	);

	Ok(())
}

#[test]
fn function_params() -> Result<()> {
	assert_eq!(
		par(r#"

		fn main() {
			return sum(4, 5);
		}

		fn sum(x: i64, y: i64) {
			return x + y;
		}

	"#)?,
		9
	);

	Ok(())
}
