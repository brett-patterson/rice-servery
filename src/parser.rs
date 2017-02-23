use std::io::{Write, stderr};
use std::collections::HashMap;

use hyper::client::Response;
use html5ever::parse_document;
use html5ever::rcdom::{RcDom, Handle, Text, Element, Normal};
use html5ever::tendril::TendrilSink;
use regex::Regex;

use super::alerts::alert;
use super::config::{Config, Rule};
use super::util::map_attrs;

const MEAL_PATTERN: &'static str = "^Your (\\w+), Today:$";

/// A match found by the parser.
#[derive(Debug)]
struct Match {
    item: String,
    servery: String,
}

/// The parser used to parse the HTML response of the servery menus.
pub struct Parser<'a> {
    response: Response,
    meal: Option<String>,
    servery_title: Option<String>,
    in_menu_item: bool,
    matches: HashMap<&'a Rule, Vec<Match>>,
}

impl<'a> Parser<'a> {
    /// Construct a new Parser from an HTTP response.
    pub fn new(res: Response) -> Self {
        Parser {
            response: res,
            meal: None,
            servery_title: None,
            in_menu_item: false,
            matches: HashMap::new(),
        }
    }

    /// Parse the response with a given configuration and send alerts as found.
    pub fn parse(&mut self, config: &'a Config) {
        let parsed = parse_document(RcDom::default(), Default::default())
            .from_utf8()
            .read_from(&mut self.response);

        match parsed {
            Ok(dom) => {
                self.walk(dom.document, config);

                for (rule, matches) in &self.matches {
                    let meal = self.meal.clone().unwrap_or("Unknown".to_string());
                    let title = format!("Found {} for {}", rule.keyword, meal);
                    let body = matches.iter()
                        .map(|m| format!("{} at {}", m.item, m.servery))
                        .fold("".to_string(),
                              |acc, m| if acc.len() == 0 { m } else { acc + "\n" + &m });

                    alert(&title, &body, rule, config);
                }
            }
            Err(e) => {
                writeln!(stderr(), "Error parsing menu: {}", e).unwrap();
                return;
            }
        };
    }

    /// The recursive function used to visit each node in the parsed DOM.
    fn walk(&mut self, handle: Handle, config: &'a Config) {
        let node = handle.borrow();
        match node.node {
            Text(ref text) => {
                let meal_regex = Regex::new(MEAL_PATTERN).unwrap();
                if let Some(caps) = meal_regex.captures(text) {
                    self.meal = caps.get(1).map(|c| c.as_str().to_string());
                }

                if self.in_menu_item {
                    self.check_match(text, config);
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

    /// Check if a menu item matches a rule, and if so add it to the set
    /// of matches.
    fn check_match(&mut self, item: &str, config: &'a Config) -> bool {
        let lower = &item.to_lowercase();
        for rule in config.rules.iter() {
            if lower.contains(&rule.keyword.to_lowercase()) {
                let servery = self.servery_title.clone().unwrap_or("Unknown".to_string());
                if !self.matches.contains_key(&rule) {
                    self.matches.insert(&rule, Vec::new());
                }

                self.matches.get_mut(&rule).map(|v| {
                    v.push(Match {
                        item: item.to_string(),
                        servery: servery,
                    })
                });
            }
        }

        false
    }
}