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

pub struct Challenge {
    input: &'static str,
}
impl ChallengeT for Challenge {
    type Output1 = usize;
    type Output2 = usize;

    fn day() -> u8 {
        5
    }
    fn new() -> Self {
        Challenge {
            input: include_str!("../inputs/day_5.txt"),
        }
    }
    fn part_1(&self) -> Self::Output1 {
        let mut nice = 0;
        self.input.lines().for_each(|line| {
            if is_nice_string(line) {
                nice += 1;
            }
        });

        nice
    }
    fn part_2(&self) -> Self::Output2 {
        let mut nice = 0;
        self.input.lines().for_each(|line| {
            if is_new_nice_string(line) {
                nice += 1;
            }
        });
        nice
    }
}
fn is_nice_string(s: &str) -> bool {
    let mut vowel_count = 0;
    let mut three_vowels = false;
    let mut double_letters = false;
    let mut previouse = None;

    for c in s.chars() {
        if let Some(p) = previouse {
            match (p, c) {
                ('a', 'b') | ('c', 'd') | ('p', 'q') | ('x', 'y') => return false,
                _ => (),
            }
            if !double_letters {
                if p == c {
                    double_letters = true;
                }
            }
        }
        if !three_vowels {
            match c {
                'a' | 'e' | 'i' | 'o' | 'u' => {
                    vowel_count += 1;
                    if vowel_count == 3 {
                        three_vowels = true;
                    }
                }
                _ => (),
            }
        }
        previouse = Some(c);
    }
    three_vowels && double_letters
}
fn is_new_nice_string(line: &str) -> bool {
    let chars = line.chars().collect::<Vec<char>>();

    let mut repeat_between = false;
    let mut two_letters_repeat_twice = false;

    for i in 0..chars.len() - 2 {
        if chars[i] == chars[i + 2] {
            repeat_between = true;
            break;
        }
    }
    for c in 0..chars.len() - 3 {
        let current = (chars[c], chars[c + 1]);
        for n in (c + 2)..chars.len() - 1 {
            let next = (chars[n], chars[n + 1]);
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
    use super::*;
    use crate::common::ChallengeT;

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
        assert!(is_new_nice_string(&nice[0]));
        assert!(is_new_nice_string(&nice[1]));
        let naughty = ["uurcxstgmygtbstg", "ieodomkazucvgmuy"];
        assert!(!is_new_nice_string(&naughty[0]));
        assert!(!is_new_nice_string(&naughty[1]));
    }

    #[test]
    fn part_1() {
        let res = Challenge::new().part_1();
        assert_eq!(res, 236);
    }
    #[test]
    fn part_2() {
        let res = Challenge::new().part_2();
        assert_eq!(res, 51);
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
