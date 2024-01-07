use parse_digit_gen::gen_digit_parser;

gen_digit_parser! {
    look_for_digit_forward,
    "one" -> 1,
    "two" -> 2,
    "three" -> 3,
    "four" -> 4,
    "five" -> 5,
    "six" -> 6,
    "seven" -> 7,
    "eight" -> 8,
    "nine" -> 9
}

gen_digit_parser! {
    look_for_digit_backward,
    "eno" -> 1,
    "owt" -> 2,
    "eerht" -> 3,
    "ruof" -> 4,
    "evif" -> 5,
    "xis" -> 6,
    "neves" -> 7,
    "thgie" -> 8,
    "enin" -> 9
}
