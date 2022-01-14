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
    // adjectives: Vec<String>,
    // item_vec: Vec<Box<Item>>,
    // connecting_rooms: Vec<Box<Room>>,
}




// struct Item{
//     name: String,
//     description: String,
//
// }
//
// trait ItemAction {
//     fn use_item(&self, verb: String) {}
// }

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
    //             {"name": "skull", "decsription": "to large to be human"}
    //         ],
    //         "connecting_rooms": [
    //             "name": "staircase room",
    //             "name": "basement room",
    //         ]
    // }"#;

    let data = r#"
        {
            "name": "sword room",
            "description": "an air of royalty fills the room"

        }
    "#;

    let room: Room = serde_json::from_str(data)?;
    println!("name: {}", room.name);
    Ok(())
}

