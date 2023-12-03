pub mod creator_page;
pub mod homepage;
pub mod post;

use askama::{Result, Template};
use tokio::time::Instant;

pub trait TimedRender {
    fn timed_render(&self) -> Result<String>;
}

impl<T: Template> TimedRender for T {
    fn timed_render(&self) -> Result<String> {
        let render_start = Instant::now();
        let html = self.render();
        println!("Rendered in {}us", render_start.elapsed().as_micros());
        return html;
    }
}
