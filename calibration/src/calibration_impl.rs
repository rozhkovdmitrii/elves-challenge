use crate::calibration_impl::Direction::{Backward, Forward};
use once_cell::sync::Lazy;
use std::collections::VecDeque; //TODO get rid of it

struct CalibrationDoc<'a>(&'a str);

#[derive(Clone, Copy)]
enum Direction {
    Forward,
    Backward,
}

type Rule = ((char, char), char);
type Route = Vec<Rule>;

static FWD_ROUTES: Lazy<Vec<Vec<Rule>>> = Lazy::new(|| {
    vec![
        route("0", '0'),
        route("one", '1'),
        route("two", '2'),
        route("three", '3'),
        route("four", '4'),
        route("five", '5'),
        route("six", '6'),
        route("seven", '7'),
        route("eight", '8'),
        route("nine", '9'),
    ]
});

static BKW_ROUTES: Lazy<Vec<Vec<Rule>>> = Lazy::new(|| {
    vec![
        route("0", '0'),
        route("eno", '1'),
        route("owt", '2'),
        route("eerht", '3'),
        route("ruof", '4'),
        route("evif", '5'),
        route("xis", '6'),
        route("neves", '7'),
        route("thgie", '8'),
        route("enin", '9'),
    ]
});

fn find(input: &str, direction: Direction) -> Option<u32> {
    let mut input = input;
    let routes = match direction {
        Forward => &FWD_ROUTES,
        Backward => &BKW_ROUTES,
    };

    while !input.is_empty() {
        for route in routes.iter() {
            let current = route[0].0 .1;
            let current_input = input;
            if let Some(value) = find_impl(current_input, current, 0, &route, direction) {
                return Some(value);
            };
        }
        input = match direction {
            Forward => &input[1..],
            Backward => &input[..input.len() - 1],
        };
    }
    None
}

fn route(digit_alias: &str, goal: char) -> Vec<Rule> {
    let prev = '_';
    let mut rules = VecDeque::new();
    for ch in digit_alias.chars() {
        if rules.is_empty() {
            rules.push_back(((prev, ch), ' '));
            continue;
        }
        let mut prew = rules.pop_back().unwrap();
        prew.1 = ch;
        let next = ((prew.0 .1, prew.1), ch);
        rules.push_back(prew);
        rules.push_back(next)
    }
    let prew = rules.pop_back().unwrap();
    let last = (prew.0, goal);
    rules.push_back(last);
    rules.into()
}

fn find_impl(
    input: &str,
    current: char,
    depth: usize,
    rules: &[Rule],
    direction: Direction,
) -> Option<u32> {
    if input.is_empty() {
        return None;
    }
    let (to_be_checked, next_input) = match direction {
        Forward => (input.chars().nth(0), &input[1..]),
        Backward => (input.chars().last(), &input[0..input.len() - 1]),
    };

    let to_be_checked =
        to_be_checked.expect("Expected to_be_checked to be set from non empty input");
    if let result @ Some(_) = to_be_checked.to_digit(10) {
        return result;
    };

    if to_be_checked != current {
        return None;
    }

    let Some(rule) = rules.first() else {
        return None;
    };

    match rule.1 {
        val if val.is_ascii_digit() => val.to_digit(10),
        _ => find_impl(next_input, rule.1, depth + 1, &rules[1..], direction),
    }
}

impl CalibrationDoc<'_> {
    fn convert_fwd(value: &str) -> u32 {
        find(value, Forward).unwrap_or_default()
    }

    fn convert_bkwd(value: &str) -> u32 {
        find(value, Backward).unwrap_or_default()
    }

    fn get_line_calibration_v2(single_line: &str) -> u32 {
        // The most performant solution is to make an automate that parses an input char by char
        // and gives birth for a resulting number and this is the next step of this algorithm evolution
        let mut result = Self::convert_fwd(single_line);
        result = result * 10 + Self::convert_bkwd(single_line);
        result
    }

    fn get_line_calibration_v1(single_line: &str) -> u32 {
        let digit_from_index = |i| single_line.chars().nth(i).expect("Expected index to be");
        let check = |c| char::is_ascii_digit(&c);
        let first = single_line.find(check).map(digit_from_index);
        let last = single_line.rfind(check).map(digit_from_index);
        let two_digit_num = first.iter().chain(last.iter()).collect::<String>();
        two_digit_num.parse().unwrap_or_default()
    }

    fn get_calibration_v1(&self) -> u32 {
        self.0.lines().map(Self::get_line_calibration_v1).sum()
    }

    fn get_calibration_v2(&self) -> u32 {
        self.0.lines().map(Self::get_line_calibration_v2).sum()
    }
}

#[test]
fn test_rules() {
    const R1_0: Rule = (('_', 'o'), 'n');
    const R1_1: Rule = (('o', 'n'), 'e');
    const R1_2: Rule = (('n', 'e'), '1');

    assert_eq!(route("one", '1'), [R1_0, R1_1, R1_2])
}

