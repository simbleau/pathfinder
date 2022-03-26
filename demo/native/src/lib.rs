#[macro_use]
extern crate lazy_static;

#[cfg(all(target_os = "macos", not(feature = "pf-gl")))]
extern crate objc;

mod renderer;
use renderer::*;

use pathfinder_demo::{window::DataPath, BackgroundColor, DemoApp, Mode, Options, UIVisibility};
use std::{path::PathBuf, time::Duration};
pub struct PathfinderRenderer {
    app: Option<DemoApp<WindowImpl>>,
}

impl PathfinderRenderer {
    pub fn new() -> Self {
        PathfinderRenderer { app: None }
    }

    pub fn init<P>(&mut self, path: P) -> ()
    where
        P: Into<PathBuf>,
    {
        let datapath = DataPath::Path(path.into());

        // Read command line options.
        let options = Options {
            jobs: None,
            mode: Mode::TwoD,
            input_path: datapath,
            ui: UIVisibility::None,
            background_color: BackgroundColor::Light,
            high_performance_gpu: true,
            renderer_level: None,
            hidden_field_for_future_proofing: (),
        };

        self.app = Some(renderer::init(options));
    }

    pub fn render(&mut self, frames: usize) -> Vec<Duration> {
        let mut app = self.app.take().unwrap();
        let durs = renderer::run(frames, &mut app);
        app.should_exit = true;
        app.window.quit();
        drop(app);
        durs
    }
}

#[cfg(test)]
#[test]
fn run_100_frames() {
    let mut p = PathfinderRenderer::new();
    p.init("/home/simbleau/git/vgpu-bench/assets/svg/examples/ASU.svg");
    let durs = p.render(100);
    println!("Durs: {:?}", durs);
}
