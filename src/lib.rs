#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate hyper;
extern crate html5ever;

use std::path::Path;
use std::fs::File;
use std::io::{Write, stderr};
use std::collections::HashMap;
use hyper::client::{Client, Response};
use html5ever::{parse_document, Attribute};
use html5ever::rcdom::{RcDom, Handle, Text, Element, Normal};
use html5ever::tendril::TendrilSink;

const MENU_URL: &'static str = "http://dining.rice.edu/";

#[derive(Deserialize, Debug)]
struct Config {
    rules: Vec<Rule>,
}

#[derive(Deserialize, Debug)]
pub struct Rule {
    keyword: String,
}

pub struct ServeryMonitor {
    config: Config,
    client: Client,
    servery_title: Option<String>,
    in_menu_item: bool,
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
            servery_title: None,
            in_menu_item: false,
        }
    }

    pub fn process(&mut self) {
        match self.fetch_menu() {
            Ok(mut res) => {
                let parsed = parse_document(RcDom::default(), Default::default())
                    .from_utf8()
                    .read_from(&mut res);

                match parsed {
                    Ok(dom) => self.walk(dom.document),
                    Err(e) => {
                        writeln!(stderr(), "Error parsing menu: {}", e).unwrap();
                        return;
                    }
                };
            }
            Err(e) => {
                writeln!(stderr(), "{}", e).unwrap();
            }
        }
    }

    fn walk(&mut self, handle: Handle) {
        let node = handle.borrow();
        match node.node {
            Text(ref text) => {
                if self.in_menu_item {
                    if self.is_match(text) {
                        let servery = self.servery_title.clone().unwrap_or("Unknown".to_string());
                        println!("Found {} at {}", text, servery);
                    }

                    self.in_menu_item = false;
                }
            }
            Element(_, ref etype, ref attrs) => {
                match etype {
                    &Normal => {
                        let attr_map = self.map_attrs(attrs);
                        match attr_map.get("class") {
                            Some(&"servery-title") => {
                                self.servery_title = attr_map.get("id").map(|s| (*s).to_string());
                            }
                            Some(&"menu-item") => {
                                self.in_menu_item = true;
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        }

        for child in node.children.iter() {
            self.walk(child.clone());
        }
    }

    fn is_match(&self, item: &str) -> bool {
        let lower = &item.to_lowercase();
        for rule in self.config.rules.iter() {
            if lower.contains(&rule.keyword.to_lowercase()) {
                return true;
            }
        }

        false
    }

    fn map_attrs<'a>(&self, attrs: &'a Vec<Attribute>) -> HashMap<&'a str, &'a str> {
        let mut map = HashMap::new();
        for attr in attrs.iter() {
            map.insert(&*attr.name.local, &*attr.value);
        }
        map
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