// File: day_7.rs
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

type Operations = HashMap<&'static str, Op>;
pub struct Challenge {
	input: &'static str,
	ops: Operations,
}
impl ChallengeT for Challenge {
	type Output1 = u16;
	type Output2 = u16;

	fn day() -> u8 {
		7
	}
	fn new() -> Self {
		let input = include_str!("../inputs/day_7.txt");
		Challenge {
			input: input,
			ops: input.lines()
				.map(|line| {
					Op::from_line(line)
				})
				.collect::<Operations>(),
		}
	}
	fn part_1(&self) -> Self::Output1 {
		let mut cache = HashMap::new();
		solve_for("a", &self.ops, &mut cache)
	}
	fn part_2(&self) -> Self::Output2 {
		let a = 3176;
		let mut cache = HashMap::new();
		cache.insert("b", a);
		solve_for("a", &self.ops, &mut cache)
	}
}
fn get_val(signal_name: &'static str, s: &Signal, ops: &Operations, cache: &mut HashMap<&str, u16>) -> u16 {
	let val = match s {
		Signal::Input(input) => {
			if let Some(cached) = cache.get(input) {
				*cached
			} else {
				let partial = solve_for(input, ops, cache);
				println!("partial {}", partial);
				cache.insert(input, partial);
				partial
			}
		},
		Signal::Value(v) => {
			println!("Value Signal {}", v);
			cache.insert(signal_name, *v);
			*v
		}
	};
	val
}
fn solve_for(signal_name: &'static str, ops: &Operations, cache: &mut HashMap<&str, u16>) -> u16 {
	let op = ops.get(signal_name).unwrap();
	println!("{} -> {:?}", signal_name, op);
	match op {
		Op::ASSIGN(a) => get_val(signal_name, a, ops, cache),
		Op::AND(a, b) => {
			let a_val = get_val(signal_name, a, ops, cache);
			let b_val = get_val(signal_name, b, ops, cache);
			let res = a_val & b_val;
			res
		},
		Op::OR(a, b) => {
			let a_val = get_val(signal_name, a, ops, cache);
			let b_val = get_val(signal_name, b, ops, cache);
			let res = a_val | b_val;
			res
		},
		Op::NOT(a) => {
			let a_val = get_val(signal_name, a, ops, cache);
			let res = !a_val;
			res
		},
		Op::LSHIFT(a, b) => {
			let a_val = get_val(signal_name, a, ops, cache);
			let res = a_val << b;
			res
		},
		Op::RSHIFT(a, b) => {
			let a_val = get_val(signal_name, a, ops, cache);
			let res = a_val >> b;
			res
		},
	}
}
#[derive(Debug)]
enum Signal {
	Input(&'static str),
	Value(u16),
}
impl From<&'static str> for Signal {
	fn from(s: &'static str) -> Signal {
		if let Ok(n) = s.parse() {
			Signal::Value(n)
		} else {
			Signal::Input(s)
		}
	}
}
#[derive(Debug)]
enum Op {
	// value to assign
	ASSIGN(Signal),
	// a + b
	AND(Signal, Signal),
	// a | b
	OR(Signal, Signal),
	// !a
	NOT(Signal),
	// a << u16
	LSHIFT(Signal, u16),
	// b >> u16
	RSHIFT(Signal, u16),
}
impl Op {
	fn from_line<'l>(line: &'static str) -> (&'static str, Op) {
		let parts = line.split(' ').collect::<Vec<&'static str>>();
		for i in 0..parts.len() {
			match parts[i] {
				"AND" |
				"OR" => {
					let a = parts[0].into();
					let b = parts[2].into();
					let c = parts[4];
					match parts[i] {
						"AND" => return (c, Op::AND(a, b)),
						"OR" => return (c, Op::OR(a, b)),
						_ => (),
					}
				},
				"NOT" => {
					let a = parts[1].into();
					let b = parts[3];
					return (b, Op::NOT(a));
				},
				"LSHIFT" |
				"RSHIFT" => {
					let a = parts[0].into();
					let b = parts[2].parse().unwrap();
					let c = parts[4].into();
					match parts[i] {
						"LSHIFT" => return (c, Op::LSHIFT(a, b)),
						"RSHIFT" => return (c, Op::RSHIFT(a, b)),
						_ => (),
					}
				},
				_ => (),
			}
		}
		let a = parts[0].into();
		let b = parts[2];
		(b, Op::ASSIGN(a))
	}
}

#[cfg(test)]
mod tests {
	use crate::common::ChallengeT;
	use super::*;

	#[test]
	fn part_1_test() {
		let res = Challenge::new().part_1();
		assert_eq!(res, 3176);
	}
	#[test]
	fn part_2_test() {
		let res = Challenge::new().part_2();
		assert_eq!(res, 0);
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
