use std::collections::HashMap;
use html5ever::Attribute;

/// Turn a vector of html5ever Attributes into a HashMap mapping attribute name
/// to attribute value.
pub fn map_attrs<'a>(attrs: &'a Vec<Attribute>) -> HashMap<&'a str, &'a str> {
    let mut map = HashMap::new();
    for attr in attrs.iter() {
        map.insert(&*attr.name.local, &*attr.value);
    }
    map
}