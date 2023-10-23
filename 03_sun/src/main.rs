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
    exit_frame: u64,
    angle: f32,
}

impl Particle {
    fn new(x: f32, y: f32, angle: f32) -> Particle {
        Particle {
            pos: vec2(x, y),
            last_pos: vec2(x, y),
            vel: vec2(0., 0.),
            exit_frame: (random_f32() * 500.) as u64,
            angle,
        }
    }

    fn update(&mut self, frame: u64) {
        self.last_pos = self.pos;
        let aux = self.pos;
        self.pos += self.vel;

        let r = (self.pos.x * self.pos.x + self.pos.y * self.pos.y).sqrt();

        let mut limit = 1;
        while frame >= self.exit_frame + 20 * limit {
            limit += 1;
        }

        if limit % 2 == 0 {
            self.angle -= 0.005;
        } else {
            self.angle += 0.005;
        }

        let x = r * self.angle.cos();
        let y = r * self.angle.sin();
        self.pos = vec2(x, y);

        self.vel += aux;
        self.vel *= 0.005;
    }

    fn reset(&mut self, base_frame: u64) {
        let theta = random_f32() * 2. * PI;

        let x = config::RADIUS * theta.cos();
        let y = config::RADIUS * theta.sin();

        self.pos = vec2(x, y);
        self.last_pos = vec2(x, y);
        self.angle = theta;
        self.vel = vec2(0., 0.);
        self.exit_frame = (random_f32() * 500.) as u64 + base_frame;
    }
}

struct Model {
    particles: Vec<Particle>,
}

fn model(_app: &App) -> Model {
    let mut p = vec![];
    for _i in 0..PARTICLE_COUNT {
        let theta = random_f32() * 2. * PI;

        let x = config::RADIUS * theta.cos();
        let y = config::RADIUS * theta.sin();
        p.push(Particle::new(x, y, theta));
    }

    Model { particles: p }
}

fn is_out_of_frame(frame: Rect, pos: Vec2) -> bool {
    (pos.x < frame.left() || pos.x > frame.right())
        && (pos.y < frame.bottom() || pos.y > frame.top())
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let window = app.window_rect();
    let frame = app.elapsed_frames();

    for i in 0..model.particles.len() {
        let p = &mut model.particles[i];

        if is_out_of_frame(window, p.pos) {
            p.reset(frame);
        } else if frame >= p.exit_frame {
            p.update(frame);
        }
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    if frame.nth() == 0 {
        frame.clear(WHITE)
    }

    // draw.background().color(WHITE);
    draw.rect()
        .w_h(config::WIDTH as f32, config::HEIGHT as f32)
        .x_y(0., 0.)
        .color(rgba(1., 1., 1., 0.1));

    for p in &model.particles {
        draw.line()
            .start(p.last_pos)
            .end(p.pos)
            // .weight(4.)
            .color(hsla(0., 0., 0., 0.5));
    }

    draw.to_frame(app, &frame).unwrap();
    capture::capture(app, frame)
}
