extern crate stopwatch;
use stopwatch::{Stopwatch};

fn main() {
	filter_and_sum(1000000, 1000);
	filter_divide_and_sum(1000000, 1000);
}

fn filter_and_sum(n: i32, m: i32){
	let sw = Stopwatch::start_new();
	let sum = (0..n).filter(|x| x % m == 0)
					.fold(0, |sum, x| sum + x);
	println!("{0}: {1} ms", sum, sw.elapsed_ms());
}

fn filter_divide_and_sum(n: i32, m: i32){
	let sw = Stopwatch::start_new();
	let sum = (0..n).filter(|x| x % m == 0)
					.map(|x| x as f32/3.0)
					.fold(0.0, |sum, x| sum + x);
	println!("{0}: {1} ms", sum, sw.elapsed_ms());
}
