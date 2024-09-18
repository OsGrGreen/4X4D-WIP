
use crate::{entities::{self, entity_vertex_buffer::{EntityPosAttr, EntityVBO}, EntityHandler}, Attr};

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

// Void, 0
// Water, 1
// Woods, 2
// Plains, 3
// Mountain, 4
// Hills, 5
// Fog of War, 6
// Debug, 7

//Hardcoded for now..

pub const BIOME_TO_TEXTURE: [[f32;3];8] = [
    [0.25,0.75,0.25], // 0 Void
    [0.25,0.50,0.25], //1 water
    [0.0,0.75,0.25], // 2 Woods
    [0.75,0.75,0.25], // 3 Plains
    [0.25,0.75,0.25], // 4 Mountain
    [0.0,0.50,0.25], // 5 Hills
    [0.25,0.50,0.25], // 6 Fog of war
    [0.50,0.75,0.25]]; // 7 debug

 
pub const NONE_POS_ATTR: EntityPosAttr = EntityPosAttr{world_position: [10.0,10.0,-10.0], colour: [0.0,0.0,0.0]};


// Make this slightly more efficient...
// By for example knowing the camera z-posistion and not updating tiles that are not in view...
// Updating like I do in "unit_vertex_buffer" does not seem to be more efficient, however change is minimal..


// Should also take a hashmap of all units
pub fn update_hex_map_colors(vertex_buffer: &mut glium::VertexBuffer<Attr>, tiles: &Vec<Vec<Tile>>,entity_handler: &mut EntityHandler,start_tile: (isize,isize), size_screen: (i32,i32)) {
    
    //let timer = Instant::now();

    //println!("Start tile is: {:#?}", start_tile);
    let vertex_copy_hex = vertex_buffer.read().unwrap();
    let mut mapping_hex = vertex_buffer.map_write();
    let change_units = entity_handler.entity_vbo.vbo.slice_mut(0..entity_handler.entity_vbo.end as usize).unwrap();

    let mut write_vec:Vec<EntityPosAttr> = vec![NONE_POS_ATTR;change_units.len()];
    let start_row = ((start_tile.0) + NUM_ROWS as isize) as usize % NUM_ROWS;
    let start_column = ((start_tile.1) + NUM_COLMS as isize) as usize % NUM_COLMS;
    let mut row_pos = start_row;
    let mut column_pos = start_column;


    //println!("Start row is: {}", row_pos);
    //println!("Start column is: {}", column_pos);
    let mut traveresd_hexes = 0;
    for (i, hex) in vertex_copy_hex.iter().enumerate() {
        //println!("hex nr: {}, has world_vec pos: {}, {}", i, row_pos, column_pos);
        let current_tile = tiles[row_pos][column_pos];

        let mut final_colour = current_tile.get_biome_colour();


        /*
        * Add logic for changing tile if it is improved here... 
        */
        if current_tile.get_improved() == 1{
            for c in &mut final_colour{
                *c -= 0.4;
            }
        }
        let unit_pos = (row_pos as u32,column_pos as u32);
        if current_tile.get_occupied() == 1{
            if entity_handler.entity_map.entities.contains_key(&unit_pos)   {
                write_vec[entity_handler.entity_map.entities.get(&unit_pos).unwrap().get_render_id()] = EntityPosAttr{
                    world_position: hex.world_position,
                    colour: final_colour,
                };
            } else{
                for c in &mut final_colour{
                    *c += 0.8;
                } 
            }
        }
        
        
        mapping_hex.set(i,Attr {
            world_position: hex.world_position, //Move world_posistion out to another buffer. Is unnecessary to have it a part of this one
                                                //We just move more infromation than we need
            colour: final_colour,
            tex_offsets: BIOME_TO_TEXTURE[current_tile.get_biome() as usize],
        });

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
    }

    change_units.write(&write_vec);
    //println!("Time elapsed for updating screen is: {} ms", timer.elapsed().as_millis());
}

