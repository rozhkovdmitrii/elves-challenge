/// This module is an implementation of [Advent of Code day 1](https://adventofcode.com/2023/day/1)
///
/// It provides the CalibrationDoc abstraction, which is designed to retrieve a calibration value
/// from a document consisting of lines of text. Each of them initially contains a certain
/// calibration value
///
/// Getting the calibration value could be performed by calling `get_line_calibration_v2` method
///

#[path = "calibration_doc/pattern.rs"]
mod pattern;

use pattern::{Pattern, BKW_PATTERNS, FWD_PATTERNS};

use self::Direction::{Backward, Forward};

#[derive(Clone, Copy)]
enum Direction {
    Forward,
    Backward,
}

pub struct CalibrationDoc<'a>(&'a str);

impl CalibrationDoc<'_> {
    pub fn new(input: &str) -> CalibrationDoc {
        CalibrationDoc(input)
    }

    pub fn get_calibration_v1(&self) -> u32 {
        self.0.lines().map(Self::get_line_calibration_v1).sum()
    }

    fn get_line_calibration_v1(single_line: &str) -> u32 {
        let digit_from_index = |i| single_line.chars().nth(i).expect("Expected index to be");
        let check = |c| char::is_ascii_digit(&c);
        let first = single_line.find(check).map(digit_from_index);
        let last = single_line.rfind(check).map(digit_from_index);
        let two_digit_num = first.iter().chain(last.iter()).collect::<String>();
        two_digit_num.parse().unwrap_or_default()
    }

    pub fn get_calibration_v2(&self) -> u64 {
        self.0.lines().map(Self::line_calibration_v2).sum()
    }

    fn line_calibration_v2(single_line: &str) -> u64 {
        // The most performant solution is to make an automate that parses an input char by char
        // and gives birth for a resulting number
        let mut result = Self::look_forward(single_line);
        result = result * 10 + Self::look_backward(single_line);
        result
    }

    fn look_forward(value: &str) -> u64 {
        Self::look_for_digit(value, Forward)
    }

    fn look_backward(value: &str) -> u64 {
        Self::look_for_digit(value, Backward)
    }

    fn look_for_digit(input: &str, direction: Direction) -> u64 {
        let mut input = input;
        let pattern = match direction {
            Forward => &FWD_PATTERNS,
            Backward => &BKW_PATTERNS,
        };

        while !input.is_empty() {
            if let Some(result) = Self::look_for_digit_impl(input, pattern, direction) {
                return result;
            }

            input = match direction {
                Forward => &input[1..],
                Backward => &input[..input.len() - 1],
            };
        }
        0
    }

    fn look_for_digit_impl(input: &str, pattern: &Pattern, direction: Direction) -> Option<u64> {
        if let Pattern::Result(result) = pattern {
            return Some(*result);
        }

        if input.is_empty() {
            return None;
        }

        match pattern {
            Pattern::Result(value) => Some(*value),
            Pattern::Check(rules) => {
                let (char, next_input) = match direction {
                    Forward => (input.chars().nth(0), &input[1..]),
                    Backward => (input.chars().last(), &input[0..input.len() - 1]),
                };
                let Some(ref mut pattern) = rules.get(&char.expect("Next char expected to exist"))
                else {
                    return None;
                };
                Self::look_for_digit_impl(next_input, pattern, direction)
            }
        }
    }
}

