use core::fmt::Debug;
use std::fmt::Formatter;

pub fn run() {
    let mut mists_hollow = vec!(
        Room {
            name: "Courtyard",
            description:
            r#"A large area filled with grey woods and broken hedges guards the grounds of the courtyard"#,
            adjectives: vec!("Large", "Expansive"),
            items: vec!(
                Box::new(Sword {
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
                Box::new(Sword {
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

pub struct Room {
    name: &'static str,
    description: &'static str,
    adjectives: Vec<&'static str>,
    items: Vec<Box<dyn Item>>,
    connecting_rooms: Vec<&'static str>,
}

impl Debug for Room {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name);
        f.write_str(": ");
        f.write_str(self.description)
    }
}

struct Sword {
    name: &'static str,
    description: &'static str,
    power: i32,
}

impl Sword {
    fn power(&self) -> i32 { self.power }
}

trait Item {
    fn name(&self) -> &str ;
    fn get_description(&self) -> &str ;
    fn use_item(&self, verb: String);
    fn message(&self) -> String;
}

impl Item for Sword {
    fn name(&self) -> &str { &self.name }
    fn get_description(&self) -> &str { &self.description }
    fn use_item(&self, verb: String) {
        println!("The sword has been {} and you hit for {}", verb, self.power);
    }
    fn message(&self) -> String {
        format!("item: {} description: {} power: {}",
                self.name() , self.get_description(), self.power())
    }
}