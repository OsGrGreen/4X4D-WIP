use std::collections::HashMap;use entity_base::BaseEntity;
use glium::{glutin::surface::WindowSurface, Display};
use entity_vertex_buffer::EntityVBO;
use units::unit::{BaseUnit, UnitType};

use crate::world::{hex::Hex, layout::EVEN, offset_coords::qoffset_to_cube, tile::Tile};

pub mod entity_base;
pub mod units;
pub mod improvements;
pub mod entity_vertex_buffer;

pub struct EntityHandler{
    pub entity_vbo: EntityVBO,
    pub entity_map: EntityMap,
    selected_entity: Option<(u32,u32)>, //Gives the key to entity in the map.
}

impl EntityHandler{
    pub fn new(max_entities: usize, display: &Display<WindowSurface>) -> EntityHandler{
        EntityHandler{
            entity_vbo: EntityVBO::new(max_entities, display),
            entity_map: EntityMap::new(),
            selected_entity: None
        }
    }

    pub fn create_unit(&mut self, world: &mut Vec<Vec<Tile>>, pos: (u32,u32), unit_type: UnitType, player_id: u8,extra_health: i32, extra_movement: i16, extra_power: i32, extra_range: i16){
        self.entity_map.entities.insert(pos, Box::new(BaseUnit::new(unit_type, self.entity_vbo.end,extra_health,extra_power, extra_range, extra_movement, player_id, pos))); 
        self.entity_vbo.add_texture_unit(unit_type);
        world[pos.0 as usize][pos.1 as usize].set_occupied(1);
    }

    pub fn get_selected_unit(&self) -> Option<&Box<dyn Entity>>{
        if self.selected_entity.is_some(){
            self.entity_map.entities.get(&self.selected_entity.unwrap())
        }else{
            None
        }
    }

    pub fn get_neighbouring_units(&self, neighbor_tiles: Vec<(u32,u32)>) -> Vec<(u32,u32)>{
        let mut return_vec = vec![];

        for pos in neighbor_tiles{
            if self.entity_map.entities.contains_key(&pos){
                return_vec.push(pos);
            }
        }

        return return_vec;
    }

    pub fn get_friendly_neighbouring_units(&self, target_tile: (u32,u32), neighbor_tiles: Vec<(u32,u32)>) -> Vec<(u32,u32)>{
        let mut return_vec = vec![];
        let main_unit = self.entity_map.entities.get(&target_tile).unwrap();
        for pos in neighbor_tiles{
            let unit = self.entity_map.entities.get(&pos);
            if unit.is_some(){
                let unit = unit.unwrap();
                if unit.get_player() == main_unit.get_player(){
                    return_vec.push(pos);
                }
            }
        }

        return return_vec;
    }

    pub fn move_unit(&mut self, target_pos: (u32,u32), world:&mut Vec<Vec<Tile>>) -> bool{

        //Check that target_pos is not forbidden tile...

        let target_pos_hex = qoffset_to_cube(EVEN, target_pos);
        match self.selected_entity{
            Some(pos) => {
                let current_hex = qoffset_to_cube(EVEN, pos);
                let ents = &mut self.entity_map.entities;
                let selected_unit = ents.get_mut(&pos).unwrap();
                //Can not go that far
                // Why have I made a system that really does not work....
                // Gonna cry a little bit.
                let move_distance = current_hex.distance(target_pos_hex);
                if move_distance > selected_unit.get_movement().into(){
                    return false;
                }
                if world[target_pos.0 as usize][target_pos.1 as usize].get_occupied() == 0{
                    // Remove the unit from the current tile
                    world[pos.0 as usize][pos.1 as usize].set_occupied(0);

                    // Make new tile occupied
                    world[target_pos.0 as usize][target_pos.1 as usize].set_occupied(1);

                    // Update unit
                    selected_unit.movement(target_pos, move_distance);
                    let unit_clone = selected_unit.clone();

                    // Remove old pos from map
                    ents.remove(&pos);

                    // Add new pos to map
                    ents.insert(target_pos, unit_clone);

                }else{
                    //Make it attack that fucker instead...
                    println!("Is occupied");
                    return false;
                }
                self.selected_entity = None;
                return true;
            }
            _ => return false,
        }
    }

    pub fn select(&mut self, pos: (u32,u32)){
        if self.entity_map.entities.contains_key(&pos){
            self.selected_entity = Some(pos);
        }
    }

    pub fn deselect(&mut self){
        self.selected_entity = None;
    }
}

pub trait Entity{
    fn attack(&mut self) -> u16;
    fn damage(&mut self, dmg: u16) -> bool;
    fn movement(&mut self, target_pos: (u32,u32), distance: u16) -> ();
    fn destroy(&mut self) -> ();
    fn buff(&mut self) -> ();
    fn get_texture(&self) -> [f32;3];
    fn get_render_id(&self) -> usize;
    fn get_pos(&self) -> (u32,u32);
    fn get_movement(&self) -> u16;
    fn clone(&self) -> Box<dyn Entity>;
    fn get_entity(&self) -> BaseEntity;
    fn get_player(&self) -> u8;
}


use core::fmt::Debug;
impl Debug for dyn Entity {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "ID: {}", self.get_render_id())
    }
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