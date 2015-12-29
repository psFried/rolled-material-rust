#[cfg(test)]
mod test;

use estimator;
use estimator::units::{self, Unit, LengthUnit, Length, parse_str};
use super::conrod::WidgetId;


pub struct InputState {
    pub thickness_input_value: String,
    pub od_input_value: String,
    pub id_input_value: String,
    pub valid_units: Vec<LengthUnit>,
    pub selected_unit: Option<usize>
}

impl InputState {

    pub fn new() -> InputState {
        const UNIT: LengthUnit = units::INCHES;
        let thickness_val = 0.08;
        let od_val = 12.0;
        let id_val = 4.0;

        InputState {
            thickness_input_value: format!("{:.2}", thickness_val).to_string(),
            od_input_value: format!("{:.2}", od_val).to_string(),
            id_input_value: format!("{:.2}", id_val).to_string(),
            valid_units: vec![units::CENTIMETERS, units::INCHES],
            selected_unit: Some(0)
        }
    }

    pub fn get_input_unit(&self) -> LengthUnit {
        self.valid_units[self.selected_unit.unwrap_or(0)].clone()
    }

    pub fn get_output_unit(&self) -> LengthUnit {
        units::YARDS
    }

    pub fn get_input_unit_strings(&self) -> Vec<String> {
        self.valid_units.iter().map(|unit| unit.full_name().to_string()).collect::<Vec<String>>()
    }

    pub fn get_material_roll(&self) -> Option<estimator::MaterialRoll> {
        let zero: Length = Length::zero();

        let lengths: Option<(Length, Length, Length)> = units::parse_str(&self.thickness_input_value, self.get_input_unit())
            .and_then(|thickness| { units::parse_str(&self.id_input_value, self.get_input_unit()).map(|id| { (thickness, id) }) })
            .and_then(|(thickness, id)| { units::parse_str(&self.od_input_value, self.get_input_unit())
                .map(|od| { (thickness, id, od) })
            }).and_then(|(thickness, id, od)| {
                if thickness > zero &&
                        id > zero &&
                        od > zero &&
                        od > id {
                    Some((thickness, id, od))
                } else {
                    None
                }
            });

        lengths.map(|(thickness, id, od)| {
            estimator::MaterialRoll{
                id: id,
                od: od,
                thickness: thickness
            }
        })
    }

}
