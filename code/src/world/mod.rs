pub mod hex;
pub mod layout;
pub mod world_camera;
pub mod draw_functions;
pub mod tile;
pub mod offset_coords;

//Must be divisible by 2
pub const NUM_ROWS:usize = 200;
pub const NUM_COLMS:usize = 200;

pub static tile_yield:u32 = 2;