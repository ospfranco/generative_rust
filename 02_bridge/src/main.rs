use nannou::{prelude::*, color};
use log::{warn, LevelFilter};
use rand::Rng;

mod logger;

const DEBUG_LOGGING: bool = true;

static LOGGER: logger::SimpleLogger = logger::SimpleLogger { enabled: DEBUG_LOGGING };

fn main() {
    nannou::app(model)
        .size(600, 1000)
        .update(update)
        .simple_window(view)
        .run();
}

struct Model {}

fn model(app: &App) -> Model {
    let _ = log::set_logger(&LOGGER)
        .map(|()| log::set_max_level(LevelFilter::Warn));
    let mut rng = rand::thread_rng();
    

    Model {
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {

}

fn view(app: &App, model: &Model, frame: Frame){
    let draw = app.draw();

    draw.background().color(WHITE);

    draw.to_frame(app, &frame).unwrap();
}