use crate::entities::entity_base::BaseEntity;

pub enum UnitType {
    worker,
    warrior,
    archer,
}

//TODO Implement specific types of units

pub struct BaseUnit{
    unit_type: UnitType, //Is this needed (?), yes could be nice.
    pub render_id: u32,
    entity: BaseEntity,
}

impl BaseUnit{
    pub fn new(unit_type: UnitType, extra_health: i32, extra_power: i32, extra_range: i16, extra_movement: i16, player_id: u8, pos: (u32, u32)) -> BaseUnit{
        let health = Self::type_to_health(&unit_type) + extra_health;
        let power = Self::type_to_power(&unit_type) + extra_power;
        let range = Self::type_to_range(&unit_type) + extra_range;
        let movement = Self::type_to_movement(&unit_type) + extra_movement;

        BaseUnit { unit_type: unit_type,  render_id: 0,entity: BaseEntity::new(player_id, health, power as u32, range as u16, movement as u16, pos)}

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