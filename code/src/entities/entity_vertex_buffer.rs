use std::time::Instant;

use crate::{entities::{Entity, EntityMap}, Attr};
use glium::{glutin::surface::WindowSurface, Display};
use rand::{random, Rng};

use super::units::unit::{BaseUnit, UnitType};


#[derive(Copy, Clone, Debug)]
pub struct EntityPosAttr {
    pub world_position: [f32; 3],
    pub colour: [f32; 3], // Changed to array
}
implement_vertex!(EntityPosAttr, world_position, colour);


impl EntityPosAttr{
    pub fn is_zero(&self) -> bool{
        if self.colour == [0.0,0.0,0.0]{
            return true
        }else{
            return false
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct EntityTexAttr {
    tex_offsets: [f32;3], //x offset, y offset, scaling factor          For reading in texture atlas
}
implement_vertex!(EntityTexAttr, tex_offsets);

pub struct EntityVBO{
    pub vbo: glium::VertexBuffer<EntityPosAttr>,
    pub tex_vbo: glium::VertexBuffer<EntityTexAttr>,
    pub start: u32,
    pub end: u32,
}

impl EntityVBO{
    pub fn new(max_entities: usize, display: &Display<WindowSurface>) -> EntityVBO{
        
        //Make this be truly empty, by not using empty_dynamic...
        let mut empty_vec_1:Vec<EntityPosAttr> = Vec::with_capacity(max_entities);
        let mut empty_vec_2:Vec<EntityTexAttr> = Vec::with_capacity(max_entities);
        for i in 0..max_entities{
            empty_vec_1.push(EntityPosAttr { world_position: [0.0,0.0,0.0], colour: [0.0,0.0,0.0]});
            empty_vec_2.push(EntityTexAttr { tex_offsets: [0.0,0.0,0.0] });
        }
        
        return EntityVBO { 
            vbo: glium::vertex::VertexBuffer::dynamic(display, &empty_vec_1).unwrap(),
            tex_vbo:glium::vertex::VertexBuffer::dynamic(display, &empty_vec_2).unwrap(),
            start: 0,
            end: 0,
        }
    }

    pub fn add_units(&mut self, units:Vec<EntityPosAttr>){
        if self.start >= self.end{
            let amount_of_units = units.len();
            let vbo_slice = self.vbo.slice_mut(0..amount_of_units).unwrap();
            vbo_slice.write(&units);
            self.start += amount_of_units as u32;
            self.end += amount_of_units as u32;
        }else{
            let write_slice = self.vbo.slice_mut(self.start as usize..self.end as usize).unwrap();
            let mut read_slice: Vec<EntityPosAttr> = write_slice.read().unwrap();
            for (i,unit) in units.into_iter().enumerate(){
                if read_slice[i].is_zero(){
                    read_slice[i] = unit;
                    self.start = i as u32;
                }
            }
            write_slice.write(&read_slice);
        }
    }


    // Make this able to handle if start is not end...
    pub fn add_texture_unit(&mut self, unit_type: UnitType){
        let write_slice = self.tex_vbo.slice_mut(self.end as usize..self.end as usize + 1).unwrap();
        write_slice.write(&[EntityTexAttr{tex_offsets: BaseUnit::type_to_texture_ids(&unit_type)}]);
        self.end += 1;
        self.start += 1;
    }

    /*pub fn remove_units(&mut self, units:Vec<BaseUnit>){
        let mut vbo_mapping = self.vbo.map_write();
        for unit in units{
            vbo_mapping.set(unit.render_id as usize, EntityPosAttr { world_position: [0.0,0.0,0.0], colour: [0.0,0.0,0.0]});
            if unit.render_id < self.start{
                self.start = unit.render_id;
            }
        }
    }*/


    //Maybe make this a little bit more efficient...
    pub fn animate_all_entities(&mut self, time: f32, entities: &EntityMap){
        let write_slice = self.tex_vbo.slice_mut(0..self.end as usize).unwrap();
        let none = EntityTexAttr{tex_offsets: [0.0,0.0,0.0]};
        let mut write_vec:Vec<EntityTexAttr> = vec![none;entities.entities.len()];
        //println!("{}", entities.entities.len());
        for (_, unit) in entities.entities.iter(){
            let mut tex_offsets = unit.get_texture();
            tex_offsets[0] += 1.0*tex_offsets[2]*(time); //replace 0.125 with tex_offsets[2]
            if tex_offsets[0] >= 1.0{
                tex_offsets[0] = 0.0;
            }
            write_vec[unit.get_render_id()] = EntityTexAttr{tex_offsets: tex_offsets};
        }
        write_slice.write(&write_vec);
    }
}