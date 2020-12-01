// File: lib.rs
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

#![feature(test)]
extern crate test;

pub mod common;
use common::ChallengeT;

pub mod day_1;
pub mod day_2;
pub mod day_3;
pub mod day_5;

use std::env::args;

pub fn main() {
	for arg in args() {
		match arg.as_str() {
			a if a.starts_with("target") => (),
			"1" => day_1::Challenge::print_result(),
			"2" => day_2::Challenge::print_result(),
			"3" => day_3::Challenge::print_result(),
			"5" => day_5::Challenge::print_result(),
			_ => println!("ERROR: UNKNOWN ARGUMENT"),
		}
	}
}


#[cfg(test)]
mod tests {
	use test::Bencher;
	use super::*;

	#[bench]
	fn all(b: &mut Bencher) {
		b.iter(|| {
			day_1::Challenge::print_result();
			day_2::Challenge::print_result();
			day_3::Challenge::print_result();
			day_5::Challenge::print_result();
		})
	}
}
