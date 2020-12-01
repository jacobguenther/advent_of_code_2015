// File: day_5.rs
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
	type Output1 = usize;
	type Output2 = usize;

	fn day() -> u8 {
		5
	}
	fn part_1() -> Self::Output1 {
		let mut nice = 0;
		include_str!("../inputs/day_5.txt")
			.lines()
			.for_each(|line| {
				if is_nice_string(line) {
					nice += 1;
				}
			});

		let mut nice_2 = 0;
		for line in include_str!("../inputs/day_5.txt").lines() {
			println!("{}", line);
			if is_nice_string(line) {
				nice_2 += 1;
			}
		};
		assert_eq!(nice, nice_2);
		nice
	}
	fn part_2() -> Self::Output2 {
		let mut nice = 0;
		include_str!("../inputs/day_5.txt")
			.lines()
			.for_each(|l| {
				let line = l.chars().collect::<Vec<char>>();
				if is_new_nice_string(&line) {
					nice += 1;
				}
			});
		nice
	}
}
fn is_nice_string(s: &str) -> bool {
	let naughty_pairs = ["ab", "cd", "pq", "xy"];
	for p in naughty_pairs.iter() {
		if s.contains(p) {
			return  false;
		}
	}
	let vowel_count = s.chars().fold(0, |count, c| {
		count + match c {
			'a' | 'e' | 'i' | 'o' | 'u' => 1,
			_ => 0,
		}
	});
	let mut double_letters = false;
	let mut previouse = None;
	for c in s.chars() {
		if let Some(p) = previouse {
			if c == p {
				double_letters = true;
				break;
			}
		}
		previouse = Some(c);
	}
	double_letters && vowel_count >= 3
}
fn is_new_nice_string(chars: &Vec<char>) -> bool {
	let mut repeat_between = false;
	for i in 0..chars.len()-2 {
		if chars[i] == chars[i+2] {
			repeat_between = true;
			break;
		}
	}
	let mut two_letters_repeat_twice = false;
	for i in 0..chars.len()-3 {
		let current = (chars[i], chars[i+1]);
		for ii in (i+2)..chars.len()-1 {
			let next = (chars[ii], chars[ii+1]);
			if current == next {
				two_letters_repeat_twice = true;
				break;
			}
		}
		if two_letters_repeat_twice {
			break;
		}
	}
	repeat_between && two_letters_repeat_twice
}
#[cfg(test)]
mod tests {
	use crate::common::ChallengeT;
	use super::*;

	#[test]
	fn nice_string_test() {
		assert!(is_nice_string("ugknbfddgicrmopn"));
		assert!(is_nice_string("aaa"));
		assert!(!is_nice_string("jchzalrnumimnmhp"));
		assert!(!is_nice_string("haegwjzuvuyypxyu"));
		assert!(!is_nice_string("dvszwmarrgswjxmb"));
	}

	#[test]
	fn new_nice_string_test() {
		let nice = ["qjhvhtzxzqqjkmpb", "xxyxx"];
		assert!(is_new_nice_string(&nice[0].chars().collect()));
		assert!(is_new_nice_string(&nice[1].chars().collect()));
		let naughty = ["uurcxstgmygtbstg", "ieodomkazucvgmuy"];
		assert!(!is_new_nice_string(&naughty[0].chars().collect()));
		assert!(!is_new_nice_string(&naughty[1].chars().collect()));
	}

	#[test]
	fn part_1() {
		let res = Challenge::part_1();
		assert_eq!(res, 236);
	}
	#[test]
	fn part_2() {
		let res = Challenge::part_2();
		assert_eq!(res, 51);
	}
}
