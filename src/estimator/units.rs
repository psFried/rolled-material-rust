use std::ops::{Add, Sub, Mul, Div};

pub trait Unit {
    fn abbrev(&self) -> &'static str;
    fn value_to_reference(&self, value: f64) -> f64;
    fn value_from_reference(&self, ref_value: f64) -> f64;
    fn full_name(&self) -> &'static str;
}

#[derive(Debug, PartialEq, Clone)]
pub struct LengthUnit {
    full_name: &'static str,
    abbrev: &'static str,
    factor_to_reference: f64
}

impl Unit for LengthUnit {

    fn abbrev(&self) -> &'static str {
        self.abbrev
    }

    fn full_name(&self) -> &'static str {
        self.full_name
    }

    fn value_to_reference(&self, value: f64) -> f64 {
        return self.factor_to_reference * value;
    }

    fn value_from_reference(&self, value: f64) -> f64 {
        return value / self.factor_to_reference;
    }

}

pub const METERS: LengthUnit = LengthUnit{ full_name: "Meters", abbrev: "m", factor_to_reference: 1.0f64 };
pub const CENTIMETERS: LengthUnit = LengthUnit{ full_name: "Centimeters", abbrev: "cm", factor_to_reference: 0.01f64 };

pub const INCHES: LengthUnit = LengthUnit{ full_name: "Inches", abbrev: "in", factor_to_reference: 0.0254f64 };
pub const YARDS: LengthUnit = LengthUnit{ full_name: "Yards", abbrev: "yrd", factor_to_reference: 0.9144f64 };

#[derive(Debug, PartialEq, Clone)]
pub struct Length {
    pub value: f64,
    pub unit: LengthUnit
}

impl Length {

    pub fn new(value: f64, unit: LengthUnit) -> Length {
        Length{value: value, unit: unit}
    }

    pub fn format(&self) -> String {
        format!("{:.2} {}", self.value, self.unit.abbrev())
    }

    pub fn convert_to(&self, unit: LengthUnit) -> Length {
        let converted_value: f64 = self.value(&unit);
        return Length { value: converted_value, unit: unit };
    }

    pub fn value(&self, unit: &LengthUnit) -> f64 {
        if self.unit == *unit {
            self.value
        } else {
            let value_as_reference = self.unit.value_to_reference(self.value);
            unit.value_from_reference(value_as_reference)
        }
    }

}

impl Add for Length {
    type Output = Length;

    fn add(self, other: Length) -> Length {
        Length::new(self.value + other.value(&self.unit), self.unit)
    }
}

impl Sub for Length {
    type Output = Length;

    fn sub(self, other: Length) -> Length {
        Length::new(self.value - other.value(&self.unit), self.unit)
    }
}

impl Div<f64> for Length {
    type Output = Length;

    fn div(self, divisor: f64) -> Length {
        Length::new(self.value / divisor, self.unit)
    }
}

impl Div for Length {
    type Output = Length;

    fn div(self, other: Length) -> Length {
        Length::new(self.value / other.value(&self.unit), self.unit)
    }
}

impl Mul<f64> for Length {
    type Output = Length;

    fn mul(self, other: f64) -> Length {
        Length::new(self.value * other, self.unit)
    }
}

pub fn parse_str(input: &str, unit: LengthUnit) -> Option<Length> {
    let trimmed_input = input.trim();
    let value_opt = trimmed_input.parse::<f64>().ok();
    value_opt.map(|val| Length{value: val, unit: unit})
}


#[cfg(test)]
mod test {
    use super::*;

    const EPSILON: f64 = 0.00001;

    #[test]
    fn dividing_a_length_by_a_length_should_return_a_length() {
        let l1 = Length::new(100.0, CENTIMETERS);
        let l2 = Length::new(0.5, METERS);
        let result = l1 / l2;
        assert_eq!(CENTIMETERS, result.unit);
        assert_equals(2.0, result.value, EPSILON);
    }

    #[test]
    fn dividing_a_length_by_a_number_should_return_a_length() {
        let l1 = Length::new(10.0, YARDS);
        let result: Length = l1 / 5.0;
        assert_eq!(YARDS, result.unit);
        assert_equals(2.0, result.value, EPSILON);
    }

    #[test]
    fn subtracting_a_greater_length_from_a_smaller_one_should_yield_a_negative_length() {
        let l1 = Length::new(1.0, YARDS);
        let l2 = Length::new(1.0, METERS);
        let result = l1 - l2;
        assert_eq!(YARDS, result.unit);
        assert_equals(-0.09361, result.value, EPSILON);
    }

    #[test]
    fn subtracting_lengths_should_return_correct_value_when_units_are_different() {
        let l1 = Length::new(10.0, METERS);
        let l2 = Length::new(150.0, CENTIMETERS);
        let result = l1 - l2;
        assert_eq!(METERS, result.unit);
        assert_equals(8.5, result.value, EPSILON);
    }

    #[test]
    fn adding_lengths_should_return_correct_sum_when_units_are_the_same() {
        let l1 = Length::new(5.0, INCHES);
        let l2 = l1.clone();
        let result: Length = l1 + l2;
        assert_eq!(INCHES, result.unit);
        assert_equals(10.0, result.value, EPSILON);
    }

    #[test]
    fn adding_lengths_should_return_correct_sum_when_units_are_different() {
        let l1 = Length::new(5.0, INCHES);
        let l2 = Length::new(2.54, CENTIMETERS);
        let result: Length = l1 + l2;
        assert_eq!(INCHES, result.unit);
        assert_equals(6.0, result.value, EPSILON);
    }

    #[test]
    fn test_parse() {
        let fifty_cm = parse_str("50", CENTIMETERS);
        assert!(fifty_cm.is_some());
        let len = fifty_cm.unwrap();
        assert_eq!(len.unit, CENTIMETERS);
        assert_equals(50.0f64, len.value, 0.0001f64);
    }

    #[test]
    fn test_unit_conversion() {
        let len_inches = Length{value: 66.11f64, unit: INCHES};
        let len_cm = len_inches.convert_to(CENTIMETERS);
        let eps = 0.0001f64;
        let expected = 167.9194f64;
        assert_equals(expected, len_cm.value, eps);
    }

    #[test]
    fn length_as_other_unit_should_return_correct_value() {
        let one_inch = Length::new(1.0, INCHES);
        assert_equals(2.54, one_inch.value(&CENTIMETERS), 0.01);
    }

    #[test]
    fn formatting_length_should_return_a_human_readable_string() {
        let len = Length{value: 40.0f64, unit: INCHES};

        assert_eq!("40.00 in".to_string(), len.format());

        let len2 = Length{value: 77.333333333333f64, unit: METERS };
        assert_eq!("77.33 m".to_string(), len2.format());
    }

    pub fn assert_equals(expected: f64, actual: f64, epsilon: f64) {
        let is_within_epsilon = (actual <= (expected + epsilon)) &&
            actual >= (expected - epsilon);
        assert!(is_within_epsilon, format!("Expected {:?} to equal {:?} within {:?}", actual, expected, epsilon));
    }
}
