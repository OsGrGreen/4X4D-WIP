use std::collections::HashMap;

pub mod entity_base;
pub mod units;
pub mod improvements;

pub trait Entity{
    fn attack(&mut self) -> u16;
    fn damage(&mut self, dmg: u16) -> bool;
    fn movement(&mut self, target_pos: (u32,u32)) -> ();
    fn destroy(&mut self) -> ();
    fn buff(&mut self) -> ();
    fn get_texture(&self) -> ([f32;3]);
}
pub struct EntityMap{
    pub entities: HashMap<(u32,u32),Box<dyn Entity>>,
}

impl EntityMap{
    pub fn new() -> EntityMap{
        EntityMap{
            entities: HashMap::default()
        }
    }
}