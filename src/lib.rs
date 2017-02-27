#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate hyper;
extern crate html5ever;
extern crate lettre;
extern crate regex;

mod menu;
mod monitor;
mod util;

pub use monitor::Monitor;
