use super::Entity;

pub struct BaseEntity{
    pub player_id: u8,
    pub render_id: u32,
    health: i32,
    power: u32,
    attack_range: u16,
    movement: u16, //Could probably fit into a u8
    max_movement: u16, //Could probably fit into a u8 (maybe combine a u8 with movement)
    axial_pos: (u32,u32), //Possible to change into a single u32 where first 16 bits are q and last 16 bits are r /too save space
}

impl BaseEntity{
    pub fn new(player_id: u8, render_id: u32,health: i32, power:u32,attack_range: u16, movement: u16, axial_pos: (u32,u32)) -> BaseEntity{
        BaseEntity{
            player_id: player_id,
            render_id: render_id,
            health: health,
            power: power,
            attack_range: attack_range,
            movement: movement,
            max_movement: movement,
            axial_pos: axial_pos,
        }
    }
}
