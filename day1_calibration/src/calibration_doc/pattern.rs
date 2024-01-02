use once_cell::sync::Lazy;
use std::collections::BTreeMap;

pub(super) enum Pattern {
    Result(u64),
    Check(BTreeMap<char, Pattern>),
}

impl Pattern {
    fn with(mut self, digit: &str, value: u64) -> Pattern {
        Self::extend_pattern(digit, value, &mut self);
        self
    }

    fn extend_pattern(digit: &str, value: u64, pattern: &mut Pattern) {
        let Some(first) = digit.chars().nth(0) else {
            *pattern = Pattern::Result(value);
            return;
        };
        match pattern {
            Pattern::Result(_) => panic!("Unexpected case while constructing new pattern route"),
            Pattern::Check(ref mut rules) => {
                let mut pattern = rules
                    .entry(first)
                    .or_insert_with(|| Pattern::Check([].into()));
                Self::extend_pattern(&digit[1..], value, &mut pattern);
            }
        };
    }

    fn dec_radix_based() -> Self {
        Pattern::default()
            .with("0", 0)
            .with("1", 1)
            .with("2", 2)
            .with("3", 3)
            .with("4", 4)
            .with("5", 5)
            .with("6", 6)
            .with("7", 7)
            .with("8", 8)
            .with("9", 9)
    }
}

impl Default for Pattern {
    fn default() -> Self {
        Pattern::Check(BTreeMap::default())
    }
}

pub(super) static FWD_PATTERNS: Lazy<Pattern> = Lazy::new(|| {
    Pattern::dec_radix_based()
        .with("one", 1)
        .with("two", 2)
        .with("three", 3)
        .with("four", 4)
        .with("five", 5)
        .with("six", 6)
        .with("seven", 7)
        .with("eight", 8)
        .with("nine", 9)
});

pub(super) static BKW_PATTERNS: Lazy<Pattern> = Lazy::new(|| {
    Pattern::dec_radix_based()
        .with("eno", 1)
        .with("owt", 2)
        .with("eerht", 3)
        .with("ruof", 4)
        .with("evif", 5)
        .with("xis", 6)
        .with("neves", 7)
        .with("thgie", 8)
        .with("enin", 9)
});
