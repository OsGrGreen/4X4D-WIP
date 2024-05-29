/* use glium::glutin::api::egl::display;
use glium::Display;
use glium::{glutin::surface::WindowSurface};
use winit::event_loop::EventLoop;
use winit::window::Window; */

use std::{collections::HashMap, option};

use glium::{glutin::surface::WindowSurface, index::Index, uniforms::UniformsStorage, Display, DrawParameters, Frame, Program, Surface};

#[derive(Copy, Clone,Debug)]
pub struct Vertex_Simple {
    pub position: [f32; 2],
}
implement_vertex!(Vertex_Simple, position);

pub struct Renderer
    {
        // What to be rendered (Verts)
        vbo: glium::vertex::VertexBufferAny,
        // How to be rendered (Indicies) (Kan behöva öka från u16 till u32)
        indicies: glium::IndexBuffer<u16>,
        // The program to render it
        program: Program,
        // Specific Uniforms (Känns kanske lite svårt att spara på ett bra sätt här?)
        // Texture
}

impl Renderer{
        pub fn new<'a>(shape: Vec<Vertex_Simple>, inds: Vec<u16>, prim_type: Option<glium::index::PrimitiveType> ,vert_shader: &'a str, frag_shader: &'a str, geo_shader: Option<&'a str>, disp: &Display<WindowSurface>) -> Result<Renderer, &'a str>{
            let shape_len = shape.len();

            let vbo = glium::VertexBuffer::new(disp, &shape).unwrap();

            let program = glium::Program::from_source(disp, vert_shader, frag_shader, geo_shader).unwrap();

            if inds.len() < 1{
                //println!("Found no indecies");
                let mut inds = vec![];
                for n in (0..shape_len).step_by(3){
                    //println!("Pushing: {}, {}, {}", n, (n+1)%shape_len,(n+2)%shape_len);
                    inds.push(n as u16);
                    inds.push(((n+1)%shape_len) as u16);
                    inds.push(((n+2)%shape_len) as u16);
                }
                let indicies = glium::IndexBuffer::new(disp,prim_type.unwrap_or(glium::index::PrimitiveType::TrianglesList),
                &inds).unwrap();

                Ok(Renderer{
                    vbo: vbo.into(),
                    indicies: indicies,
                    program: program,
                })
            }else{
                let indicies = glium::IndexBuffer::new(disp,prim_type.unwrap_or(glium::index::PrimitiveType::TrianglesList),
                &inds).unwrap();

                Ok(Renderer{
                    vbo: vbo.into(),
                    indicies: indicies,
                    program: program,
                })
            }
        }

        pub fn draw(&self, frame: &mut Frame, draw_parameters: Option<&DrawParameters>){
            let uniforms = uniform! {
                matrix: [
                    [0.01, 0.0, 0.0, 0.0],
                    [0.0, 0.01, 0.0, 0.0],
                    [0.0, 0.0, 0.01, 0.0],
                    [ 0.0, 0.0, 0.0, 1.0f32],
                ]
            };
            frame.draw(&self.vbo, &self.indicies, &self.program, &uniforms,
                    &Default::default()).unwrap();
        }
    }  