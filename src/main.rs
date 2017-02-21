extern crate servery_monitor;

use std::path::Path;
use servery_monitor::ServeryMonitor;

fn main() {
    let path = Path::new("config.json");
    let mut monitor = ServeryMonitor::from_file(&path);
    monitor.process();
}