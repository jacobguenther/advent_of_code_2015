// File: day_6.rs
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
use grid::Grid2;

pub struct Challenge {}
impl ChallengeT for Challenge {
	type Output1 = usize;
	type Output2 = u32;

	fn day() -> u8 {
		6
	}
	fn part_1() -> Self::Output1 {
		let mut grid = Grid2::<bool>::new(1000, 1000, &false);
		include_str!("../inputs/day_6.txt")
			.lines()
			.for_each(|line| LineInstruction::from_line(line)
				.apply_instruction(&mut grid)
			);
		count_lights_on(&grid)
	}
	fn part_2() -> Self::Output2 {
		let mut grid = Grid2::<u32>::new(1000, 1000, &0);
		include_str!("../inputs/day_6.txt")
			.lines()
			.for_each(|line| LineInstruction::from_line(line)
				.apply_instruction_2(&mut grid)
			);
		get_brightness(&grid)
	}
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum Instruction {
	On,
	Off,
	Toggle,
}
#[derive(Copy, Clone, Debug, PartialEq)]
struct LineInstruction {
	instruction: Instruction,
	coords: [usize; 4],
}
impl LineInstruction {
	fn from_line(line: &str) -> Self {
		let (instruction, slice) = if line.starts_with("turn on") {
			(Instruction::On, &line[8..])
		} else if line.starts_with("turn of") {
			(Instruction::Off, &line[9..])
		} else {
			(Instruction::Toggle, &line[7..])
		};

		let mut parts: [&str; 2] = ["", ""];
		slice.split(" through ")
			.take(2)
			.enumerate()
			.for_each(|(i, p)| {
				parts[i] = p
			});
		let mut coords = [0, 0, 0, 0];
		parts[0].split(',')
			.chain(parts[1].split(','))
			.take(4)
			.map(|n| n.parse::<usize>().unwrap())
			.enumerate()
			.for_each(|(i, n)| {
				coords[i] = n;
			});

		Self {
			instruction: instruction,
			coords: coords,
		}
	}
	fn row_range(&self) -> std::ops::Range<usize> {
		self.coords[1]..(self.coords[3]+1)
	}
	fn col_range(&self) -> std::ops::Range<usize> {
		self.coords[0]..(self.coords[2]+1)
	}
	fn apply_instruction(&self, grid: &mut Grid2<bool>) {
		for y in self.row_range() {
			for x in self.col_range() {
				let val = match self.instruction {
					Instruction::On => true,
					Instruction::Off => false,
					Instruction::Toggle => !grid.get(x, y),
				};
				grid.set(x, y, &val);
			}
		}
	}
	fn apply_instruction_2(&self, grid: &mut Grid2<u32>) {
		for y in self.row_range() {
			for x in self.col_range() {
				let mut val = grid.get(x, y);
				match self.instruction {
					Instruction::On => val += 1,
					Instruction::Off => match val {
						0 => continue,
						1 => val = 0,
						_ => val -= 1,
					}
					Instruction::Toggle => val += 2,
				}
				grid.set(x, y, &val);
			}
		}
	}
}
fn count_lights_on(grid: &Grid2<bool>) -> usize {
	grid.data.iter()
		.filter(|light_on| **light_on)
		.count()
}
fn get_brightness(grid: &Grid2<u32>) -> u32 {
	grid.data.iter().sum()
}

#[cfg(test)]
mod tests {
	use crate::common::ChallengeT;
	use super::Challenge;
	use super::*;

	#[test]
	fn part_1_test_instruction() {
		let res = LineInstruction::from_line("turn on 0,0 through 10,10");
		assert_eq!(LineInstruction {
			instruction: Instruction::On,
			coords: [0,0,10,10]
		}, res);
		let res = LineInstruction::from_line("turn off 23,12 through 24,199");
		assert_eq!(LineInstruction {
			instruction: Instruction::Off,
			coords: [23,12,24,199]
		}, res);
		let res = LineInstruction::from_line("toggle 23,12 through 24,199");
		assert_eq!(LineInstruction {
			instruction: Instruction::Toggle,
			coords: [23,12,24,199]
		}, res);
	}

	#[test]
	fn part_1_test() {
		let res = Challenge::part_1();
		assert_eq!(res, 377891);
	}
	#[test]
	fn part_2_test() {
		let res = Challenge::part_2();
		assert_eq!(res, 14110788);
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
