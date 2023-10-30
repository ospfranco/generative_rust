use log::warn;
#[allow(unused_variables)]
#[allow(unused_imports)]
use log::{info, LevelFilter};
use nannou::prelude::*;
mod capture;
mod config;
mod logger;

static LOGGER: logger::SimpleLogger = logger::SimpleLogger {
    enabled: config::DEBUG_LOGGING,
};

fn main() {
    let _ = log::set_logger(&LOGGER).map(|()| log::set_max_level(LevelFilter::Warn));

    nannou::app(model)
        .size(config::WIDTH, config::HEIGHT)
        .update(update)
        .simple_window(view)
        .run();
}

struct Model {
    x: f32,
    y: f32,
    w: f32,
    h: f32,
}

impl Clone for Model {
    fn clone(&self) -> Self {
        Model {
            x: self.x,
            y: self.y,
            w: self.w,
            h: self.h,
        }
    }
}

impl Model {
    fn update(&mut self, x: f32, y: f32, w: f32, h: f32) {
        self.x += x;
        self.y += y;
        self.w = w;
        self.h = h;
    }

    fn update_w_h(&mut self, w: f32, h: f32) {
        self.w = w;
        self.h = h;
    }
}

fn model(_app: &App) -> Model {
    Model {
        x: 0.,
        y: 0.,
        w: 4.,
        h: 8.,
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    if (config::TARGET_X - model.x).abs() <= 0.00001
        && (config::TARGET_Y - model.y).abs() <= 0.00001
    {
        model.update_w_h(
            model.w * config::SCALE_FACTOR,
            model.h * config::SCALE_FACTOR,
        );
    } else {
        model.update(
            (config::TARGET_X - model.x) * 0.005,
            (config::TARGET_Y - model.y) * 0.005,
            model.w * config::SCALE_FACTOR,
            model.h * config::SCALE_FACTOR,
        );
    }
}

fn calculate_point(model: &Model, x: i32, y: i32, step: f32, max_iteration: i32) -> i32 {
    let yp = (model.x - model.w / 2.) + x as f32 * step;
    let xp = (model.y - model.h / 2.) + y as f32 * step;

    let mut iteration = 0;
    let mut xi = 0.;
    let mut yi = 0.;
    while xi * xi + yi * yi <= 4. && iteration < max_iteration {
        let xtemp = xi * xi - yi * yi + xp as f64;
        yi = 2. * xi * yi + yp as f64;

        xi = xtemp;

        iteration += 1
    }

    iteration
}

fn calculate_region(
    model: Model,
    x0: i32,
    size: i32,
    step: f32,
    h: i32,
    max_iteration: i32,
) -> Vec<Vec<i32>> {
    let mut res = vec![vec![0; h as usize]; size as usize];
    let mut x = 0;
    while x < size as i32 {
        let mut y = 0;
        while y < h {
            res[x as usize][y as usize] = calculate_point(&model, x0 + x, y, step, max_iteration);
            y += 1;
        }
        x += 1;
    }

    res
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    let window = app.window_rect();
    let nth = frame.nth();

    frame.clear(BLACK);

    let bottom_left = window.bottom_left();

    let x0 = bottom_left.x;
    let y0 = bottom_left.y;
    let w = window.w();
    let h = window.h() as i32;

    let max_iteration = std::cmp::min(100 + nth, 2000) as i32;
    let step = model.w / w;

    let region_size = config::WIDTH as i32 / config::THREAD_COUNT;

    let threads: Vec<_> = (0..config::THREAD_COUNT)
        .map(|i| {
            let temp_model = model.clone();
            std::thread::spawn(move || {
                calculate_region(
                    temp_model,
                    region_size * i,
                    region_size,
                    step,
                    h,
                    max_iteration,
                )
            })
        })
        .collect();

    for (i, thread) in threads.into_iter().enumerate() {
        let res = thread.join().unwrap();

        let dx = region_size as f32 * i as f32;

        let mut x = 0;
        while x < region_size {
            let mut y = 0;
            while y < h {
                let v = res[x as usize][y as usize];

                if v == max_iteration {
                    draw.rect()
                        .w_h(1., 1.)
                        .x_y(x as f32 + x0 + dx, y as f32 + y0)
                        .color(BLACK);
                } else {
                    let gray_percentage = v as f32 / max_iteration as f32;
                    let tetha = map_range(gray_percentage, 0., 1., 0.721, 0.9);
                    let lightness = map_range(gray_percentage, 0., 1., 0., 0.5);
                    draw.rect()
                        .w_h(1., 1.)
                        .x_y(x as f32 + x0 + dx, y as f32 + y0)
                        .color(hsl(tetha, 1.0, lightness));
                }

                y += 1;
            }
            x += 1;
        }
    }

    draw.to_frame(app, &frame).unwrap();

    capture::capture(app, frame);

    warn!("Frame {}", nth);

    if config::CAPTURE_OUTPUT && nth == config::CAPTURE_FRAMES {
        std::process::exit(0);
    }
}
