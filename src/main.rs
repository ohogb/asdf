#![feature(naked_functions)]

mod asdf;

#[cfg(test)]
mod tests;

use asdf::instance::Instance;

fn main() -> Result<(), Box<dyn std::error::Error>> {
	let string = r#"

		fn main() {
			fizz_buzz();
			return zxcv();
		}

		fn zxcv() {
			print("hello world!\n");
			return 5;
		}

		fn fizz_buzz() {

			mut i = 1;

			while i != 100 {

				mut first = (i % 3) == 0;
				mut second = (i % 5) == 0;

				if first {
					print("fizz");
				}

				if second {
					print("buzz");
				}

				if first == 0 && second == 0 {
					print(to_string(i));
				}

				print("\n");
				i = i + 1;
			}

			return 0;
		}
	"#;

	let mut instance = Instance::new();
	instance.parse(string)?;

	let ret = instance.execute();
	print!("ret: {}\n", ret);

	return Ok(());
}
