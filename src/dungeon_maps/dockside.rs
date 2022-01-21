pub fn get_dungeon() -> &'static str {
    let dockside = r#"
            "dockside" = {
                "description" = "lots of boats around",
                "adjectives" = ["open", "breezy"],
                "items" = [
                    {"name"="sword", "power"="12", "length"="long"},
                    {"name"="lighter", "fuel level"="low"}
                    ],
                "connecting_rooms" = ["lighthouse", "tavern"],
            }"#;
    dockside
}
