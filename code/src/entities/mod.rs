use std::collections::HashMap;
use glium::{glutin::surface::WindowSurface, Display};
use entity_vertex_buffer::EntityVBO;
use units::unit::{BaseUnit, UnitType};

use crate::world::tile::Tile;

pub mod entity_base;
pub mod units;
pub mod improvements;
pub mod entity_vertex_buffer;

pub struct EntityHandler{
    pub entity_vbo: EntityVBO,
    pub entity_map: EntityMap,
}

impl EntityHandler{
    pub fn new(max_entities: usize, display: &Display<WindowSurface>) -> EntityHandler{
        EntityHandler{
            entity_vbo: EntityVBO::new(max_entities, display),
            entity_map: EntityMap::new(),
        }
    }

    pub fn create_unit(&mut self, world: &mut Vec<Vec<Tile>>, pos: (u32,u32), unit_type: UnitType, player_id: u8,extra_health: i32, extra_movement: i16, extra_power: i32, extra_range: i16){
        self.entity_map.entities.insert(pos, Box::new(BaseUnit::new(unit_type, self.entity_vbo.end,extra_health,extra_power, extra_range, extra_movement, player_id, pos))); 
        self.entity_vbo.end += 1;
        world[pos.0 as usize][pos.1 as usize].set_occupied(1);

    }
}

pub trait Entity{
    fn attack(&mut self) -> u16;
    fn damage(&mut self, dmg: u16) -> bool;
    fn movement(&mut self, target_pos: (u32,u32)) -> ();
    fn destroy(&mut self) -> ();
    fn buff(&mut self) -> ();
    fn get_texture(&self) -> [f32;3];
    fn get_render_id(&self) -> usize;
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