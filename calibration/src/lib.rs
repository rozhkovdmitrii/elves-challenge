#[cfg(test)]
struct CalibrationDoc<'a>(&'a str);

#[cfg(test)]
impl CalibrationDoc<'_> {
    fn get_line_calibration(single_line: &str) -> u64 {
        let digit_from_index = |i| single_line.chars().nth(i).expect("Expected index to be");
        let check = |c| char::is_ascii_digit(&c);
        let first = single_line.find(check).map(digit_from_index);
        let last = single_line.rfind(check).map(digit_from_index);
        let two_digit_num = first.iter().chain(last.iter()).collect::<String>();
        two_digit_num.parse().unwrap_or_default()
    }

    fn get_calibration(&self) -> u64 {
        self.0.lines().map(Self::get_line_calibration).sum()
    }
}

#[test]
fn test_getting_line_calibration() {
    assert_eq!(CalibrationDoc::get_line_calibration(""), 0);
    assert_eq!(CalibrationDoc::get_line_calibration("a"), 0);
    assert_eq!(CalibrationDoc::get_line_calibration("1"), 11);
    assert_eq!(CalibrationDoc::get_line_calibration("9"), 99);
    assert_eq!(CalibrationDoc::get_line_calibration("10"), 10);
    assert_eq!(CalibrationDoc::get_line_calibration("01"), 1);
    assert_eq!(CalibrationDoc::get_line_calibration("abcdefghij"), 0);
    assert_eq!(CalibrationDoc::get_line_calibration("1abcdefghij"), 11);
    assert_eq!(CalibrationDoc::get_line_calibration("abcdefghij1"), 11);
    assert_eq!(CalibrationDoc::get_line_calibration("abcd1efghij"), 11);
    assert_eq!(CalibrationDoc::get_line_calibration("0abcd1efghij"), 1);
    assert_eq!(
        CalibrationDoc::get_line_calibration("523aadsff21345125132sdf9"),
        59
    );

    assert_eq!(CalibrationDoc::get_line_calibration("1abc2"), 12);
    assert_eq!(CalibrationDoc::get_line_calibration("pqr3stu8vwx"), 38);
    assert_eq!(CalibrationDoc::get_line_calibration("a1b2c3d4e5f"), 15);
    assert_eq!(CalibrationDoc::get_line_calibration("treb7uchet"), 77);
}

#[test]
fn test_calibration_doc() {
    assert_eq!(CalibrationDoc("3a;sdklfjlaskdj f1").get_calibration(), 31);
    let input = r#"1abc2
                 pqr3stu8vwx
                 a1b2c3d4e5f
                 treb7uchet"#;
    assert_eq!(CalibrationDoc(input).get_calibration(), 142);
    let input = include_str!("calibration_doc");
    assert_eq!(CalibrationDoc(input).get_calibration(), 54605);
}
