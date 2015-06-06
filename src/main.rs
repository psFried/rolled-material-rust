mod units;

// use units::units;
use units::units::METERS;
use units::units::Length;

const EPSILON: f64 = 0.00001f64;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let out: String = get_output(args);
    println!("{}", out);

}

fn get_output(args: Vec<String>) -> String {
    "not done yet".to_string()
}

// fn format_output(length: f64) -> String {
//
// }

fn calc_total_length(inside_diameter: &Length, outside_diameter: &Length, thickness: &Length) -> Length {
    let mut current_diameter = inside_diameter.convert_to(METERS).value;
    let od_m = outside_diameter.convert_to(METERS).value;
    let thickness_adder = 2f64 * thickness.convert_to(METERS).value;
    let mut length = 0f64;
    while current_diameter <= od_m {
        length += determine_circumfrence(current_diameter);
        current_diameter += thickness_adder;
    }
    Length{value: length, unit: METERS }
}

fn determine_circumfrence(diameter: f64) -> f64 {
    diameter * std::f64::consts::PI
}


#[test]
fn test_determine_circumference() {
    assert_is_within(12.56637, determine_circumfrence(4f64), EPSILON);

}

#[test]
fn test_calc_total_length() {
    let id = Length{value: 0.1f64, unit: METERS};
    let od = Length{value: 1.1f64, unit: METERS};
    let thickness = Length{value: 0.01f64, unit: METERS};

    let result = calc_total_length(&id, &od, &thickness);
    println!("result= {:?}", result);
    println!("do yo uhear me?");

}

fn assert_is_within(actual: f64, expected: f64, epsilon: f64) {
    let diff: f64 = (actual - expected).abs();
    assert!(diff < epsilon)
}
