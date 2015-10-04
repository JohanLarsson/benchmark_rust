extern crate stopwatch;
use stopwatch::{Stopwatch};

fn main() {
	let sw = Stopwatch::start_new();
	let sum_evens = (0..1000000)
					.filter(|x| x % 1000 == 0)
					.fold(0, |sum, x| sum + x);
	println!("{0}: {1} ms", sum_evens, sw.elapsed_ms());
}
