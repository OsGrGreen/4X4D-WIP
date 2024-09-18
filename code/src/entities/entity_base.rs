use crate::world::OffsetTile;

use super::Entity;

#[derive(Copy, Clone, Debug)]
pub struct BaseEntity{
    pub player_id: u8,
    pub render_id: u32,
    health: i32,
    power: u32,
    attack_range: u16,
    movement: u16, //Could probably fit into a u8
    max_movement: u16, //Could probably fit into a u8 (maybe combine a u8 with movement)
    offset_pos: OffsetTile, //Possible to change into a single u32 where first 16 bits are q and last 16 bits are r /too save space
}

impl BaseEntity{
    pub fn new(player_id: u8, render_id: u32,health: i32, power:u32,attack_range: u16, movement: u16, offset_pos: OffsetTile) -> BaseEntity{
        BaseEntity{
            player_id: player_id,
            render_id: render_id,
            health: health,
            power: power,
            attack_range: attack_range,
            movement: movement,
            max_movement: movement,
            offset_pos,
        }
    }

    pub fn get_offset_pos(&self) -> OffsetTile{
        self.offset_pos
    }

    pub fn get_movement(&self) -> u16{
        self.movement
    }

    pub fn decrement_movement(&mut self, distance: u16){
        self.movement -= distance;
    }

    pub fn set_pos(&mut self, target_pos: OffsetTile) -> (){
        self.offset_pos = target_pos;
    }

}
