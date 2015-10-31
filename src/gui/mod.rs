extern crate conrod;
extern crate find_folder;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate piston;
extern crate input;

mod state;

#[cfg(test)]
mod test;

use self::state::InputState;

use std::path::{Path, PathBuf};

use self::glutin_window::GlutinWindow;
use self::opengl_graphics::{GlGraphics, OpenGL};
use self::opengl_graphics::glyph_cache::GlyphCache;
use self::piston::event_loop::{Events, EventLoop};
use self::piston::input::{RenderEvent};
use self::piston::window::{WindowSettings, Size};
use self::conrod::WidgetIndex;


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
    // Construct the window.
    // let window: PistonWindow = WindowSettings::new("Click me!", [200, 100])
    //     .exit_on_esc(true).build().unwrap();
    //
    // // construct our `Ui`.
    // let mut ui = {
    //     let assets = find_folder::Search::ParentsThenKids(3, 3)
    //         .for_folder("assets").unwrap();
    //     let font_path = assets.join("fonts/NotoSans/NotoSans-Regular.ttf");
    //     let theme = Theme::default();
    //     let glyph_cache = Glyphs::new(&font_path, window.factory.borrow().clone());
    //     Ui::new(glyph_cache.unwrap(), theme)
    // };

    println!("thickness= {:?}, od= {:?}, id= {:?}", THICKNESS_CONTROL, OD_INPUT_FIELD, ID_INPUT_FIELD);

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

    let mut app_state: InputState = InputState::new();

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

fn create_ui<C>(ui: &mut Ui<C>, app_state: &mut InputState)  where C: CharacterCache {
    let vertical_spacing = 40.0;
    let horizontal_pad = 25.0;

    app_state.focus_next.take();
    let mut focus_next: Option<WidgetId> = None;

    // Set the background color to use for clearing the screen.
    Background::new().rgb(0.3, 0.4, 0.5).set(ui);

    // Seems like you have to manually compute x/y for first widget. This seems broken
    let x: f64 = -(ui.win_w / 2.0) + 110.0;
    let y: f64 = (ui.win_h / 2.0) - vertical_spacing;

    Label::new("Material Thickness")
        .xy(x, y)
        .set(THICKNESS_LABEL, ui);

    TextBox::new(&mut app_state.thickness_input_value)
        .react(|new_val: &mut String| {
            fix_numeric_str(new_val);
            focus_next = Some(OD_INPUT_FIELD);
        })
        .right_from(THICKNESS_LABEL, horizontal_pad)
        .align_middle_y()
        .set(THICKNESS_CONTROL, ui);

    Label::new("Outside Diameter")
        .down_from(THICKNESS_LABEL, vertical_spacing)
        .align_right()
        .set(OD_INPUT_LABEL, ui);

    TextBox::new(&mut app_state.od_input_value)
        .react(|new_val: &mut String| {
            fix_numeric_str(new_val);
            focus_next = Some(ID_INPUT_FIELD);
        })
        .right_from(OD_INPUT_LABEL, horizontal_pad)
        .align_middle_y()
        .set(OD_INPUT_FIELD, ui);

    Label::new("Inside Diameter")
        .down_from(OD_INPUT_LABEL, vertical_spacing)
        .align_right()
        .set(ID_INPUT_LABEL, ui);

    TextBox::new(&mut app_state.id_input_value)
        .react(|new_val: &mut String| {
            fix_numeric_str(new_val);
            focus_next = Some(THICKNESS_CONTROL);
        })
        .right_from(ID_INPUT_LABEL, horizontal_pad)
        .align_middle_y()
        .set(ID_INPUT_FIELD, ui);

    app_state.focus_next = focus_next;

    let output_length = app_state.get_material_roll()
        .map(|roll| {
            roll.get_roll_length().convert_to(app_state.output_unit.clone()).format()
        }).unwrap_or_else(|| {
        "##.##".to_string()
    });

    Label::new(&format!("Total Length: {}", &output_length))
        .font_size(32)
        .down_from(ID_INPUT_LABEL, vertical_spacing)
        .align_left()
        .set(OUTPUT_DISPLAY, ui);

    if let Some(widget_id) = focus_next {
        ui.change_focus_to(widget_id);
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
