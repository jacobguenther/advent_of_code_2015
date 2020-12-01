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
	type Output1 = u32;
	type Output2 = u32;

	fn day() -> u8 {
		2
	}
	fn part_1() -> Self::Output1 {
		let mut box_dims = [0, 0, 0];
		include_str!("../inputs/day_2.txt")
			.lines()
			.map(|line| {
				parse_line(line, &mut box_dims);
				calc_wrapping_paper(&box_dims)
			})
			.sum()
	}
	fn part_2() -> Self::Output2 {
		let mut box_dims = [0, 0, 0];
		include_str!("../inputs/day_2.txt")
			.lines()
			.map(|line| {
				parse_line(line, &mut box_dims);
				calc_ribbon(&box_dims)
			})
			.sum()
	}
}
fn parse_line(line: &str, out: &mut [u32]) {
	line.split('x')
		.map(|s| s.parse::<u32>().unwrap() )
		.enumerate()
		.for_each(|(i, dim)| {
			out[i] = dim
		});
	out.sort();
}
// surface area + surface area of smallest side
fn calc_wrapping_paper(box_dims: &[u32]) -> u32 {
	let (l, w, h) = (box_dims[0], box_dims[1], box_dims[2]);
	3*l*w + 2*(w*h + h*l)
}
fn calc_ribbon(box_dims: &[u32]) -> u32 {
	let (l, w, h) = (box_dims[0], box_dims[1], box_dims[2]);
	let volume = l*w*h;
	let shortest_parimeter = l+l + w+w;
	volume + shortest_parimeter
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

	use test::Bencher;
	#[bench]
	fn part_1_bench(b: &mut Bencher) {
		b.iter(|| Challenge::part_1())
	}
	#[bench]
	fn part_2_bench(b: &mut Bencher) {
		b.iter(|| Challenge::part_2())
	}
}
