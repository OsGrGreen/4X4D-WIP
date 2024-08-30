use crate::Attr;
use glium::{glutin::surface::WindowSurface, Display};

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
        let amount_of_units = units.len();
        let vbo_slice = self.vbo.slice_mut(0..amount_of_units).unwrap();
        vbo_slice.write(&units);
        self.start += amount_of_units as u32;
        self.end += amount_of_units as u32;
    }
}