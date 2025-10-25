use std::io;

fn main() {
	let mut sum = 0;

	loop {
		let mut input = String::new();
		io::stdin()
			.read_line(&mut input)
			.expect("Failed input");
		let input = input.trim();

		if input == "-1" {
			break;
		}

		match input.parse::<i32>() {
			Ok(number) => {
				if number >= 0 {
					sum += number;
				} else {
					println!("NaN");
					return;
				}
			},
			Err(_) => {
				println!("NaN");
				return;
			},
		}
	}

	println!("{}", sum);
}
