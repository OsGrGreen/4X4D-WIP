/* use glium::glutin::api::egl::display;
use glium::Display;
use glium::{glutin::surface::WindowSurface};
use winit::event_loop::EventLoop;
use winit::window::Window; */


use glam::Mat4;
use glium::{glutin::surface::WindowSurface, uniforms::{AsUniformValue, Uniforms, UniformsStorage}, Display, DrawParameters, Frame, Program, Surface};

use crate::world::layout::Point;

#[derive(Copy, Clone,Debug)]
pub struct VertexSimple {
    pub position: [f32; 2],
}
implement_vertex!(VertexSimple, position);

#[derive(Copy, Clone,Debug)]
pub struct Vertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
    pub tex_coords: [f32; 2], 
}
implement_vertex!(Vertex, position,normal,tex_coords);

pub struct Renderer
    {
        // What to be rendered (Verts)
        pub vbo: glium::vertex::VertexBufferAny,
        // How to be rendered (Indicies) (Kan behöva öka från u16 till u32)
        pub indicies: glium::IndexBuffer<u16>,
        // The program to render it
        pub program: Program,
        // Specific Uniforms (Känns kanske lite svårt att spara på ett bra sätt här?)
        // Texture
}

impl Renderer{
        pub fn new<'a>(shape: Vec<Vertex>, inds: Vec<u16>, prim_type: Option<glium::index::PrimitiveType> ,vert_shader: &'a str, frag_shader: &'a str, geo_shader: Option<&'a str>, disp: &Display<WindowSurface>) -> Result<Renderer, &'a str>{
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

        pub fn draw<T, R>(&self, frame: &mut Frame, draw_parameters: Option<&DrawParameters>, uniforms: Option<&UniformsStorage<T, R>>)
        where
        T: AsUniformValue,
        R: Uniforms,
        {
            if uniforms.is_some(){
                frame.draw(&self.vbo, &self.indicies, &self.program, uniforms.unwrap(),
                draw_parameters.unwrap_or(&Default::default())).unwrap();
            }else{
                frame.draw(&self.vbo, &self.indicies, &self.program, &glium::uniforms::EmptyUniforms,
                    draw_parameters.unwrap_or(&Default::default())).unwrap();
            }
                
        }
}  

pub fn calculate_perspective(dim: (f32, f32)) -> Mat4{
    let perspective = {
        let (width, height) = dim;
        let aspect_ratio = height as f32 / width as f32;
        
        let fov: f32 = std::f32::consts::PI / 4.0;
        let zfar = 100.0;
        let znear = 0.1;
    
        let f = 1.0 / (fov / 2.0).tan();
        [
            [f *   aspect_ratio   ,    0.0,              0.0              ,   0.0],
            [         0.0         ,     f ,              0.0              ,   0.0],
            [         0.0         ,    0.0,  (zfar+znear)/(zfar-znear)    ,   1.0],
            [         0.0         ,    0.0, -(2.0*zfar*znear)/(zfar-znear),   0.0],
        ]
    };

    return Mat4::from_cols_array_2d(&perspective)
}

pub fn point_to_Vertex(p: Point, uv: (f32,f32)) -> Vertex{
    return Vertex{position: [p.x, p.y, 0.0], normal: [0.0,0.0,0.0], tex_coords: [uv.0, uv.1]}
}

const UV_HEX: [(f32,f32);6] =  [(1.0,0.366),(0.866,0.866),(0.366,1.0),(0.0,0.634),(0.134,0.134),(0.634,0.0)];

pub fn array_to_VBO(points: Vec<Point>) -> Vec<Vertex>{
    let mut output: Vec<Vertex> = vec![];
    for (i,p) in points.into_iter().enumerate(){
        output.push(point_to_Vertex(p, UV_HEX[i]));
    }
    return output
}