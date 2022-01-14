use crate::dungeon::items::Item;
use core::fmt::Debug;
use core::fmt::Formatter;

pub struct Room {
    pub name: &'static str,
    pub description: &'static str,
    pub adjectives: Vec<&'static str>,
    pub items: Vec<Box<dyn Item>>,
    pub connecting_rooms: Vec<&'static str>,
}

impl Debug for Room {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name);
        f.write_str(": ");
        f.write_str(self.description)
    }
}
