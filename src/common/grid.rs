// File: common/grid.rs
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

use super::vec2::*;

pub struct Grid<T> {
	pub size: Vec2<usize>,
	pub data: Vec<Vec<T>>,
}
impl<T> Grid<T> where
	T: Default + Copy + Clone,
{
	pub fn new(x: usize, y: usize, default: &T) -> Self {
		let mut data = Vec::with_capacity(y);
		for _ in 0..y {
			let mut row = Vec::with_capacity(x);
			for _ in 0..x {
				row.push(default.clone())
			}
			data.push(row);
		}
		Self {
			size: Vec2::new(x, y),
			data: data
		}
	}
	#[inline(always)]
	pub fn get(&self, x: usize, y: usize) -> T {
		self.data[y][x]
	}
	#[inline(always)]
	pub fn set(&mut self, x: usize, y: usize, val: &T) {
		self.data[y][x] = val.clone();
	}
}

pub struct Grid2<T> {
	pub size: Vec2<usize>,
	pub data: Vec<T>,
}
impl<T> Grid2<T> where
	T: Default + Copy + Clone,
{
	pub fn new(x: usize, y: usize, default: &T) -> Self {
		let s = x*y;
		let mut data = Vec::with_capacity(s);
		for _ in 0..s {
			data.push(default.clone())
		}
		Self {
			size: Vec2::new(x, y),
			data: data
		}
	}
	#[inline(always)]
	pub fn get(&self, x: usize, y: usize) -> T {
		self.data[y*self.size.x + x]
	}
	#[inline(always)]
	pub fn set(&mut self, x: usize, y: usize, val: &T) {
		self.data[y*self.size.x + x] = val.clone();
	}
}