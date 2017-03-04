extern crate serde;
#[macro_use] extern crate serde_derive;
extern crate serde_json;
extern crate hyper;
extern crate html5ever;
extern crate lettre;
extern crate regex;
extern crate iron;
#[macro_use] extern crate router;

mod menu;
mod monitor;
mod server;
mod util;

pub use monitor::Monitor;
pub use server::run_server;
