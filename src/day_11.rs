// File: day_11.rs
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
	part_1_result: Vec<u8>,
}
impl ChallengeT for Challenge {
	type Output1 = String;
	type Output2 = String;

	fn day() -> u8 {
		11
	}
	fn new() -> Self {
		let mut input = "vzbxkghb".as_bytes().iter().map(|b| *b).collect::<Vec<_>>();

		while !is_valid(&input) {
			increment_password(&mut input);
		}

		Challenge {
			part_1_result: input.clone(),
		}
	}
	fn part_1(&self) -> Self::Output1 {
		std::str::from_utf8(&self.part_1_result).unwrap().to_owned()
	}
	fn part_2(&self) -> Self::Output2 {
		let mut input = self.part_1_result.clone();
		increment_password(&mut input);
		while !is_valid(&input) {
			increment_password(&mut input);
		}
		std::str::from_utf8(&input).unwrap().to_owned()
	}
}
fn increment_password(password: &mut Vec<u8>) {
	for c in password.iter_mut().rev() {
		if *c == 'z' as u8 {
			*c = 'a' as u8;
		} else {
			*c += 1;
			break;
		}
	}
}
fn is_valid(password: &Vec<u8>) -> bool {
	let mut no_i_o_or_l = true;
	let mut has_increasing = false;
	let mut last_was_pair = false;
	let mut pairs_count = 0;

	for (current_i, (current, next)) in password.iter().zip(&password[1..]).enumerate() {
		if *current == 'i' as u8 || *current == 'o' as u8 || *current == 'l' as u8 {
			no_i_o_or_l = false;
			break;
		}
		if !has_increasing {
			if let Some(next_next) = password.get(current_i + 2) {
				if *current == next - 1 && *current == next_next - 2 {
					has_increasing = true;
				}
			}
		}
		if !last_was_pair && *current == *next {
			pairs_count += 1;
			last_was_pair = true;
		} else {
			last_was_pair = false;
		}
	}
	has_increasing && no_i_o_or_l && pairs_count >= 2
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::common::ChallengeT;

	#[test]
	fn part_1_test() {
		let res = Challenge::new().part_1();
		assert_eq!(res, "vzbxxyzz");
	}
	#[test]
	fn part_2_test() {
		let res = Challenge::new().part_2();
		assert_eq!(res, "vzcaabcc");
	}

	use test::Bencher;
	#[bench]
	fn bench_simple(b: &mut Bencher) {
		b.iter(|| {
			let ch = Challenge::new();
			ch.part_1();
			ch.part_2();
		})
	}
}
