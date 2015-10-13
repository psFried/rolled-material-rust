

use super::fix_numeric_str;

#[test]
fn simple_integer_should_be_unchanged() {
    let mut number = &mut "123".to_string();
    fix_numeric_str(&mut number);
    assert_eq!("123".to_string(), *number);
}

#[test]
fn decimal_number_should_be_unchanged() {
    let mut number = &mut "123.456".to_string();
    fix_numeric_str(&mut number);
    assert_eq!("123.456".to_string(), *number);
}

#[test]
fn number_with_letter_should_be_truncated_so_only_numeric_chars_remain() {
    let mut number = &mut "123jjk4".to_string();
    fix_numeric_str(&mut number);
    assert_eq!("123".to_string(), *number);
}