#[test]
fn test_find() {
    assert_eq!(find("1", Forward), Some(1));
    assert_eq!(find("asdf1", Forward), Some(1));
    assert_eq!(find("1asdf", Backward), Some(1));
    assert_eq!(find("one", Forward), Some(1));
    assert_eq!(find("twone", Backward), Some(1));
    assert_eq!(find("done", Forward), Some(1));
    assert_eq!(find("two", Forward), Some(2));
    assert_eq!(find("twoe", Backward), Some(2));
    assert_eq!(find("dtwo", Forward), Some(2));
    assert_eq!(find("three", Forward), Some(3));
    assert_eq!(find("three", Backward), Some(3));
    assert_eq!(find("adsfthreeasdf", Forward), Some(3));
    assert_eq!(find("aathreeadsf", Backward), Some(3));
    assert_eq!(find("doneasdf", Forward), Some(1));
    assert_eq!(find("eightwo", Backward), Some(2));
    assert_eq!(find("eightwo", Forward), Some(8));
    assert_eq!(find("nineight", Backward), Some(8));

    assert_eq!(find("ddd", Forward), None);
}

#[test]
fn test_convert() {
    assert_eq!(CalibrationDoc::convert_fwd("one"), 1);
    assert_eq!(CalibrationDoc::convert_fwd("two"), 2);
    assert_eq!(CalibrationDoc::convert_fwd("three"), 3);
    assert_eq!(CalibrationDoc::convert_fwd("four"), 4);
    assert_eq!(CalibrationDoc::convert_fwd("five"), 5);
    assert_eq!(CalibrationDoc::convert_fwd("six"), 6);
    assert_eq!(CalibrationDoc::convert_fwd("seven"), 7);
    assert_eq!(CalibrationDoc::convert_fwd("eight"), 8);
    assert_eq!(CalibrationDoc::convert_fwd("nine"), 9);
    assert_eq!(CalibrationDoc::convert_fwd("1"), 1);
    assert_eq!(CalibrationDoc::convert_fwd("2"), 2);
    assert_eq!(CalibrationDoc::convert_fwd("3"), 3);
    assert_eq!(CalibrationDoc::convert_fwd("4"), 4);
    assert_eq!(CalibrationDoc::convert_fwd("5"), 5);
    assert_eq!(CalibrationDoc::convert_fwd("6"), 6);
    assert_eq!(CalibrationDoc::convert_fwd("7"), 7);
    assert_eq!(CalibrationDoc::convert_fwd("8"), 8);
    assert_eq!(CalibrationDoc::convert_fwd("9"), 9);
}

