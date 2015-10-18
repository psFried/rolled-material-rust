pub mod units;

#[cfg(test)]
mod test;

use std::f64::consts::PI;

pub use self::units::*;

pub const EPSILON: f64 = 0.000016f64;

pub struct MaterialRoll {
    pub id: Length,
    pub od: Length,
    pub thickness: Length
}

impl MaterialRoll {

    pub fn get_roll_length(&self) -> Length {
        let unit: LengthUnit = CENTIMETERS;
        let mut id_val = self.id.value(&unit);
        let od = self.od.value(&unit);
        let thickness = self.thickness.value(&unit);
        let mut length: f64 = 0.0;

        while id_val < od {
            length += id_val * PI;
            id_val += thickness;
        }

        Length::new(length, unit)
    }
}
