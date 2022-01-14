use std::collections::HashMap;
use std::fmt;
use std::fmt::{Debug, Write};
use serde::{Deserialize, Serialize};
use serde_json::Result;
pub fn run() {
    typed_example().unwrap()
    // item register?
}

#[derive(Serialize, Deserialize)]
struct Room{
    name: String,
    description: String,
    adjectives: Vec<String>,
    // items: Vec<Box<dyn Item>>,
    // items_info_register: HashMap<String, String>,
    items_info_register: Vec<ItemInstance> // This is mega jank
    // connecting_rooms: Vec<Box<Room>>,
}

struct ItemInstance {
    name: String,
    data: String,
}

trait Item {
    fn name(&self) -> &String;
    fn description(&self) -> &String;
    fn use_item(&self, verb: String);
    // fn message(&self) -> &str;
}

// impl Debug for dyn Item {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         f.write_str(self.message())
//     }
// }

struct Sword {
    name: String,
    description: String,
    power: i32,
}

impl Sword {
    fn new (n: String, desc: String, pow: i32) -> Self {
        Sword {name: n, description: desc, power: pow}
    }
    fn power(&self) -> i32 { self.power }
}

impl Item for Sword {
    fn name(&self) -> &String { &self.name }
    fn description(&self) -> &String { &self.description }
    fn use_item(&self, verb: String) {
        println!("The sword has been {} and you hit for {}", verb, self.power);
    }
    // fn message(&self) -> &str{
    //     format!("item: {} description: {} power: {}",
    //             self.name(), self.description(), self.power())
    // }
}

// Serde code below
fn typed_example() -> Result<()> {
    // JSON input from a dungeon_name.json file.
    // let data = r#"
    //     {
    //         "name": "sword room",
    //         "description": "An air of royalty fills the rooms as a sword glimmers above rock",
    //         "adjectives": "gleaming",
    //         "items_vec": [
    //             {"name": "sword", "description": "stuck in a stone"},
    //             {"name": "skull", "description": "to large to be human"}
    //             {"name": "sword", "description": "large and bloody", "power": 90},
    //         ],
    //         "connecting_rooms": [
    //             "name": "staircase room",
    //             "name": "basement room",
    //         ]
    // }"#;

    let data = r#"
        {
            "name": "sword room",
            "description": "an air of royalty fills the room",
            "adjectives": ["dark", "musty", "damp"],
            "items": {
                "name": "sword",
                "description": "large and bloody",
                "power": "90"
            }
        }"#;

    let room: Room = serde_json::from(data)?;
    println!("items: {:?}", room.items_info_register);
    Ok(())
}

