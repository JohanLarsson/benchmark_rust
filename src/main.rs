#![feature(test)]
extern crate test;

use test::Bencher;
#[bench]
fn filter_and_sum_bench(b: &mut Bencher) {
  b.iter(||{
    filter_and_sum(1000000, 1000)
  });
}

#[bench]
fn filter_divide_and_sum_bench(b: &mut Bencher) {
  b.iter(||{
	filter_divide_and_sum(1000000, 1000)
  });
}

fn filter_and_sum(n: i32, m: i32) -> i32{
	let sum = (0..n).filter(|x| x % m == 0)
					.fold(0, |sum, x| sum + x);
    return sum;
}

fn filter_divide_and_sum(n: i32, m: i32) -> f32 {
	let sum = (0..n).filter(|x| x % m == 0)
					.map(|x| x as f32/3.0)
					.fold(0.0, |sum, x| sum + x);
    return sum;
}
