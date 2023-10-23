use nannou::{App, Frame};

use crate::config;

fn captured_frame_path(app: &App, frame: &Frame) -> std::path::PathBuf {
    app.project_path()
        .expect("failed to locate `project_path`")
        .join("frames")
        .join(format!("{:04}", frame.nth()))
        .with_extension("png")
}

pub fn capture(app: &App, frame: Frame) {
    if frame.nth() < config::CAPTURE_FRAMES && config::CAPTURE_OUTPUT {
        let file_path = captured_frame_path(app, &frame);
        app.main_window().capture_frame(file_path);
    }
}
