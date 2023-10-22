use nannou::prelude::*;
use log::{warn, LevelFilter};

mod logger;

const DEBUG_LOGGING: bool = false;

static LOGGER: logger::SimpleLogger = logger::SimpleLogger { enabled: DEBUG_LOGGING };

fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .run();
}

struct Model {
}

fn model(_app: &App) -> Model {

    let _ = log::set_logger(&LOGGER)
        .map(|()| log::set_max_level(LevelFilter::Warn));

    Model {}
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
}

fn view(app: &App, _model: &Model, frame: Frame){
    // get canvas to draw on
    let draw = app.draw();

    // set background to blue
    draw.background().color(BLACK);

    // Generate sine wave data based on the time of the app
    let sine = app.time.sin();
    let slowersine = (app.time / 2.0).sin();

    // Get boundary of the window (to constrain the movements of our circle)
    let boundary = app.window_rect();

    // Map the sine wave functions to ranges between the boundaries of the window
    let x: f32 = map_range(sine, -1.0, 1.0, boundary.left(), boundary.right());
    let y: f32 = map_range(slowersine, -1.0, 1.0, boundary.bottom(), boundary.top());

    // Draw a blue ellipse at the x/y coordinates 0.0, 0.0
    draw.ellipse().color(STEELBLUE).x_y(x, y).w_h(100f32, 100f32);

    warn!("{}", sine.to_string());

    draw.to_frame(app, &frame).unwrap();
}