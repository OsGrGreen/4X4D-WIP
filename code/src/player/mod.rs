use crate::improvements::resource::ResourceCounter;

pub struct Player<'a>{
    player_id: u32,
    name: &'a str,
    resource_counters: Vec<ResourceCounter<'a>>,
    faction_id: u32,
}