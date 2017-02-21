use std::io::{Write, stderr};
use hyper::client::Response;
use html5ever::parse_document;
use html5ever::rcdom::{RcDom, Handle, Text, Element, Normal};
use html5ever::tendril::TendrilSink;

use super::config::Config;
use super::util::map_attrs;

pub struct Parser {
    response: Response,
    servery_title: Option<String>,
    in_menu_item: bool,
}

impl Parser {
    pub fn new(res: Response) -> Self {
        Parser {
            response: res,
            servery_title: None,
            in_menu_item: false,
        }
    }

    pub fn parse(&mut self, config: &Config) {
        let parsed = parse_document(RcDom::default(), Default::default())
            .from_utf8()
            .read_from(&mut self.response);

        match parsed {
            Ok(dom) => self.walk(dom.document, config),
            Err(e) => {
                writeln!(stderr(), "Error parsing menu: {}", e).unwrap();
                return;
            }
        };
    }

    fn walk(&mut self, handle: Handle, config: &Config) {
        let node = handle.borrow();
        match node.node {
            Text(ref text) => {
                if self.in_menu_item {
                    if self.is_match(text, config) {
                        let servery = self.servery_title.clone().unwrap_or("Unknown".to_string());
                        println!("Found {} at {}", text, servery);
                    }

                    self.in_menu_item = false;
                }
            }
            Element(_, ref etype, ref attrs) => {
                match etype {
                    &Normal => {
                        let attr_map = map_attrs(attrs);
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
            self.walk(child.clone(), config);
        }
    }

    fn is_match(&self, item: &str, config: &Config) -> bool {
        let lower = &item.to_lowercase();
        for rule in config.rules.iter() {
            if lower.contains(&rule.keyword.to_lowercase()) {
                return true;
            }
        }

        false
    }
}