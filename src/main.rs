extern crate rice_servery;

use std::env;
use std::path::Path;
use rice_servery::{Monitor, run_server};

const USAGE: &'static str = "Usage: rice_servery <command>
Possible commands:
1. monitor [config_file]
Start the servery monitor. A custom configuration file can be provided as an
argument, otherwise \"config.json\" will be used.

2. server
Start the servery API server.
";

fn main() {
    let mut args = env::args().skip(1);
    let action = match args.next() {
        Some(a) => a,
        None => {
            println!("No command specified.");
            println!("{}", USAGE);
            return;
        }
    };

    match action.as_str() {
        "monitor" => {
            let filename = match args.next() {
                Some(p) => p,
                None => "config.json".to_string(),
            };
            
            let path = Path::new(&filename);

            let mut monitor = Monitor::from_file(&path);
            monitor.process();
        }
        "server" => {
            run_server();
        }
        _ => {
            println!("Unknown command.");
            println!("{}", USAGE);
        }
    }

}