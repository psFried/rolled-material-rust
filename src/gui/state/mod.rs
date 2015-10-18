#[cfg(test)]
mod test;

use estimator;
use estimator::units::{self, LengthUnit, Length, parse_str};


pub struct AppState {
    pub thickness_input_value: String,
    pub thickness_input_unit: LengthUnit,
    pub od_input_value: String,
    pub id_input_value: String,
    pub diameter_inputs_unit: LengthUnit,
    pub output_value: String,
    pub output_unit: LengthUnit,
}

impl AppState {

    pub fn new() -> AppState {
        const UNIT: LengthUnit = units::INCHES;
        let thickness_val = 0.05;
        let od_val = 20.0;
        let id_val = 4.0;

        AppState {
            thickness_input_value: format!("{:.2}", thickness_val).to_string(),
            thickness_input_unit: UNIT,
            od_input_value: format!("{:.2}", od_val).to_string(),
            id_input_value: format!("{:.2}", id_val).to_string(),
            diameter_inputs_unit: units::INCHES,
            output_value: "##.##".to_string(),
            output_unit: UNIT
        }
    }

    pub fn get_material_roll(&self) -> Option<estimator::MaterialRoll> {
        let lengths: Option<(Length, Length, Length)> = units::parse_str(&self.thickness_input_value, self.thickness_input_unit.clone())
            .and_then(|thickness| { units::parse_str(&self.id_input_value, self.diameter_inputs_unit.clone()).map(|id| { (thickness, id) }) })
            .and_then(|(thickness, id)| { units::parse_str(&self.od_input_value, self.diameter_inputs_unit.clone())
                .map(|od| { (thickness, id, od) })
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
