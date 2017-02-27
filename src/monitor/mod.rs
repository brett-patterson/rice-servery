use std::collections::HashMap;
use std::fs::File;
use std::io::{Write, stderr};
use std::path::Path;

use serde_json;

mod alerts;
mod config;

use super::menu::Menu;
use self::config::{Config, Rule};
use self::alerts::alert;

/// A match found by the monitor.
#[derive(Debug)]
struct Match {
    item: String,
    servery: String,
}

/// A monitor used to search for keywords in servery menus.
pub struct Monitor {
    config: Config,
    matches: HashMap<Rule, Vec<Match>>,
}

impl Monitor {
    /// Construct a Monitor from a JSON configuration file.
    pub fn from_file(path: &Path) -> Self {
        let file = match File::open(path) {
            Ok(file) => file,
            Err(e) => panic!("Error reading {}: {}", path.display(), e),
        };

        let config: Config = match serde_json::from_reader(&file) {
            Ok(cfg) => cfg,
            Err(e) => panic!("Error parsing {}: {}", path.display(), e),
        };

        Monitor { config: config, matches: HashMap::new() }
    }

    /// Fetch the servery data, parse it, and run it through each of the rules
    /// given by the configuration.
    pub fn process(&mut self) {
        match Menu::fetch() {
            Ok(menu) => {
                for (servery, items) in menu.items_by_servery.iter() {
                    for item in items.iter() {
                        self.check_match(item, servery);
                    }
                }

                for (rule, matches) in &self.matches {
                    let title = format!("Found {} for {}", rule.keyword, menu.meal);
                    let body = matches.iter()
                        .map(|m| format!("{} at {}", m.item, m.servery))
                        .fold("".to_string(),
                              |acc, m| if acc.len() == 0 { m } else { acc + "\n" + &m });

                    alert(&title, &body, rule, &self.config);
                }
            }
            Err(e) => {
                writeln!(stderr(), "{}", e).unwrap();
            }
        }
    }

    /// Check if a menu item matches a rule, and if so add it to the set
    /// of matches.
    fn check_match(&mut self, item: &str, servery: &str) {
        let lower = &item.to_lowercase();
        for rule in self.config.rules.iter() {
            if lower.contains(&rule.keyword.to_lowercase()) {
                if !self.matches.contains_key(rule) {
                    self.matches.insert(rule.clone(), Vec::new());
                }

                self.matches.get_mut(rule).map(|v| {
                    v.push(Match {
                        item: item.to_string(),
                        servery: servery.to_string(),
                    })
                });
            }
        }
    }
}