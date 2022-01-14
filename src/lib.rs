mod dungeon;
use dungeon::room::Room;
use dungeon::items;

pub fn run() {
    let mut mists_hollow = vec!(
        Room {
            name: "Courtyard",
            description:
            r#"A large area filled with grey woods and broken hedges guards the grounds of the courtyard"#,
            adjectives: vec!("Large", "Expansive"),
            items: vec!(
                Box::new(items::Sword {
                    name: "short sword",
                    description: "A shining piece of metal stained with red remains",
                    power: 12,
                }
            )),
            connecting_rooms: vec!("Mansion", "Garden"),
        },
        Room {
            name: "Mansion",
            description:
            r#"Rich with splendor and prestige, the air is hot and the sight is grand. A red
            carpet invites you up the staircase. "#,
            adjectives: vec!("Grand"),
            items: vec!(
                Box::new(items::Sword {
                    name: "short sword",
                    description: "A shining piece of metal stained with red remains",
                    power: 12,
                }
                )),
            connecting_rooms: vec!("Mansion", "Garden"),
        }
    );
    run_dungeon(&mut mists_hollow);
}


pub fn run_dungeon(dungeon: &mut Vec<Room>) {
    for room in dungeon.iter() {
        println!("{:?}", *room);
    }
}
