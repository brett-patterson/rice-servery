use std::collections::HashMap;

use hyper::client::Response;
use html5ever::parse_document;
use html5ever::rcdom::{RcDom, Handle, Text, Element, Normal};
use html5ever::tendril::TendrilSink;
use regex::Regex;

use menu::Menu;
use util::map_attrs;

const MEAL_PATTERN: &'static str = "^Your (\\w+), Today:$";

/// The public parser entry point.
pub fn parse(res: Response) -> Option<Menu> {
    let parser = Parser::new(res);
    parser.parse()
}

/// The parser used to parse the HTML response of the servery menus.
struct Parser {
    response: Response,
    meal: Option<String>,
    servery_title: Option<String>,
    items_by_servery: HashMap<String, Vec<String>>,
    in_menu_item: bool,
}

impl Parser {
    /// Construct a new Parser from an HTTP response.
    fn new(res: Response) -> Self {
        Parser {
            response: res,
            meal: None,
            servery_title: None,
            items_by_servery: HashMap::new(),
            in_menu_item: false,
        }
    }

    /// Parse the content of the response.
    fn parse(mut self) -> Option<Menu> {
        let parsed = parse_document(RcDom::default(), Default::default())
            .from_utf8()
            .read_from(&mut self.response);

        match parsed {
            Ok(dom) => {
                self.walk(dom.document);
                if let Some(meal) = self.meal {
                    Some(Menu {
                        meal: meal,
                        items_by_servery: self.items_by_servery,
                    })
                } else {
                    None
                }
            }
            Err(_) => None,
        }
    }

    /// The recursive function used to visit each node in the parsed DOM.
    fn walk(&mut self, handle: Handle) {
        let node = handle.borrow();
        match node.node {
            Text(ref text) => {
                let meal_regex = Regex::new(MEAL_PATTERN).unwrap();
                if let Some(caps) = meal_regex.captures(text) {
                    self.meal = caps.get(1).map(|c| c.as_str().to_string());
                }

                if self.in_menu_item {
                    if let Some(ref servery) = self.servery_title {
                        if let Some(mut items) = self.items_by_servery.get_mut(servery) {
                            items.push(text.to_string());
                        }
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
                                if let Some(ref title) = self.servery_title {
                                    self.items_by_servery.insert(title.clone(), Vec::new());
                                }
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
}