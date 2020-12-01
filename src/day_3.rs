// File: day_3.rs
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
use vec2::Vec2;
use std::collections::HashSet;

pub struct Challenge {
	input: &'static str,
}
impl ChallengeT for Challenge {
	type Output1 = usize;
	type Output2 = usize;

	fn day() -> u8 {
		3
	}
	fn new() -> Self {
		Challenge {
			input: include_str!("../inputs/day_3.txt"),
		}
	}
	fn part_1(&self) -> Self::Output1 {
		let mut houses_visited = HashSet::<vec2::Vec2<i32>>::new();
		let mut position = vec2::Vec2::new(0, 0);
		houses_visited.insert(position);
		self.input.chars()
			.for_each(|c| {
				move_santa(c, &mut position);
				houses_visited.insert(position);
			});
		houses_visited.len()
	}
	fn part_2(&self) -> Self::Output2 {
		let mut houses_visited = HashSet::<Vec2<i32>>::new();
		let mut position_san = Vec2::new(0, 0);
		let mut position_rob = Vec2::new(0, 0);
		houses_visited.insert(position_san);
		self.input.chars()
			.enumerate()
			.for_each(|(i, c)| {
				if i%2 == 0 {
					move_santa(c, &mut position_san);
					houses_visited.insert(position_san);
				} else {
					move_santa(c, &mut position_rob);
					houses_visited.insert(position_rob);
				}
			});
		houses_visited.len()
	}
}
#[inline(always)]
fn move_santa(dir: char, position: &mut Vec2<i32>) {
	match dir {
		'<' => position.x -= 1,
		'>' => position.x += 1,
		'^' => position.y += 1,
		'v' => position.y -= 1,
		_ => (),
	}
}

#[cfg(test)]
mod tests {
	use crate::common::ChallengeT;
	use super::Challenge;

	#[test]
	fn part_1() {
		let res = Challenge::new().part_1();
		assert_eq!(res, 2592);
	}
	#[test]
	fn part_2() {
		let res = Challenge::new().part_2();
		assert_eq!(res, 2360);
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
