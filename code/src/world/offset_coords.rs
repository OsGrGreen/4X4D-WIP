use crate::world::layout::{EVEN, ODD};

use super::hex::Hex;



/**
 * For flat rotation
 */
pub fn qoffset_from_cube(offset: i32, h:&Hex) -> (isize,isize){
    assert!(offset == EVEN || offset == ODD);
    let col = h.get_q() as isize;
    let row = h.get_r() as isize + ((h.get_q() + offset*(h.get_q() & 1)) as f32 / 2.0) as isize;
    return (col, row);
}

/**
 * For flat rotation
 */
pub fn qoffset_to_cube(offset: i32, col: isize, row: isize) -> (isize,isize){
    assert!(offset == EVEN || offset == ODD);
    let q = col as isize;
    let r = row as isize - ((col as isize + (offset as f32* ((col & 1) as f32 / 2.0)) as isize)) as isize;
    (q, r)
}


/**
 * For pointy rotation
 */
pub fn roffset_from_cube(offset: i32, h:&Hex) -> (isize,isize){
    assert!(offset == EVEN || offset == ODD);
    let col = h.get_q() as isize + ((h.get_r() + offset*(h.get_r() & 1)) as f32 / 2.0) as isize;
    let row = h.get_r() as isize;
    return (col, row);
}

/**
 * For pointy rotation
 */
pub fn roffset_to_cube(offset: i32, h:&Hex) -> (u32,u32){
    (0,0)
}