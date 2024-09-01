pub enum UnitType {
    worker,
    warrior,
    archer,
}

pub struct Unit{
    unit_type: UnitType,
    health: i32,
    power: u32,
    attack_range: u16,
    movement: u16,
    max_movement: u16,
    player_id: u32,
    pub render_id: u32,
    cube_posistion: (u32,u32)
}

impl Unit{
    pub fn new(unit_type: UnitType, extra_health: i32, extra_power: i32, extra_range: i16, extra_movement: i16, player_id: u32, pos: (u32, u32)) -> Unit{
        let health = Self::type_to_health(&unit_type) + extra_health;
        let power = Self::type_to_power(&unit_type) + extra_power;
        let range = Self::type_to_range(&unit_type) + extra_range;
        let movement = Self::type_to_movement(&unit_type) + extra_movement;

        Unit { unit_type: unit_type, health: health, power: power as u32, attack_range: range as u16, movement: movement as u16, max_movement: movement as u16, player_id: player_id, render_id: 0, cube_posistion: pos}

    }

    fn type_to_health(unit_type: &UnitType) -> i32{
        match unit_type{
            UnitType::worker => 5,
            UnitType::warrior => 15,
            UnitType::archer => 8,
            _ => 2,
        }
    }
    
    fn type_to_power(unit_type: &UnitType) -> i32{
        match unit_type{
            UnitType::worker => 1,
            UnitType::warrior => 5,
            UnitType::archer => 3,
            _ => 2,
        }
    }

    fn type_to_range(unit_type: &UnitType) -> i16{
        match unit_type{
            UnitType::worker => 1,
            UnitType::warrior => 1,
            UnitType::archer => 3,
            _ => 1,
        }
    }

    fn type_to_movement(unit_type: &UnitType) -> i16{
        match unit_type{
            UnitType::worker => 2,
            UnitType::warrior => 3,
            UnitType::archer => 2,
            _ => 2,
        }
    }

    pub fn type_to_texture_ids(unit_type: &UnitType) -> [f32;3]{
        match unit_type{
            UnitType::worker => [0.0,0.0,0.125],
            UnitType::warrior => [0.125,0.0,0.125],
            UnitType::archer => [0.25,0.0,0.125],
            _ => [0.375,0.0,0.125],
        }
    }

}