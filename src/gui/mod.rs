extern crate conrod;
extern crate find_folder;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate piston;

#[cfg(test)]
mod test;

use std::path::{Path, PathBuf};

use self::glutin_window::GlutinWindow;
use self::opengl_graphics::{GlGraphics, OpenGL};
use self::opengl_graphics::glyph_cache::GlyphCache;
use self::piston::event_loop::{Events, EventLoop};
use self::piston::input::{RenderEvent};
use self::piston::window::{WindowSettings, Size};

use self::conrod::{
    Background,
    Button,
    color,
    Colorable,
    CharacterCache,
    Labelable,
    Label,
    Sizeable,
    Theme,
    Ui,
    Widget,
    NumberDialer,
    Frameable,
    Positionable,
    TextBox,
    WidgetId
};

use estimator;
use estimator::units::{self, LengthUnit, Length, parse_str};


struct AppState {
    thickness_input_value: String,
    thickness_input_unit: LengthUnit,
    od_input_value: String,
    id_input_value: String,
    diameter_inputs_unit: LengthUnit,
    output_value: String,
    output_unit: LengthUnit,
}

impl AppState {

    fn new() -> AppState {
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

    fn get_material_roll(&self) -> Option<estimator::MaterialRoll> {
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

widget_ids!{
    OD_INPUT_LABEL,
    OD_INPUT_FIELD,
    ID_INPUT_FIELD,
    ID_INPUT_LABEL,
    THICKNESS_CONTROL,
    THICKNESS_LABEL,
    DIAMETER_UNIT_DROP_DOWN,
    THICKNESS_UNIT_DROP_DOWN,
    OUTPUT_UNIT_DROP_DOWN,
    OUTPUT_DISPLAY
}


pub fn run() {
    let opengl = OpenGL::V3_2;
    let window: GlutinWindow = WindowSettings::new(
            "Estimate Rolled Material Length".to_string(),
            Size { width: 500, height: 300 }
        ).opengl(opengl)
        .exit_on_esc(true)
        .samples(4)
        .build()
        .unwrap();

    let mut gl = GlGraphics::new(opengl);
    let assets: PathBuf = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets").unwrap();
    let font_path: PathBuf = assets.join("fonts/NotoSans/NotoSans-Regular.ttf");
    let theme = Theme::default();
    let glyph_cache: GlyphCache = GlyphCache::new(&font_path).unwrap();
    let ui = &mut Ui::new(glyph_cache, theme);

    let mut app_state: AppState = AppState::new();

    let event_iter = window.events().ups(180).max_fps(60);
    for event in event_iter {
        ui.handle_event(&event);

        if let Some(args) = event.render_args() {
            gl.draw(args.viewport(), |graphics_context, gl| {
                create_ui(ui, &mut app_state);
                ui.draw_if_changed(graphics_context, gl);
            });
        }
    }
}

fn create_ui<C>(ui: &mut Ui<C>, app_state: &mut AppState)  where C: CharacterCache {
    let vertical_spacing = 40.0;
    let horizontal_pad = 25.0;

    // Set the background color to use for clearing the screen.
    Background::new().rgb(0.3, 0.4, 0.5).set(ui);

    // Seems like you have to manually compute x/y for first widget. This seems broken
    let x: f64 = -(ui.win_w / 2.0) + 110.0;
    let y: f64 = (ui.win_h / 2.0) - vertical_spacing;

    Label::new("Material Thickness")
        .xy(x, y)
        .set(THICKNESS_LABEL, ui);

    TextBox::new(&mut app_state.thickness_input_value)
        .react(fix_numeric_str)
        .right_from(THICKNESS_LABEL, horizontal_pad)
        .align_middle_y()
        .set(THICKNESS_CONTROL, ui);

    Label::new("Outside Diameter")
        .down_from(THICKNESS_LABEL, vertical_spacing)
        .align_right()
        .set(OD_INPUT_LABEL, ui);

    TextBox::new(&mut app_state.od_input_value)
        .react(fix_numeric_str)
        .right_from(OD_INPUT_LABEL, horizontal_pad)
        .align_middle_y()
        .set(OD_INPUT_FIELD, ui);

    Label::new("Inside Diameter")
        .down_from(OD_INPUT_LABEL, vertical_spacing)
        .align_right()
        .set(ID_INPUT_LABEL, ui);

    TextBox::new(&mut app_state.id_input_value)
        .react(fix_numeric_str)
        .right_from(ID_INPUT_LABEL, horizontal_pad)
        .align_middle_y()
        .set(ID_INPUT_FIELD, ui);

}

#[allow(unused_variables)]
fn fix_numeric_str(input: &mut String) {
    let number_base = 10;

    let invalid_char: Option<(usize, char)> = input.char_indices().find(|&(idx, ch)| {
        !(ch.is_digit(number_base) || ch == '.')
        });

    match invalid_char {
        Some((idx, ch)) => input.truncate(idx),
        None => {}
    };
}
