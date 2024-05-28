#[macro_use]
extern crate glium;
extern crate winit;
use winit::event_loop::ControlFlow;
use glium::{implement_vertex, Surface};


#[derive(Copy, Clone,Debug)]
struct Vertex {
    position: [f32; 2],
}
implement_vertex!(Vertex, position);

const VERT_SHADER: &str = r#"
    #version 330 core

    in vec2 position;
    uniform mat4 matrix;

    out vec3 colPos;

    void main() {
        gl_Position = vec4(position.x,position.y,0.0, 1.0);
        colPos = mat3(matrix)*vec3(0.6, position);
    }
"#;

const FRAG_SHADER: &str = r#"
    #version 330 core
    out vec4 color;
    in vec3 colPos; 

    void main() {
        color = vec4(colPos, 1.0);
    }
"#;


#[derive(Copy, Clone,Debug)]
struct Point {
    x: f32,
    y: f32,
}

fn pointy_hex_corner(center: Point, size: usize, i: i32) -> Point {
    let angle_deg = 60.0 * i as f32 - 30.0;
    let angle_rad = std::f32::consts::PI / 180.0 * angle_deg;
    Point {
        x: center.x + size as f32 * angle_rad.cos(),
        y: center.y + size as f32 * angle_rad.sin(),
    }
}



fn main() {
    // 1. The **winit::EventLoop** for handling events.
    let event_loop = winit::event_loop::EventLoopBuilder::new().build().expect("event loop building");
    // 2. Create a glutin context and glium Display
    event_loop.set_control_flow(ControlFlow::Poll);
    let (window, display) = glium::backend::glutin::SimpleWindowBuilder::new().build(&event_loop);
    window.set_title("4X4D");

    let hex_vert = vec![
        Vertex{position: [0.0, 0.0]},
        Vertex{position: [-0.5, 1.0]},
        Vertex{position: [0.5, 1.0]},
        Vertex{position: [1.0, 0.0]},
        Vertex{position: [0.5, -1.0]},
        Vertex{position: [-0.5, -1.0]},
        Vertex{position: [-1.0, 0.0]},
    ];

    let hex_indecies: [u16; 18] = [ 
        0, 1, 2,
        0, 2, 3,
        0, 3, 4,
        0, 4, 5,
        0, 5, 6,
        0, 6, 1];

    let mut time: f32 = 0.0;
    let _ = event_loop.run(move |event, window_target| {
        match event {
            winit::event::Event::WindowEvent { event, .. } => match event {
            winit::event::WindowEvent::CloseRequested => window_target.exit(),
            winit::event::WindowEvent::Resized(window_size) => {
                display.resize(window_size.into());
            },
            winit::event::WindowEvent::RedrawRequested => {

                time += 0.02;

                let x_off = time.sin() * 0.5;

                let uniforms = uniform! {
                    matrix: [
                        [time.cos(), time.sin(), 0.0, 0.0],
                        [-time.sin(), time.cos(), 0.0, 0.0],
                        [0.0, x_off, 1.0, 0.0],
                        [ x_off, 0.0, 0.0, 1.0f32],
                    ]
                };

                let mut target = display.draw();
                target.clear_color(0.0, 1.0, 0.0, 1.0);
            

                let shape: Vec<Vertex> = vec![
                    Vertex { position: [0.25, 0.25] },
                    Vertex { position: [ 0.0,  -0.5] },
                    Vertex { position: [ -0.5, 0.0] }
                ];
        
                let vertex_buffer = glium::VertexBuffer::new(&display, &hex_vert).unwrap();
                
                let indices = glium::IndexBuffer::new(&display, glium::index::PrimitiveType::TrianglesList,
                    &hex_indecies).unwrap();
                let program = glium::Program::from_source(&display, VERT_SHADER, FRAG_SHADER, None).unwrap();
               
                target.draw(&vertex_buffer, &indices, &program, &uniforms,
                    &Default::default()).unwrap();
                target.finish().unwrap();

            },
            _ => (),
            },
            winit::event::Event::AboutToWait => {
                window.request_redraw();
            },
            _ => (),
        };
    });
}
