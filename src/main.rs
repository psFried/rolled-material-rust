mod units;

const EPSILON: f64 = 0.00001f64;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let out: String = get_output(args);
    println!("{}", out);

}

fn get_output(args: Vec<String>) -> String {
    if args.len() == 4 {
        let inside_diameter_mm = args[1].parse::<f64>().ok().unwrap();
        let outside_diameter_mm = args[2].parse::<f64>().ok().unwrap();
        let thickness_mm = args[3].parse::<f64>().ok().unwrap();

        let length = calc_total_length_meters(inside_diameter_mm / 2f64, outside_diameter_mm / 2f64, thickness_mm);
        println!("length= {}", length);
        format!("{}", length).to_string()
    } else {
        "must supply 3 arguments".to_string()
    }
}

// fn format_output(length: f64) -> String {
//
// }

fn calc_total_length_meters(inside_radius_mm: f64, outside_radius_mm: f64, thickness_mm: f64) -> f64 {
    let mut current_radius = inside_radius_mm;

    let mut total_length_mm = 0f64;

    while current_radius < outside_radius_mm {
        total_length_mm += determine_circumfrence(current_radius);
        current_radius += thickness_mm;
    }

    total_length_mm / 1000f64
}

fn determine_circumfrence(diameter: f64) -> f64 {
    diameter * std::f64::consts::PI
}


#[test]
fn test_determine_circumference() {
    assert_is_within(12.56637, determine_circumfrence(4f64), EPSILON);

}

#[test]
fn test_calc_total_length_meters() {
    // let len = &units::units::Length { value: 5f64, unit: units::units::Meters()};

}

fn assert_is_within(actual: f64, expected: f64, epsilon: f64) {
    let diff: f64 = (actual - expected).abs();
    assert!(diff < epsilon)
}
