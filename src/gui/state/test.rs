
use ::estimator::units::{INCHES, Length};
use super::InputState;


#[test]
fn app_state_should_return_material_roll_with_parsed_lengths() {
    let app_state: InputState = InputState::new();
    let roll_option = app_state.get_material_roll();
    assert!(roll_option.is_some());

    let material_roll = roll_option.unwrap();
    assert_eq!(Length::new(0.05, INCHES), material_roll.thickness);
    assert_eq!(Length::new(4.0, INCHES), material_roll.id);
    assert_eq!(Length::new(20.0, INCHES), material_roll.od);

}

#[test]
fn app_state_get_material_roll_should_return_none_if_id_input_is_invalid() {
    let mut app_state = InputState::new();
    app_state.id_input_value = "j/k".to_string();
    let roll_option = app_state.get_material_roll();

    assert!(roll_option.is_none());
}

#[test]
fn app_state_get_material_roll_should_return_none_if_od_input_is_invalid() {
    let mut app_state = InputState::new();
    app_state.od_input_value = "j/k".to_string();
    let roll_option = app_state.get_material_roll();

    assert!(roll_option.is_none());
}

#[test]
fn app_state_get_material_roll_should_return_none_if_thickness_input_is_invalid() {
    let mut app_state = InputState::new();
    app_state.thickness_input_value = "j/k".to_string();
    let roll_option = app_state.get_material_roll();

    assert!(roll_option.is_none());
}