#[test]
fn test_look_for_digit() {
    assert_eq!(CalibrationDoc::look_forward("one"), 1);
    assert_eq!(CalibrationDoc::look_forward("two"), 2);
    assert_eq!(CalibrationDoc::look_forward("three"), 3);
    assert_eq!(CalibrationDoc::look_forward("four"), 4);
    assert_eq!(CalibrationDoc::look_forward("five"), 5);
    assert_eq!(CalibrationDoc::look_forward("six"), 6);
    assert_eq!(CalibrationDoc::look_forward("seven"), 7);
    assert_eq!(CalibrationDoc::look_forward("eight"), 8);
    assert_eq!(CalibrationDoc::look_forward("nine"), 9);
    assert_eq!(CalibrationDoc::look_forward("0"), 0);
    assert_eq!(CalibrationDoc::look_forward("1"), 1);
    assert_eq!(CalibrationDoc::look_forward("2"), 2);
    assert_eq!(CalibrationDoc::look_forward("3"), 3);
    assert_eq!(CalibrationDoc::look_forward("4"), 4);
    assert_eq!(CalibrationDoc::look_forward("5"), 5);
    assert_eq!(CalibrationDoc::look_forward("6"), 6);
    assert_eq!(CalibrationDoc::look_forward("7"), 7);
    assert_eq!(CalibrationDoc::look_forward("8"), 8);
    assert_eq!(CalibrationDoc::look_forward("9"), 9);

    assert_eq!(CalibrationDoc::look_backward("one"), 1);
    assert_eq!(CalibrationDoc::look_backward("two"), 2);
    assert_eq!(CalibrationDoc::look_backward("three"), 3);
    assert_eq!(CalibrationDoc::look_backward("four"), 4);
    assert_eq!(CalibrationDoc::look_backward("five"), 5);
    assert_eq!(CalibrationDoc::look_backward("six"), 6);
    assert_eq!(CalibrationDoc::look_backward("seven"), 7);
    assert_eq!(CalibrationDoc::look_backward("eight"), 8);
    assert_eq!(CalibrationDoc::look_backward("nine"), 9);

    assert_eq!(CalibrationDoc::look_backward("0"), 0);
    assert_eq!(CalibrationDoc::look_backward("1"), 1);
    assert_eq!(CalibrationDoc::look_backward("2"), 2);
    assert_eq!(CalibrationDoc::look_backward("3"), 3);
    assert_eq!(CalibrationDoc::look_backward("4"), 4);
    assert_eq!(CalibrationDoc::look_backward("5"), 5);
    assert_eq!(CalibrationDoc::look_backward("6"), 6);
    assert_eq!(CalibrationDoc::look_backward("7"), 7);
    assert_eq!(CalibrationDoc::look_backward("8"), 8);
    assert_eq!(CalibrationDoc::look_backward("9"), 9);

    assert_eq!(CalibrationDoc::look_forward("asdf1"), 1);
    assert_eq!(CalibrationDoc::look_backward("1asdf"), 1);
    assert_eq!(CalibrationDoc::look_backward("twone"), 1);
    assert_eq!(CalibrationDoc::look_forward("done"), 1);
    assert_eq!(CalibrationDoc::look_backward("twoe"), 2);
    assert_eq!(CalibrationDoc::look_forward("dtwo"), 2);
    assert_eq!(CalibrationDoc::look_forward("adthreeaf"), 3);
    assert_eq!(CalibrationDoc::look_backward("aathreeadsf"), 3);
    assert_eq!(CalibrationDoc::look_forward("doneasdf"), 1);
    assert_eq!(CalibrationDoc::look_backward("eightwo"), 2);
    assert_eq!(CalibrationDoc::look_forward("eightwo"), 8);
    assert_eq!(CalibrationDoc::look_backward("nineight"), 8);

    assert_eq!(CalibrationDoc::look_forward("ddd"), 0);
}

