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
    render_id: u32,
    posistion: (u32,u32)
}