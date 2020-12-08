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
use grid::Grid;

pub struct Challenge {
	_input: &'static str,
	instructions: Vec<LineInstruction>,
}
impl ChallengeT for Challenge {
	type Output1 = usize;
	type Output2 = u32;

	fn day() -> u8 {
		6
	}
	fn new() -> Self {
		let input = include_str!("../inputs/day_6.txt");
		Challenge {
			_input: input,
			instructions: input
				.lines()
				.map(|line| LineInstruction::from_line(line))
				.collect(),
		}
	}
	fn part_1(&self) -> Self::Output1 {
		let mut grid = Grid::<bool>::new(1000, 1000, &false);
		self.instructions.iter().for_each(|instruction| {
			instruction.apply_instruction(&mut grid);
		});
		grid.data.iter().filter(|light_on| **light_on).count()
	}
	fn part_2(&self) -> Self::Output2 {
		let mut grid = Grid::<u32>::new(1000, 1000, &0);
		self.instructions.iter().for_each(|instruction| {
			instruction.apply_instruction_2(&mut grid);
		});
		grid.data.iter().sum()
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
		let (instruction, slice) = if line.starts_with("toggle") {
			(Instruction::Toggle, &line[7..])
		} else if line.starts_with("turn on") {
			(Instruction::On, &line[8..])
		} else {
			(Instruction::Off, &line[9..])
		};

		let mut coords = [0, 0, 0, 0];
		slice
			.split(" through ")
			.flat_map(|part| part.split(','))
			.map(|n| n.parse::<usize>().unwrap())
			.zip(coords.iter_mut())
			.for_each(|(n, coord)| {
				*coord = n;
			});

		Self {
			instruction: instruction,
			coords: coords,
		}
	}
	fn row_range(&self) -> std::ops::Range<usize> {
		self.coords[1]..(self.coords[3] + 1)
	}
	fn col_range(&self) -> std::ops::Range<usize> {
		self.coords[0]..(self.coords[2] + 1)
	}
	fn apply_instruction(&self, grid: &mut Grid<bool>) {
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
	fn apply_instruction_2(&self, grid: &mut Grid<u32>) {
		for y in self.row_range() {
			for x in self.col_range() {
				let mut val = grid.get(x, y);
				match self.instruction {
					Instruction::On => val += 1,
					Instruction::Toggle => val += 2,
					Instruction::Off => match val {
						0 => continue,
						1 => val = 0,
						_ => val -= 1,
					},
				}
				grid.set(x, y, &val);
			}
		}
	}
}

#[cfg(test)]
mod tests {
	use super::Challenge;
	use super::*;
	use crate::common::ChallengeT;

	#[test]
	fn part_1_test_instruction() {
		let res = LineInstruction::from_line("turn on 0,0 through 10,10");
		assert_eq!(
			LineInstruction {
				instruction: Instruction::On,
				coords: [0, 0, 10, 10]
			},
			res
		);
		let res = LineInstruction::from_line("turn off 23,12 through 24,199");
		assert_eq!(
			LineInstruction {
				instruction: Instruction::Off,
				coords: [23, 12, 24, 199]
			},
			res
		);
		let res = LineInstruction::from_line("toggle 23,12 through 24,199");
		assert_eq!(
			LineInstruction {
				instruction: Instruction::Toggle,
				coords: [23, 12, 24, 199]
			},
			res
		);
	}

	#[test]
	fn part_1_test() {
		let res = Challenge::new().part_1();
		assert_eq!(res, 377891);
	}
	#[test]
	fn part_2_test() {
		let res = Challenge::new().part_2();
		assert_eq!(res, 14110788);
	}

	use test::Bencher;
	#[bench]
	fn part_line_instruction_bench(b: &mut Bencher) {
		b.iter(|| LineInstruction::from_line("toggle 23,12 through 24,199"))
	}
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
