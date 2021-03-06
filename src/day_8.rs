// File: day_8.rs
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

pub struct Challenge {
	chars: Vec<Vec<char>>,
}
impl ChallengeT for Challenge {
	type Output1 = usize;
	type Output2 = usize;

	fn day() -> u8 {
		8
	}
	fn new() -> Self {
		let input = include_str!("../inputs/day_8.txt");
		Challenge {
			chars: input
				.lines()
				.map(|line| line.chars().collect::<Vec<char>>())
				.collect(),
		}
	}
	fn part_1(&self) -> Self::Output1 {
		self.chars
			.iter()
			.map(|chars| {
				let mut difference = 2;
				let mut current = 1;
				while current < chars.len() - 1 {
					match (chars[current], chars[current + 1]) {
						('\\', '\"') | ('\\', '\\') => {
							current += 2;
							difference += 1;
						}
						('\\', 'x') => {
							difference += 3;
							current += 4;
						}
						_ => current += 1,
					}
				}
				difference
			})
			.sum()
	}
	fn part_2(&self) -> Self::Output2 {
		self.chars
			.iter()
			.map(|chars| {
				let mut differece = 4;
				let mut current = 1;
				while current < chars.len() - 1 {
					match chars[current] {
						'\"' => differece += 1,
						'\\' => differece += 1,
						_ => (),
					}
					current += 1;
				}
				differece
			})
			.sum()
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::common::ChallengeT;

	#[test]
	fn part_1_test() {
		let res = Challenge::new().part_1();
		assert_eq!(res, 1371);
	}
	#[test]
	fn part_2_test() {
		let res = Challenge::new().part_2();
		assert_eq!(res, 2117);
	}

	use test::Bencher;
	#[bench]
	fn part_1_bench(b: &mut Bencher) {
		b.iter(|| Challenge::new().part_1())
	}
	#[bench]
	fn part_2_bench(b: &mut Bencher) {
		b.iter(|| Challenge::new().part_2())
	}
	#[bench]
	fn both_bench(b: &mut Bencher) {
		b.iter(|| {
			let challenge = Challenge::new();
			challenge.part_1();
			challenge.part_2();
		})
	}
}
