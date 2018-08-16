use std::iter::{once, repeat};

fn main() {
	let fizzes = repeat("").take(2).chain(once("fizz")).cycle();
	let buzzes = repeat("").take(4).chain(once("buzz")).cycle();
	let fizz_buzzes = fizzes.zip(buzzes);

	let fizz_buzz = (1..101).zip(fizz_buzzes)
		.map(|tup| {
			match tup {
				(i, ("", "")) => format!("{}:", i),
				(i, (fizz, buzz)) => format!("{}: {}{}", i, fizz, buzz),
			}
		});

	for fb in fizz_buzz {
		println!("{}", fb);
	}

}
