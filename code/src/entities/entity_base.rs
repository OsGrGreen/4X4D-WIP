use super::Entity;

pub struct BaseEntity{
    player_id: u8,
    health: i32,
    power: u32,
    attack_range: u16,
    movement: u16, //Could probably fit into a u8
    max_movement: u16, //Could probably fit into a u8 (maybe combine a u8 with movement)
    axial_pos: (u32,u32), //Possible to change into a single u32 where first 16 bits are q and last 16 bits are r /too save space
}

impl BaseEntity{
    pub fn new(player_id: u8, health: i32, power:u32,attack_range: u16, movement: u16, axial_pos: (u32,u32)) -> BaseEntity{
        BaseEntity{
            player_id: player_id,
            health: health,
            power: power,
            attack_range: attack_range,
            movement: movement,
            max_movement: movement,
            axial_pos: axial_pos,
        }
    }
}

impl Entity for BaseEntity{
    fn attack(&mut self) -> u16 {
        todo!()
    }

    fn damage(&mut self, dmg: u16) -> bool {
        todo!()
    }

    fn movement(&mut self, target_pos: (u32,u32)) -> () {
        todo!()
    }

    fn destroy(&mut self) -> () {
        todo!()
    }

    fn buff(&mut self) {
        todo!()
    }
    
    fn get_texture(&self) -> ([f32;3]) {
        todo!()
    }
}