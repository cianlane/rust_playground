use std::env;

fn help() {
	println!("usage:
fibonacci <u32>
	Prints the nth value of the fibonnaci sequence.");
}

fn fibonacci(n: u32) -> u32 {
	if n == 0 {
		0
	}
	else if n == 1 {
		1
	}
	else {
		fibonacci(n - 1) + fibonacci(n - 2)
	}
}

fn calculate_fibonacci(n: u32) {
	println!("Calculating the {}th fibonacci value....", n);
	let res = fibonacci(n);
	println!("The {}th fibonacci value is {}", n, res);
}

fn main() {
	let args: Vec<String> = env::args().collect();

	match args.len() {
		2 => {
			match args[1].parse::<u32>() {
				Ok(n) => calculate_fibonacci(n),
				_ => help(),
			}
		},
		_ => {
			help();
		}
	}
}
