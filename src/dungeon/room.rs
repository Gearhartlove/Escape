use crate::dungeon::items::Item;
use core::fmt::Debug;
use core::fmt::Formatter;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize, Serialize)]
pub struct Room {
    pub description: &'static str,
    pub adjectives: Vec<&'static str>,
    //pub items: Vec<Box<dyn Item>>,
    pub items: HashMap<String, HashMap<String, String>>,
    pub connecting_rooms: Vec<&'static str>,
}

impl Debug for Room {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.description)
    }
}
