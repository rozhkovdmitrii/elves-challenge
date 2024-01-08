This is an implementation of [Advent of Code](https://adventofcode.com/2023/day/1)

It provides the CalibrationDoc abstraction, which is designed to retrieve a calibration value from a document consisting of lines of text. Each of them initially contains a certain calibration value

Getting the calibration value could be performed by calling `get_line_calibration_v2` method

## Specific features of realization

Current implementation uses a generated parser that operates on the input string due to be calibrated.
This one surprisingly shown more than 5-8 times acceleration in comparison of using dynamic pattern matching trees that
has been used in the previous version 

### Example of generated parser

Generated parser under the hood looks as follows:

```rust
pub fn look_for_digit_forward<T>(mut input: T) -> Option<u64>
where
    T: Iterator<Item = char>,
{
    match input.next()? {
        '0' => Some(0u64),
        ...
        's' => match input.next()? {
            'e' => match input.next()? {
                'v' => match input.next()? {
                    'e' => match input.next()? {
                        'n' => Some(7u64),
                        _ => None,
                    },
                    _ => None,
                },
                _ => None,
            },
            'i' => match input.next()? {
                'x' => Some(6u64),
                _ => None,
            },
            _ => None,
        },
        ...
    }
}
```