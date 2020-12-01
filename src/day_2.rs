// File: day_2.rs
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

pub struct Challenge {}
impl ChallengeT for Challenge {
	type Output1 = i32;
	type Output2 = i32;

	fn day() -> u8 {
		2
	}
	fn part_1() -> Self::Output1 {
		include_str!("../inputs/day_2.txt")
			.lines()
			.map(|l| {
				let mut cs: Vec<i32> = l.split('x')
					.map(|s| s.parse::<i32>().unwrap())
					.collect();
				cs.sort();
				let (l, w, h) = (cs[0], cs[1], cs[2]);
				let smallest_side = cs[0] * cs[1];
				2*(l*w + w*h + h*l) + smallest_side
			})
			.sum()
	}
	fn part_2() -> Self::Output2 {
		include_str!("../inputs/day_2.txt")
			.lines()
			.map(|l| {
				let mut cs: Vec<i32> = l.split('x')
					.map(|s| s.parse::<i32>().unwrap() )
					.collect();
				cs.sort();
				let (l, w, h) = (cs[0], cs[1], cs[2]);
				let ribbon = cs[0]+cs[0] + cs[1]+cs[1];
				let volume = l*w*h;
				ribbon + volume
			})
			.sum()
	}
}

#[cfg(test)]
mod tests {
	use crate::common::ChallengeT;
	use super::Challenge;

	#[test]
	fn part_1() {
		let res = Challenge::part_1();
		assert_eq!(res, 1606483);
	}
	#[test]
	fn part_2() {
		let res = Challenge::part_2();
		assert_eq!(res, 3842356);
	}
}
