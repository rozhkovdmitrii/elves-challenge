use once_cell::sync::Lazy;

pub(super) type Check = ((char, char), char);
pub(super) type Route = Vec<Check>;

fn build_route(digit_alias: &str, goal: char) -> Route {
    let prev = '_';
    let mut rules = vec![];
    for ch in digit_alias.chars() {
        if rules.is_empty() {
            rules.push(((prev, ch), ' '));
            continue;
        }
        let mut prew = rules.pop().unwrap();
        prew.1 = ch;
        let next = ((prew.0 .1, prew.1), ch);
        rules.push(prew);
        rules.push(next)
    }
    let prew = rules.pop().unwrap();
    let last = (prew.0, goal);
    rules.push(last);
    rules
}

pub(super) static FWD_ROUTES: Lazy<Vec<Route>> = Lazy::new(|| {
    vec![
        build_route("0", '0'),
        build_route("one", '1'),
        build_route("two", '2'),
        build_route("three", '3'),
        build_route("four", '4'),
        build_route("five", '5'),
        build_route("six", '6'),
        build_route("seven", '7'),
        build_route("eight", '8'),
        build_route("nine", '9'),
    ]
});

pub(super) static BKW_ROUTES: Lazy<Vec<Route>> = Lazy::new(|| {
    vec![
        build_route("0", '0'),
        build_route("eno", '1'),
        build_route("owt", '2'),
        build_route("eerht", '3'),
        build_route("ruof", '4'),
        build_route("evif", '5'),
        build_route("xis", '6'),
        build_route("neves", '7'),
        build_route("thgie", '8'),
        build_route("enin", '9'),
    ]
});

#[test]
fn test_route() {
    const R1_0: Check = (('_', 'o'), 'n');
    const R1_1: Check = (('o', 'n'), 'e');
    const R1_2: Check = (('n', 'e'), '1');
    assert_eq!(build_route("one", '1'), [R1_0, R1_1, R1_2])
}
