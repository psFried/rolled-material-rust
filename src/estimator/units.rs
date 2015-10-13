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

#[derive(Debug, PartialEq, Clone)]
pub struct Length {
    pub value: f64,
    pub unit: LengthUnit
}

impl Length {

    pub fn format(&self) -> String {
        format!("{:.2}{}", self.value, self.unit.abbrev())
    }

    pub fn convert_to(&self, unit: LengthUnit) -> Length {
        let value_as_reference = self.unit.value_to_reference(self.value);
        let converted_value = unit.value_from_reference(value_as_reference);
        return Length { value: converted_value, unit: unit };
    }

}

pub fn parse_str(input: &str, unit: LengthUnit) -> Option<Length> {
    let trimmed_input = input.trim();
    let value_opt = trimmed_input.parse::<f64>().ok();
    value_opt.map(|val| Length{value: val, unit: unit})
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
    println!("length in cm= {}, in meters= {}", len_cm.value, len_inches.convert_to(METERS).value);
    assert_equals(expected, len_cm.value, eps);
}

#[test]
fn test_length_format() {
    let len = Length{value: 40.0f64, unit: INCHES};

    assert_eq!("40.00in".to_string(), len.format());

    let len2 = Length{value: 77.333333333333f64, unit: METERS };
    assert_eq!("77.33m".to_string(), len2.format());
}

pub fn assert_equals(expected: f64, actual: f64, epsilon: f64) {
    assert!(actual <= (expected + epsilon));
    assert!(actual >= (expected - epsilon));
}
