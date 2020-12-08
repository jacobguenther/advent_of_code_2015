// File: day_10.rs
// Author: Jacob Guenther
// Date: December 2020

/*
Copyright 2020 Jacob Guenther

Permission is hereby granted, free of charge, to any person obtaining a copy of
this software and associated documentation files (the "Software"), to deal in
the Software without restriction, including without limitation the rights to
use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software is furnished to do so,
subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER
IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
*/

use super::common::*;
use std::char::from_digit;

pub struct Challenge {
	part_1_partial: Vec<char>,
}
impl ChallengeT for Challenge {
	type Output1 = usize;
	type Output2 = usize;

	fn day() -> u8 {
		10
	}
	fn new() -> Self {
		let input = "1113122113";
		Challenge {
			part_1_partial: solve(&input.chars().collect(), 40),
		}
	}
	fn part_1(&self) -> Self::Output1 {
		self.part_1_partial.len()
	}
	fn part_2(&self) -> Self::Output2 {
		solve(&self.part_1_partial, 10).len()
	}
}
fn solve(input: &Vec<char>, iterations: usize) -> Vec<char> {
	let mut result = input.clone();
	(0..iterations).for_each(|_| {
		let len = result.len();
		let mut temp: Vec<char> = Vec::with_capacity(len);

		let mut i = 0;
		while i < len {
			let mut count = 1;
			while i + 1 < len && result[i] == result[i + 1] {
				i += 1;
				count += 1;
			}
			temp.push(from_digit(count, 10).unwrap());
			temp.push(result[i]);
			i += 1;
		}

		result = temp;
	});
	result
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::common::ChallengeT;

	#[test]
	fn part_1_test() {
		let res = Challenge::new().part_1();
		assert_eq!(res, 360154);
	}
	#[test]
	fn part_2_test() {
		let res = Challenge::new().part_2();
		assert_eq!(res, 5103798);
	}

	use test::Bencher;
	#[bench]
	fn bench_simple(b: &mut Bencher) {
		b.iter(|| {
			solve(&"1113122113".chars().collect(), 20);
		})
	}
}
