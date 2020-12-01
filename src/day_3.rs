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
use std::collections::HashSet;

pub struct Challenge {}
impl ChallengeT for Challenge {
	type Output1 = usize;
	type Output2 = usize;

	fn day() -> u8 {
		3
	}
	fn part_1() -> Self::Output1 {
		let mut houses_visited = HashSet::<vec2::Vec2<i32>>::new();
		let mut position = vec2::Vec2::new(0, 0);
		houses_visited.insert(position);
		include_str!("../inputs/day_3.txt")
			.chars()
			.for_each(|c| {
				match c {
					'<' => position.x -= 1,
					'>' => position.x += 1,
					'^' => position.y += 1,
					'v' => position.y -= 1,
					_ => (),
				}
				houses_visited.insert(position);
			});
		houses_visited.len()
	}
	fn part_2() -> Self::Output2 {
		let mut houses_visited = HashSet::<vec2::Vec2<i32>>::new();
		let mut position_san = vec2::Vec2::new(0, 0);
		let mut position_rob = vec2::Vec2::new(0, 0);
		houses_visited.insert(position_san);
		include_str!("../inputs/day_3.txt")
			.chars()
			.enumerate()
			.for_each(|(i, c)| {
				if i%2 == 0 {
					match c {
						'<' => position_san.x -= 1,
						'>' => position_san.x += 1,
						'^' => position_san.y += 1,
						'v' => position_san.y -= 1,
						_ => (),
					}
					houses_visited.insert(position_san);
				} else {
					match c {
						'<' => position_rob.x -= 1,
						'>' => position_rob.x += 1,
						'^' => position_rob.y += 1,
						'v' => position_rob.y -= 1,
						_ => (),
					}
					houses_visited.insert(position_rob);
				}
			});
		houses_visited.len()
	}
}

#[cfg(test)]
mod tests {
	use crate::common::ChallengeT;
	use super::Challenge;

	#[test]
	fn part_1() {
		let res = Challenge::part_1();
		assert_eq!(res, 0);
	}
	#[test]
	fn part_2() {
		let res = Challenge::part_2();
		assert_eq!(res, 0);
	}
}
