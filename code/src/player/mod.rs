use crate::improvements::resource::Resource_Counter;

pub struct player<'a>{
    player_id: u32,
    name: &'a str,
    resource_counters: Vec<Resource_Counter<'a>>,
    faction_id: u32,
}