#[test]
fn test_line_calibration() {
    assert_eq!(CalibrationDoc::line_calibration_v2(""), 0);
    assert_eq!(CalibrationDoc::line_calibration_v2("a"), 0);
    assert_eq!(CalibrationDoc::line_calibration_v2("1"), 11);
    assert_eq!(CalibrationDoc::line_calibration_v2("9"), 99);
    assert_eq!(CalibrationDoc::line_calibration_v2("01"), 1);
    assert_eq!(CalibrationDoc::line_calibration_v2("10"), 10);
    assert_eq!(CalibrationDoc::line_calibration_v2("abcdefghij"), 0);
    assert_eq!(CalibrationDoc::line_calibration_v2("1abcdefghij"), 11);
    assert_eq!(CalibrationDoc::line_calibration_v2("abcdefghij1"), 11);
    assert_eq!(CalibrationDoc::line_calibration_v2("abcd1efghij"), 11);
    assert_eq!(CalibrationDoc::line_calibration_v2("0abcd1efghij"), 1);
    let input = "523aadsff21345125132sdf9";
    assert_eq!(CalibrationDoc::line_calibration_v2(input), 59);
    assert_eq!(CalibrationDoc::line_calibration_v2("1abc2"), 12);
    assert_eq!(CalibrationDoc::line_calibration_v2("pqr3stu8vwx"), 38);
    assert_eq!(CalibrationDoc::line_calibration_v2("a1b2c3d4e5f"), 15);
    assert_eq!(CalibrationDoc::line_calibration_v2("treb7uchet"), 77);
    assert_eq!(CalibrationDoc::line_calibration_v2("on e"), 0);
    assert_eq!(CalibrationDoc::line_calibration_v2("one"), 11);
    assert_eq!(CalibrationDoc::line_calibration_v2("onetwothree"), 13);
    assert_eq!(CalibrationDoc::line_calibration_v2("0onetwothree"), 3);
    assert_eq!(CalibrationDoc::line_calibration_v2("nine"), 99);
    assert_eq!(CalibrationDoc::line_calibration_v2("fournine"), 49);
    assert_eq!(CalibrationDoc::line_calibration_v2("fourthfivefnine"), 49);
    assert_eq!(CalibrationDoc::line_calibration_v2("4fourthfivefnine"), 49);
    assert_eq!(CalibrationDoc::line_calibration_v2("7fourthfivefnine"), 79);
    assert_eq!(CalibrationDoc::line_calibration_v2("one"), 11);
    assert_eq!(CalibrationDoc::line_calibration_v2("two"), 22);
    assert_eq!(CalibrationDoc::line_calibration_v2("three"), 33);
    assert_eq!(CalibrationDoc::line_calibration_v2("four"), 44);
    assert_eq!(CalibrationDoc::line_calibration_v2("five"), 55);
    assert_eq!(CalibrationDoc::line_calibration_v2("six"), 66);
    assert_eq!(CalibrationDoc::line_calibration_v2("seven"), 77);
    assert_eq!(CalibrationDoc::line_calibration_v2("eight"), 88);
    assert_eq!(CalibrationDoc::line_calibration_v2("nine"), 99);

    assert_eq!(CalibrationDoc::line_calibration_v2("one1"), 11);
    assert_eq!(CalibrationDoc::line_calibration_v2("two2"), 22);
    assert_eq!(CalibrationDoc::line_calibration_v2("three3"), 33);
    assert_eq!(CalibrationDoc::line_calibration_v2("four4"), 44);
    assert_eq!(CalibrationDoc::line_calibration_v2("five5"), 55);
    assert_eq!(CalibrationDoc::line_calibration_v2("six6"), 66);
    assert_eq!(CalibrationDoc::line_calibration_v2("seven7"), 77);
    assert_eq!(CalibrationDoc::line_calibration_v2("eight8"), 88);
    assert_eq!(CalibrationDoc::line_calibration_v2("nine9"), 99);

    assert_eq!(CalibrationDoc::line_calibration_v2("0asdfone"), 1);
    assert_eq!(CalibrationDoc::line_calibration_v2("0adsftwo"), 2);
    assert_eq!(CalibrationDoc::line_calibration_v2("0adsfthree"), 3);
    assert_eq!(CalibrationDoc::line_calibration_v2("0adsffour"), 4);
    assert_eq!(CalibrationDoc::line_calibration_v2("0adsffive"), 5);
    assert_eq!(CalibrationDoc::line_calibration_v2("0adsfsix"), 6);
    assert_eq!(CalibrationDoc::line_calibration_v2("0asdfseven"), 7);
    assert_eq!(CalibrationDoc::line_calibration_v2("0adsfeight"), 8);
    assert_eq!(CalibrationDoc::line_calibration_v2("0adsfnine"), 9);

    assert_eq!(CalibrationDoc::line_calibration_v2("2htzsvdhvqvdjv"), 22);
    let input = "fivetwocrhmvxqkvbeightfive1qzcxvds";
    assert_eq!(CalibrationDoc::line_calibration_v2(input), 51);
    assert_eq!(CalibrationDoc::line_calibration_v2("eightoneqjvzv3"), 83);
    assert_eq!(CalibrationDoc::line_calibration_v2("277"), 27);
    assert_eq!(CalibrationDoc::line_calibration_v2("five93"), 53);
    assert_eq!(CalibrationDoc::line_calibration_v2("eightwo"), 82);
    let input = "2tqbxgrrpmxqfglsqjkqthree6nhjvbxpflhr1eightwohr";
    assert_eq!(CalibrationDoc::line_calibration_v2(input), 22);
}

#[test]
fn test_doc_calibration() {
    let input = "3a;sdklfjlaskdj f1";
    assert_eq!(CalibrationDoc::new(input).get_calibration_v2(), 31);
    let input = r#"1abc2
                   pqr3stu8vwx
                   a1b2c3d4e5f
                   treb7uchet"#;
    assert_eq!(CalibrationDoc::new(input).get_calibration_v2(), 142);

    let input = r#"two1nine
                   eightwothree
                   abcone2threexyz
                   xtwone3four
                   4nineeightseven2
                   zoneight234
                   7pqrstsixteen"#;
    assert_eq!(CalibrationDoc::new(input).get_calibration_v2(), 281);

    let input = include_str!("test_data/calibration_doc_1");
    assert_eq!(CalibrationDoc::new(input).get_calibration_v2(), 572);
    let input = include_str!("test_data/calibration_doc_2");
    assert_eq!(CalibrationDoc::new(input).get_calibration_v2(), 911);

    let input = include_str!("test_data/calibration_doc_github");
    assert_eq!(CalibrationDoc::new(input).get_calibration_v1(), 55208);
    let input = include_str!("test_data/calibration_doc_github");
    assert_eq!(CalibrationDoc::new(input).get_calibration_v2(), 54578);
    let input = include_str!("test_data/calibration_doc_huge");
    assert_eq!(CalibrationDoc::new(input).get_calibration_v1(), 54605);
    let input = include_str!("test_data/calibration_doc_huge");
    assert_eq!(CalibrationDoc::new(input).get_calibration_v2(), 55429);
}
