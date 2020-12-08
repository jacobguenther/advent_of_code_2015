// File: day_9.rs
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
use std::collections::HashMap;

pub struct Challenge {
	part_1_result: usize,
	part_2_result: usize,
}
impl ChallengeT for Challenge {
	type Output1 = usize;
	type Output2 = usize;

	fn day() -> u8 {
		9
	}
	fn new() -> Self {
		let input = include_str!("../inputs/day_9.txt");
		let (min, max) = solve(input);
		Challenge {
			part_1_result: min,
			part_2_result: max,
		}
	}
	fn part_1(&self) -> Self::Output1 {
		self.part_1_result
	}
	fn part_2(&self) -> Self::Output2 {
		self.part_2_result
	}
}
fn solve(input: &'static str) -> (usize, usize) {
	let mut cities = Vec::<&'static str>::new();
	let mut distances_map = HashMap::<(&'static str, &'static str), usize>::new();
	input.lines().for_each(|line| {
		let parts = line.split(' ').collect::<Vec<&'static str>>();
		let c1 = parts[0];
		let c2 = parts[2];
		let distance = parts[4].parse().unwrap();
		if !cities.contains(&c1) {
			cities.push(c1);
		}
		if !cities.contains(&c2) {
			cities.push(c2);
		}
		distances_map.insert((c1, c2), distance);
		distances_map.insert((c2, c1), distance);
	});

	use itertools::Itertools;

	let permutations = cities.iter().permutations(cities.len());

	let mut min = usize::MAX;
	let mut max = 0;
	let eight_factorial = 40320;
	for row in permutations.take(eight_factorial / 2) {
		let path_distance = sum_path(&row, &distances_map);
		if max < path_distance {
			max = path_distance;
		} else if min > path_distance {
			min = path_distance;
		}
	}
	(min, max)
}
fn sum_path(path: &Vec<&&str>, distances: &HashMap<(&str, &str), usize>) -> usize {
	path.iter()
		.take(path.len() - 1)
		.zip(&path[1..path.len()])
		.map(|(first, second)| distances.get(&(*first, *second)).unwrap())
		.sum()
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::common::ChallengeT;

	#[test]
	fn part_1_test() {
		let res = Challenge::new().part_1();
		assert_eq!(res, 117);
	}
	#[test]
	fn part_2_test() {
		let res = Challenge::new().part_2();
		assert_eq!(res, 909);
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
