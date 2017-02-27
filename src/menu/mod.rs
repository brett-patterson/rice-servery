mod parser;

use std::collections::HashMap;
use hyper;
use hyper::client::Client;
use self::parser::parse;

const MENU_URL: &'static str = "http://dining.rice.edu/";

/// A menu for a servery.
#[derive(Debug)]
pub struct Menu {
    pub meal: String,
    pub items_by_servery: HashMap<String, Vec<String>>,
}

impl Menu {
    /// Fetch and parse the menu from the default url.
    pub fn fetch() -> Result<Self, String> {
        Menu::with(MENU_URL)
    }

    /// Fetch and parse the menu from a given url.
    pub fn with(url: &str) -> Result<Self, String> {
        let client = Client::new();
        let res = match client.get(url).send() {
            Ok(r) => r,
            Err(e) => {
                return Err(format!("Could not fetch menu at {}: {}", url, e));
            }
        };

        if res.status != hyper::Ok {
            Err(format!("Unable to fetch menu at {}. Status: {}", url, res.status))
        } else {
            match parse(res) {
                Some(menu) => Ok(menu),
                None => Err(format!("Unable to parse menu at {}", url)),
            }
        }
    }
}