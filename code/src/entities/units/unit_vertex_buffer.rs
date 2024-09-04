use crate::Attr;
use glium::{glutin::surface::WindowSurface, Display};
use rand::{random, Rng};

use super::unit::BaseUnit;

pub struct UnitVbo{
    pub vbo: glium::VertexBuffer<Attr>,
    start: u32,
    end: u32,
}

impl UnitVbo{
    pub fn new(max_units: usize, display: &Display<WindowSurface>) -> UnitVbo{
        
        //Make this be truly empty, by not using empty_dynamic...
        let mut empty_vec:Vec<Attr> = Vec::with_capacity(max_units);
        for i in 0..max_units{
            empty_vec.push(Attr { world_position: [0.0,0.0,0.0], colour: [0.0,0.0,0.0], tex_offsets: [0.0,0.0,0.0] });
        }
        
        return UnitVbo { 
            vbo: glium::vertex::VertexBuffer::dynamic(display, &empty_vec).unwrap(),
            start: 0,
            end: 0,
        }
    }

    pub fn add_units(&mut self, units:Vec<Attr>){
        if self.start >= self.end{
            let amount_of_units = units.len();
            let vbo_slice = self.vbo.slice_mut(0..amount_of_units).unwrap();
            vbo_slice.write(&units);
            self.start += amount_of_units as u32;
            self.end += amount_of_units as u32;
        }else{
            let write_slice = self.vbo.slice_mut(self.start as usize..self.end as usize).unwrap();
            let mut read_slice: Vec<Attr> = write_slice.read().unwrap();
            for (i,unit) in units.into_iter().enumerate(){
                if read_slice[i].is_zero(){
                    read_slice[i] = unit;
                    self.start = i as u32;
                }
            }
            write_slice.write(&read_slice);
        }
    }

    pub fn remove_units(&mut self, units:Vec<BaseUnit>){
        let mut vbo_mapping = self.vbo.map_write();
        for unit in units{
            vbo_mapping.set(unit.render_id as usize, Attr { world_position: [0.0,0.0,0.0], colour: [0.0,0.0,0.0], tex_offsets: [0.0,0.0,0.0] });
            if unit.render_id < self.start{
                self.start = unit.render_id;
            }
        }
    }

    pub fn animate_unit(tex_offsets: &mut [f32;3]){
        tex_offsets[0] += 0.125;
    }

    pub fn animate_all_units(&mut self,rng:&mut rand::prelude::ThreadRng){
        let write_slice = self.vbo.slice_mut(0..self.end as usize).unwrap();
        let mut read_slice:Vec<Attr> = write_slice.read().unwrap();
        for id in 0..self.end as usize{
            read_slice[id].tex_offsets[0] += 0.125*rng.gen_range(0..=1) as f32;
            if read_slice[id].tex_offsets[0] >= 1.0{
                read_slice[id].tex_offsets[0] = 0.0;
            }
        }
        write_slice.write(&read_slice);
    }
}