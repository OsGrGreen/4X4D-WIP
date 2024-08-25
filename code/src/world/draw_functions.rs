use std::time::Instant;

use crate::Attr;

use super::{tile::Tile, NUM_COLMS, NUM_ROWS};


pub fn cantor_2(a:f64,b:f64) -> f64{
    return (1.0/2.0)*(a+b)*(a+b+1.0)+b
}

pub fn cantor_3(q:f64,r:f64,s:f64) -> f64{
    let res = cantor_2(q, r);
    return cantor_2(res, s);
}

pub fn reverse_cantor_2(k:f64) -> (f64, f64){
    let t = (((8.0*k+1.0).sqrt() - 1.0)/2.0).floor();
    let b = k - (t*(t+1.0))/2.0;
    let a = t-b;
    return (a,b)
}

pub fn reverse_cantor_3(k:f64) -> (f64, f64, f64){
    let (res, c) = reverse_cantor_2(k);
    let (a, b) = reverse_cantor_2(res);
    return (a,b,c)
}


//Make this slightly more efficient...
pub fn update_hex_map_colors(vertex_buffer: &mut glium::VertexBuffer<Attr>, tiles: &Vec<Vec<Tile>>, start_tile: (isize,isize), size_screen: (i32,i32)) {
    
    //let timer = Instant::now();

    //println!("Start tile is: {:#?}", start_tile);
    let vertex_copy = vertex_buffer.read().unwrap();
    let mut mapping = vertex_buffer.map_write();
    let start_row = ((start_tile.0) + NUM_ROWS as isize) as usize % NUM_ROWS;
    let start_column = ((start_tile.1) + NUM_COLMS as isize) as usize % NUM_COLMS;
    let mut row_pos = start_row;
    let mut column_pos = start_column;

    //println!("Start row is: {}", row_pos);
    //println!("Start column is: {}", column_pos);
    let mut traveresd_hexes = 0;
    for (i, hex) in vertex_copy.iter().enumerate() {
        let current_tile = tiles[column_pos][row_pos];
        //println!("Taking tile from coordinate {}, {}", column_pos, row_pos);
        column_pos += 1;
        if column_pos >= NUM_COLMS{
            column_pos = 0;
        }

        // Width is 90 hexes
        if traveresd_hexes >= size_screen.1{
            column_pos = start_column;
            row_pos += 1;
            if row_pos >= NUM_ROWS{
                row_pos = 0;
            }
            traveresd_hexes = -1;
        }
        traveresd_hexes += 1;

        let mut final_colour = current_tile.get_biome_colour();


        /*
        * Add logic for changing tile if it is improved here... 
        */
        if current_tile.get_improved() == 1{
            for c in &mut final_colour{
                *c -= 0.4;
            }
        }
        
        
        mapping.set(i,Attr {
            world_position: hex.world_position,
            colour: final_colour,
        });
    }
    //println!("Time elapsed for updating screen is: {} ms", timer.elapsed().as_millis());
}