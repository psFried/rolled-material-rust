

use super::*;

use std::f64::consts::PI;


#[test]
fn calc_total_length_should_return_length_of_roll_for_one_layer_thick() {
    let roll = MaterialRoll{
        id: Length::new(4.0, INCHES),
        od: Length::new(5.0, INCHES),
        thickness: Length::new(1.0, INCHES)
    };

    let result = roll.get_roll_length();
    let expected = 4.0 * PI;

    assert_is_within(result.value(&INCHES), expected, EPSILON);
}

#[test]
fn get_roll_length_should_return_length_of_multi_layer_roll() {
    let roll: MaterialRoll = MaterialRoll{
        id: Length::new(4.0, INCHES),
        od: Length::new(6.0, INCHES),
        thickness: Length::new(1.0, INCHES)
    };

    let result = roll.get_roll_length();
    let expected = (4.0 * PI) + (5.0 * PI);
    assert_is_within(result.value(&INCHES), expected, EPSILON);
}



fn assert_is_within(actual: f64, expected: f64, epsilon: f64) {
    let diff: f64 = (actual - expected).abs();
    assert!(diff < epsilon, format!("Expected {} to be within {} of {}", actual, epsilon, expected))
}
