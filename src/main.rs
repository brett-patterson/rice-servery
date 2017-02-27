extern crate rice_servery;

use std::path::Path;
use rice_servery::Monitor;

fn main() {
    let path = Path::new("config.json");
    let mut monitor = Monitor::from_file(&path);
    monitor.process();
}