#[test]
fn test_getting_line_calibration() {
    assert_eq!(CalibrationDoc::get_line_calibration_v2(""), 0);
    assert_eq!(CalibrationDoc::get_line_calibration_v2("a"), 0);
    assert_eq!(CalibrationDoc::get_line_calibration_v2("1"), 11);
    assert_eq!(CalibrationDoc::get_line_calibration_v2("9"), 99);
    assert_eq!(CalibrationDoc::get_line_calibration_v2("10"), 10);
    assert_eq!(CalibrationDoc::get_line_calibration_v2("01"), 1);
    assert_eq!(CalibrationDoc::get_line_calibration_v2("abcdefghij"), 0);
    assert_eq!(CalibrationDoc::get_line_calibration_v2("1abcdefghij"), 11);
    assert_eq!(CalibrationDoc::get_line_calibration_v2("abcdefghij1"), 11);
    assert_eq!(CalibrationDoc::get_line_calibration_v2("abcd1efghij"), 11);
    assert_eq!(CalibrationDoc::get_line_calibration_v2("0abcd1efghij"), 1);
    assert_eq!(
        CalibrationDoc::get_line_calibration_v2("523aadsff21345125132sdf9"),
        59
    );

    assert_eq!(CalibrationDoc::get_line_calibration_v2("1abc2"), 12);
    assert_eq!(CalibrationDoc::get_line_calibration_v2("pqr3stu8vwx"), 38);
    assert_eq!(CalibrationDoc::get_line_calibration_v2("a1b2c3d4e5f"), 15);
    assert_eq!(CalibrationDoc::get_line_calibration_v2("treb7uchet"), 77);

    assert_eq!(CalibrationDoc::get_line_calibration_v2("on e"), 0);
    assert_eq!(CalibrationDoc::get_line_calibration_v2("one"), 11);
    assert_eq!(CalibrationDoc::get_line_calibration_v2("onetwothree"), 13);
    assert_eq!(CalibrationDoc::get_line_calibration_v2("0onetwothree"), 3);
    assert_eq!(CalibrationDoc::get_line_calibration_v2("nine"), 99);
    assert_eq!(CalibrationDoc::get_line_calibration_v2("fournine"), 49);
    assert_eq!(
        CalibrationDoc::get_line_calibration_v2("fourthfivefnine"),
        49
    );
    assert_eq!(
        CalibrationDoc::get_line_calibration_v2("4fourthfivefnine"),
        49
    );
    assert_eq!(
        CalibrationDoc::get_line_calibration_v2("7fourthfivefnine"),
        79
    );
    assert_eq!(CalibrationDoc::get_line_calibration_v2("one"), 11);
    assert_eq!(CalibrationDoc::get_line_calibration_v2("two"), 22);
    assert_eq!(CalibrationDoc::get_line_calibration_v2("three"), 33);
    assert_eq!(CalibrationDoc::get_line_calibration_v2("four"), 44);
    assert_eq!(CalibrationDoc::get_line_calibration_v2("five"), 55);
    assert_eq!(CalibrationDoc::get_line_calibration_v2("six"), 66);
    assert_eq!(CalibrationDoc::get_line_calibration_v2("seven"), 77);
    assert_eq!(CalibrationDoc::get_line_calibration_v2("eight"), 88);
    assert_eq!(CalibrationDoc::get_line_calibration_v2("nine"), 99);

    assert_eq!(CalibrationDoc::get_line_calibration_v2("one1"), 11);
    assert_eq!(CalibrationDoc::get_line_calibration_v2("two2"), 22);
    assert_eq!(CalibrationDoc::get_line_calibration_v2("three3"), 33);
    assert_eq!(CalibrationDoc::get_line_calibration_v2("four4"), 44);
    assert_eq!(CalibrationDoc::get_line_calibration_v2("five5"), 55);
    assert_eq!(CalibrationDoc::get_line_calibration_v2("six6"), 66);
    assert_eq!(CalibrationDoc::get_line_calibration_v2("seven7"), 77);
    assert_eq!(CalibrationDoc::get_line_calibration_v2("eight8"), 88);
    assert_eq!(CalibrationDoc::get_line_calibration_v2("nine9"), 99);

    assert_eq!(CalibrationDoc::get_line_calibration_v2("0asdfone"), 1);
    assert_eq!(CalibrationDoc::get_line_calibration_v2("0adsftwo"), 2);
    assert_eq!(CalibrationDoc::get_line_calibration_v2("0adsfthree"), 3);
    assert_eq!(CalibrationDoc::get_line_calibration_v2("0adsffour"), 4);
    assert_eq!(CalibrationDoc::get_line_calibration_v2("0adsffive"), 5);
    assert_eq!(CalibrationDoc::get_line_calibration_v2("0adsfsix"), 6);
    assert_eq!(CalibrationDoc::get_line_calibration_v2("0asdfseven"), 7);
    assert_eq!(CalibrationDoc::get_line_calibration_v2("0adsfeight"), 8);
    assert_eq!(CalibrationDoc::get_line_calibration_v2("0adsfnine"), 9);

    assert_eq!(
        CalibrationDoc::get_line_calibration_v2("2htzsvdhvqvdjv"),
        22
    );
    assert_eq!(
        CalibrationDoc::get_line_calibration_v2("fivetwocrhmvxqkvbeightfive1qzcxvds"),
        51
    );
    assert_eq!(
        CalibrationDoc::get_line_calibration_v2("eightoneqjvzv3"),
        83
    );
    assert_eq!(CalibrationDoc::get_line_calibration_v2("277"), 27);
    assert_eq!(CalibrationDoc::get_line_calibration_v2("five93"), 53);
    assert_eq!(CalibrationDoc::get_line_calibration_v2("eightwo"), 82);

    assert_eq!(
        CalibrationDoc::get_line_calibration_v2("2tqbxgrrpmxqfglsqjkqthree6nhjvbxpflhr1eightwohr"),
        22
    );
}

#[test]
fn test_calibration_doc() {
    assert_eq!(
        CalibrationDoc("3a;sdklfjlaskdj f1").get_calibration_v2(),
        31
    );
    let input = r#"1abc2
                   pqr3stu8vwx
                   a1b2c3d4e5f
                   treb7uchet"#;
    assert_eq!(CalibrationDoc(input).get_calibration_v2(), 142);

    let input = r#"two1nine
                   eightwothree
                   abcone2threexyz
                   xtwone3four
                   4nineeightseven2
                   zoneight234
                   7pqrstsixteen"#;
    assert_eq!(CalibrationDoc(input).get_calibration_v2(), 281);

    let input = include_str!("calibration_doc_1");
    assert_eq!(CalibrationDoc(input).get_calibration_v2(), 572);
    let input = include_str!("calibration_doc_2");
    assert_eq!(CalibrationDoc(input).get_calibration_v2(), 911);

    let input = include_str!("calibration_doc_github");
    assert_eq!(CalibrationDoc(input).get_calibration_v1(), 55208);
    let input = include_str!("calibration_doc_github");
    assert_eq!(CalibrationDoc(input).get_calibration_v2(), 54578);
    let input = include_str!("calibration_doc_huge");
    assert_eq!(CalibrationDoc(input).get_calibration_v1(), 54605);
    let input = include_str!("calibration_doc_huge");
    assert_eq!(CalibrationDoc(input).get_calibration_v2(), 55429);
}
