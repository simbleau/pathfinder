#[macro_use]
extern crate lazy_static;

#[cfg(all(target_os = "macos", not(feature = "pf-gl")))]
extern crate objc;

mod renderer;
use renderer::*;

use pathfinder_demo::{DemoApp, Options};
use std::time::Duration;
pub struct PathfinderRenderer {
    app: Option<DemoApp<WindowImpl>>,
}

impl PathfinderRenderer {
    pub fn new() -> Self {
        PathfinderRenderer { app: None }
    }

    pub fn init(&mut self) -> () {
        // Read command line options.
        let mut options = Options::default();
        options.command_line_overrides();

        self.app = Some(renderer::init(options));
    }

    pub fn render(&mut self, frames: usize) -> Vec<Duration> {
        let app = self.app.as_mut().unwrap();
        let durs = renderer::run(frames, app);
        durs
    }
}
