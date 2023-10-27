use config::PARTICLE_COUNT;
#[allow(unused_imports)]
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
    original_pos: Vec2,
    sin_offset: f32,
    pos: Vec2,
    last_pos: Vec2,
    vel: Vec2,
    exit_frame: u64,
    collision_frame: u64,
    collision_end: u64,
}

impl Particle {
    fn new(x: f32, y: f32) -> Particle {
        let exit_frame = (random_f32() * 80000.) as u64;
        let collision_frame = exit_frame + (random_f32() * 600.) as u64 + 500;
        Particle {
            original_pos: vec2(x, y),
            sin_offset: random_f32(),
            pos: vec2(x, y),
            last_pos: vec2(x, y),
            vel: vec2(0., 0.),
            exit_frame,
            collision_frame,
            collision_end: collision_frame + (random_f32() + 100.) as u64,
        }
    }

    fn update(&mut self, time: f32, frame: u64, x: f32, y: f32) {
        if frame < self.exit_frame {
            self.last_pos = self.pos;
            self.pos = self.original_pos
                + vec2(
                    (time + self.sin_offset * 10.).sin() * 5.,
                    (time + self.sin_offset * 10.).cos() * 2. - self.sin_offset,
                );
        } else if frame < self.collision_frame {
            self.last_pos = self.pos;
            self.pos += self.vel;
            self.vel += vec2(x, y);
            self.vel *= 0.6;
        }
    }

    fn reset(&mut self, base_frame: u64) {
        let theta = random_f32() * 2. * PI;

        let x = config::RADIUS * theta.cos();
        let y = config::RADIUS * theta.sin();

        self.pos = vec2(x, y);
        self.last_pos = vec2(x, y);
        self.vel = vec2(0., 0.);
        self.exit_frame = (random_f32() * 30000.) as u64 + base_frame;
        self.collision_frame = self.exit_frame + (random_f32() * 1000.) as u64 + 300;
        self.collision_end = self.collision_frame + (random_f32() + 100.) as u64
    }
}

struct Model {
    particles: Vec<Particle>,
    tree: wgpu::Texture,
    tree_inverted: wgpu::Texture,
}

fn model(app: &App) -> Model {
    let groups = [
        // left
        vec2(-60., 85.),
        vec2(-40., 82.),
        vec2(-35., 75.),
        // mid lef
        vec2(-30., 40.),
        vec2(-35., 43.),
        vec2(-35., 50.),
        // mid right
        vec2(-0., 40.),
        // top
        vec2(-32., 90.),
        vec2(-30., 100.),
        vec2(0., 105.),
        vec2(-20., 103.),
        vec2(-20., 110.),
        vec2(-15., 95.),
        // right
        vec2(0., 95.),
        vec2(6., 64.),
        vec2(20., 60.),
        vec2(18., 70.),
        vec2(25., 80.),
        vec2(40., 70.),
    ];

    // Set up tree images
    let assets = app.assets_path().unwrap();
    let tree_path = assets.join("tree.png");
    let tree_inverted_path = assets.join("tree_inverted.png");
    let tree = wgpu::Texture::from_path(app, tree_path).unwrap();
    let tree_inverted = wgpu::Texture::from_path(app, tree_inverted_path).unwrap();

    let mut p = vec![];
    for _i in 0..PARTICLE_COUNT {
        let group_index = (random_f32() * (groups.len() - 1) as f32).round();
        let group = groups[group_index as usize];

        let r = 15.;
        let tetha = random_f32() * 2. * PI;
        let x = group.x + r * random_f32() * tetha.cos();
        let y = group.y + r * random_f32() * tetha.sin() * 0.5;
        p.push(Particle::new(x, y));
    }

    Model {
        particles: p,
        tree,
        tree_inverted,
    }
}

fn is_out_of_frame(frame: Rect, pos: Vec2) -> bool {
    (pos.x < frame.left() || pos.x > frame.right())
        && (pos.y < frame.bottom() || pos.y > frame.top())
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let window = app.window_rect();
    let frame = app.elapsed_frames();
    let t = frame as f64 / 100.;
    let noise = nannou::noise::Perlin::new();

    for i in 0..model.particles.len() {
        let p = &mut model.particles[i];

        // let x = noise.get([p.pos.x, p.pos.y, t + i]);
        let x = noise.get([
            p.pos.x as f64 / 128.,
            p.pos.y as f64 / 137.,
            t + i as f64 / 1000.,
        ]) as f32;
        let y = noise.get([
            -p.pos.y as f64 / 128.,
            p.pos.x as f64 / 137.,
            t + i as f64 / 1000.,
        ]) as f32;

        if is_out_of_frame(window, p.pos) {
            p.reset(frame);
        } else {
            p.update(app.time, frame, x, -y.abs());
        }
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    let window = app.window_rect();
    let frame_count = frame.nth();

    if frame_count == 0 {
        frame.clear(WHITE)
    }

    draw.texture(&model.tree).y(45.).w_h(100., 100.);
    draw.texture(&model.tree_inverted).y(-20.).w_h(100., 40.);

    draw.line()
        .color(rgba(0., 0., 0., 0.01))
        .start(vec2(window.left(), 0.))
        .end(vec2(window.right(), 0.));

    draw.rect()
        .w_h(config::WIDTH as f32, config::HEIGHT as f32)
        .x_y(0., 0.)
        .color(rgba(1., 1., 1., 0.1));

    for p in &model.particles {
        if frame_count < p.collision_frame {
            draw.line().start(p.last_pos).end(p.pos).color(BLACK);
        } else if frame_count < p.collision_end {
            let radius = 10.
                - (p.collision_end - frame_count) as f32
                    / (p.collision_end - p.collision_frame) as f32
                    * 10.;
            draw.ellipse()
                .x_y(p.pos.x, p.pos.y)
                .w_h(radius * 2., radius)
                .color(rgba(0., 0., 0., 0.))
                .stroke_weight(0.5)
                .stroke(rgba(0., 0., 0., 0.05));
        }
    }

    draw.to_frame(app, &frame).unwrap();

    capture::capture(app, frame)
}
