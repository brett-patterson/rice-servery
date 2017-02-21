#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate hyper;
extern crate html5ever;

mod config;
mod parser;
mod util;

use std::path::Path;
use std::fs::File;
use std::io::{Write, stderr};
use hyper::client::{Client, Response};

use config::Config;
use parser::Parser;

const MENU_URL: &'static str = "http://dining.rice.edu/";

pub struct ServeryMonitor {
    config: Config,
    client: Client,
}

impl ServeryMonitor {
    pub fn from_file(path: &Path) -> Self {
        let file = match File::open(path) {
            Ok(file) => file,
            Err(e) => panic!("Error reading {}: {}", path.display(), e),
        };

        let config: Config = match serde_json::from_reader(&file) {
            Ok(cfg) => cfg,
            Err(e) => panic!("Error parsing {}: {}", path.display(), e),
        };

        ServeryMonitor {
            config: config,
            client: Client::new(),
        }
    }

    pub fn process(&mut self) {
        match self.fetch_menu() {
            Ok(res) => {
                let mut parser = Parser::new(res);
                parser.parse(&self.config);
            }
            Err(e) => {
                writeln!(stderr(), "{}", e).unwrap();
            }
        }
    }

    fn fetch_menu(&self) -> Result<Response, String> {
        let res = match self.client.get(MENU_URL).send() {
            Ok(r) => r,
            Err(e) => {
                return Err(format!("Could not fetch menu at {}: {}", MENU_URL, e));
            }
        };

        if res.status != hyper::Ok {
            Err(format!("Unable to fetch menu at {}. Status: {}",
                        MENU_URL,
                        res.status))
        } else {
            Ok(res)
        }
    }
}