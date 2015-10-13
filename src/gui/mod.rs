extern crate conrod;
extern crate find_folder;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate piston;

mod test;

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
    output_unit: LengthUnit
}

impl AppState {

    fn new() -> AppState {
        AppState {
            thickness_input_value: "0.05".to_string(),
            thickness_input_unit: units::INCHES,
            od_input_value: "20".to_string(),
            id_input_value: "4".to_string(),
            diameter_inputs_unit: units::INCHES,
            output_value: "##.##".to_string(),
            output_unit: units::INCHES
        }
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

    let event_iter = window.events().ups(180).max_fps(60);
    let mut gl = GlGraphics::new(opengl);
    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets").unwrap();
    let font_path = assets.join("fonts/NotoSans/NotoSans-Regular.ttf");
    let theme = Theme::default();
    let glyph_cache = GlyphCache::new(&font_path).unwrap();
    let ui = &mut Ui::new(glyph_cache, theme);

    let mut app_state: AppState = AppState::new();

    for event in event_iter {
        ui.handle_event(&event);

        if let Some(args) = event.render_args() {
            gl.draw(args.viewport(), |graphics_context, gl| {

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


                ui.draw_if_changed(graphics_context, gl);

            });
        }

    }


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
