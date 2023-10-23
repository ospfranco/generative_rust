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
        .size(config::WIDTH, config::HEIGHT)
        .update(update)
        .simple_window(view)
        .run();
}

struct Particle {
    pos: Vec2,
    last_pos: Vec2,
    vel: Vec2,
}

impl Particle {
    fn new(x: f32, y: f32) -> Particle {
        Particle {
            pos: vec2(x, y),
            last_pos: vec2(x, y),
            vel: vec2(0., 0.),
        }
    }

    fn update(&mut self, dir: Vec2) {
        self.last_pos = self.pos;
        self.pos += self.vel;
        self.vel += dir;
        self.vel *= 0.6;
    }
}

struct Model {
    particles: Vec<Particle>,
    color_angle: f32,
}

fn model(app: &App) -> Model {
    let window = app.window_rect();
    let r = window.right();
    let l = window.left();

    let w = l - r;
    let t = window.top();
    let b = window.bottom();

    let h = t - b;

    warn!("{} {}", w, window.w());

    let mut p = vec![];
    for _i in 0..PARTICLE_COUNT {
        let x = random_f32() * w + r;
        let y = random_f32() * h + b;
        p.push(Particle::new(x, y));
    }

    Model {
        particles: p,
        color_angle: 0.575,
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let noise = nannou::noise::Perlin::new();
    let elapsed_frames = app.elapsed_frames() as f64;
    let t = elapsed_frames / 100.;
    for i in 0..model.particles.len() {
        let p = &mut model.particles[i];
        let mut x = noise.get([
            p.pos.x as f64 / 128.,
            p.pos.y as f64 / 137.,
            t + i as f64 / 1000.,
        ]) as f32;
        let mut y = noise.get([
            -p.pos.y as f64 / 128.,
            p.pos.x as f64 / 137.,
            t + i as f64 / 1000.,
        ]) as f32;

        if elapsed_frames % 20. == 0. {
            if x < 0. {
                x -= random_f32();
            } else {
                x += random_f32();
            }

            if y < 0. {
                y -= random_f32();
            } else {
                y += random_f32();
            }
        }

        let dir = vec2(x, y);
        p.update(dir);
    }

    model.color_angle += 0.001;
    model.color_angle = model.color_angle % 1.0;
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    if frame.nth() == 0 {
        frame.clear(BLACK);
    } else {
        draw.rect()
            .w_h(config::WIDTH as f32, config::HEIGHT as f32)
            .x_y(0., 0.)
            .color(rgba(0., 0., 0., 0.01));
    }

    for p in &model.particles {
        draw.line()
            .start(p.last_pos)
            .end(p.pos)
            .color(hsla(model.color_angle, 1., 0.68, 0.1));
    }
    draw.to_frame(app, &frame).unwrap();
    capture::capture(app, frame)
}
