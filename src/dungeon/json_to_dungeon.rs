use crate::dungeon::room::Room;
use serde_json::Result;

// TODO: 1. One json room to Room
//       2. Two + json rooms to Vec<Room>
pub fn convert(dungeon: &str) -> Result<Room> {
    let r: Room = serde_json::from_str(dungeon)?;
    Ok(r)
}
