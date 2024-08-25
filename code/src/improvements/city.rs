use super::building::Building;

pub struct City<'a>{
    tile: (u32,u32),
    buildings: Vec<Building<'a>>,
    player_id: u32,
    health: u32,
}