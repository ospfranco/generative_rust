use config::PARTICLE_COUNT;
use log::{warn, LevelFilter};
use nannou::noise::NoiseFn;
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
        .size(540, 960)
        .update(update)
        .simple_window(view)
        .run();
}

struct Particle {
    pos: Vec2,
    vel: Vec2,
}

impl Particle {
    fn new(x: f32, y: f32) -> Particle {
        Particle {
            pos: vec2(x, y),
            vel: vec2(0., 0.),
        }
    }

    fn update(&mut self, dir: Vec2) {
        self.pos += self.vel;
        self.vel += dir;
        self.vel *= 0.8;
    }
}

struct Model {
    particles: Vec<Particle>,
}

fn model(app: &App) -> Model {
    let r = app.window_rect().right() as f32;
    let l = app.window_rect().left() as f32;

    let w = l - r;
    let t = app.window_rect().top() as f32;
    let b = app.window_rect().bottom() as f32;

    let h = t - b;

    let mut p = vec![];
    for _i in 0..PARTICLE_COUNT {
        let x = random_f32() * w + r;
        let y = random_f32() * h + b;
        p.push(Particle::new(x, y));
    }

    Model { particles: p }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let noise = nannou::noise::Perlin::new();
    let t = app.elapsed_frames() as f64 / 100.;
    for i in 0..model.particles.len() {
        let p = &mut model.particles[i];
        let x = noise.get([
            p.pos.x as f64 / 128.,
            p.pos.y as f64 / 137.,
            t + i as f64 / 1000.,
        ]);
        let y = noise.get([
            -p.pos.y as f64 / 128.,
            p.pos.x as f64 / 137.,
            t + i as f64 / 1000.,
        ]);

        let a = vec2(x as f32, y as f32);
        p.update(a);
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    // draw.background().color(BLACK);
    for p in &model.particles {
        draw.ellipse()
            .xy(p.pos)
            .w_h(1.0, 1.0)
            .color(hsla(0.1, 1., 5., 0.01));
    }
    draw.to_frame(app, &frame).unwrap();
    capture::capture(app, frame)
}
