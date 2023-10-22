use log::{warn, LevelFilter};
use nannou::{color, prelude::*};

mod logger;

const DEBUG_LOGGING: bool = true;
const RADIUS: i32 = 20;
const DIAMETER: i32 = RADIUS * 2;
const X_STEP: i32 = 1;
const COLUMNS: i32 = 100;

static LOGGER: logger::SimpleLogger = logger::SimpleLogger {
    enabled: DEBUG_LOGGING,
};

fn main() {
    let _ = log::set_logger(&LOGGER).map(|()| log::set_max_level(LevelFilter::Warn));

    nannou::app(model)
        .size(600, 1000)
        .update(update)
        .simple_window(view)
        .run();
}

struct Ball {
    pos: (i32, i32),
    initial_transparency: f32,
}

impl Ball {
    fn set_x(&mut self, x: i32) {
        self.pos.0 = x;
    }
}

struct Model {
    balls: Vec<Ball>,
}

fn model(app: &App) -> Model {
    let window = app.window_rect();
    let top_left = window.top_left();
    let rows = (window.h() / DIAMETER as f32).ceil() as i32 + 1;

    let mut balls = Vec::with_capacity((rows * COLUMNS) as usize);
    for i in 0..=rows {
        for j in 0..=COLUMNS {
            let should_appear = random_f32() > 0.4;

            if should_appear {
                balls.push(Ball {
                    pos: (
                        top_left.x as i32 + RADIUS + DIAMETER * j,
                        top_left.y as i32 - RADIUS - DIAMETER * i,
                    ),
                    initial_transparency: random_f32(),
                });
            }
        }
    }

    Model { balls }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    for i in 0..model.balls.len() {
        let ball = &mut model.balls[i];

        ball.set_x(ball.pos.0 - X_STEP);
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(BLACK);

    for ball in model.balls.iter() {
        let sine = (app.time + ball.initial_transparency * 2.0 * PI).sin();
        let transparency = map_range(sine, -1.0, 1.0, 0.0, 1.0);

        let stroke_color = color::rgba(1.0, 1.0, 1.0, transparency);
        // let y = (transparency * 100.0).round() / 100.0;
        draw.ellipse()
            .stroke_weight(0.5)
            .stroke(stroke_color)
            .color(BLACK)
            .x_y(ball.pos.0 as f32, ball.pos.1 as f32)
            .w_h(DIAMETER as f32, DIAMETER as f32);
        // draw.text(&y.to_string()).x_y(ball.pos.0 as f32, ball.pos.1 as f32);
    }

    draw.ellipse()
        .stroke_weight(1.0)
        .stroke(WHITE)
        .color(BLACK)
        .x_y(0.0, 0.0)
        .w_h(200.0, 200.0);

    draw.to_frame(app, &frame).unwrap();
}
