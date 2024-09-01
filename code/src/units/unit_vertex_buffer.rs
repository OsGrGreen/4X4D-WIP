use crate::Attr;
use glium::{glutin::surface::WindowSurface, Display};

use super::unit::Unit;

pub struct UnitVbo{
    pub vbo: glium::VertexBuffer<Attr>,
    start: u32,
    end: u32,
}

impl UnitVbo{
    pub fn new(max_units: usize, display: &Display<WindowSurface>) -> UnitVbo{
        return UnitVbo { 
            vbo: glium::vertex::VertexBuffer::empty_dynamic(display, max_units).unwrap(),
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
            let mut read_slice = write_slice.read().unwrap();
            for (i,unit) in units.into_iter().enumerate(){
                if read_slice[i].is_zero(){
                    read_slice[i] = unit;
                    self.start = i as u32;
                }
            }
            write_slice.write(&read_slice);
        }
    }

    pub fn remove_units(&mut self, units:Vec<Unit>){
        let mut vbo_mapping = self.vbo.map_write();
        for unit in units{
            vbo_mapping.set(unit.render_id as usize, Attr { world_position: [0.0,0.0,0.0], colour: [0.0,0.0,0.0], tex_offsets: [0.0,0.0,0.0] });
            if unit.render_id < self.start{
                self.start = unit.render_id;
            }
        }
    }
}