use std::collections::HashMap;
use html5ever::Attribute;

pub fn map_attrs<'a>(attrs: &'a Vec<Attribute>) -> HashMap<&'a str, &'a str> {
    let mut map = HashMap::new();
    for attr in attrs.iter() {
        map.insert(&*attr.name.local, &*attr.value);
    }
    map
}