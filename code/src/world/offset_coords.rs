use crate::world::layout::{EVEN, ODD};

use super::hex::Hex;

/**
 * For flat rotation
 */
pub fn qoffset_from_cube(offset: i32, h:&Hex) -> (i32,i32){
    assert!(offset == EVEN || offset == ODD);
    let col = h.get_q();
    let row = h.get_r() + ((h.get_q() + offset*(h.get_q() & 1)) as f32 / 2.0) as i32;
    return (col, row);
}

/**
 * For flat rotation
 */
pub fn qoffset_to_cube(offset: i32, h:&Hex) -> (u32,u32){
    (0,0)
}


/**
 * For pointy rotation
 */
pub fn roffset_from_cube(offset: i32, h:&Hex) -> (i32,i32){
    assert!(offset == EVEN || offset == ODD);
    let col = h.get_q() + ((h.get_r() + offset*(h.get_r() & 1)) as f32 / 2.0) as i32;
    let row = h.get_r();
    return (col, row);
}

/**
 * For pointy rotation
 */
pub fn roffset_to_cube(offset: i32, h:&Hex) -> (u32,u32){
    (0,0)
}