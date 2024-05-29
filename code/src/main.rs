#[macro_use]
extern crate glium;
extern crate winit;
use util::read_model;
use winit::event_loop::ControlFlow;
use glium::{backend::Facade, glutin::surface::WindowSurface, implement_vertex, Display, Surface};
use std::time::{Duration, Instant};

mod rendering;
use rendering::render::Vertex_Simple;

mod util;

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

    let hex_vert = vec![
        Vertex_Simple{position: [0.0, 0.0]},
        Vertex_Simple{position: [-0.5, 1.0]},
        Vertex_Simple{position: [0.5, 1.0]},
        Vertex_Simple{position: [1.0, 0.0]},
        Vertex_Simple{position: [0.5, -1.0]},
        Vertex_Simple{position: [-0.5, -1.0]},
        Vertex_Simple{position: [-1.0, 0.0]},
    ];

    let hex_indecies: [u16; 18] = [ 
        0, 1, 2,
        0, 2, 3,
        0, 3, 4,
        0, 4, 5,
        0, 5, 6,
        0, 6, 1];

    let hex_indecies_fan: [u16; 8] = [ 
        0, 1, 2, 3, 4 , 5, 6, 1];

    let shape: Vec<Vertex_Simple> = vec![
        Vertex_Simple { position: [0.25, 0.25] },
        Vertex_Simple { position: [ 0.0,  -0.5] },
        Vertex_Simple { position: [ -0.5, 0.0] },
        ];

    let cup_verts = util::read_model("./models/hex.obj");
    let vert_shad = util::read_shader("./shaders/vert1.4s");
    let frag_shad_1 = util::read_shader("./shaders/frag1.4s");
    let frag_shad_2 = util::read_shader("./shaders/frag2.4s");

    let hex_renderer = rendering::render::Renderer::new(hex_vert, hex_indecies_fan.to_vec(), Some(glium::index::PrimitiveType::TriangleFan), &vert_shad, &frag_shad_1, None, &display).unwrap();
    let trig_renderer = rendering::render::Renderer::new(cup_verts, vec![], None, &vert_shad, &frag_shad_2, None, &display).unwrap();

    let _ = event_loop.run(move |event, window_target| {
        match event {
            winit::event::Event::WindowEvent { event, .. } => match event {
            winit::event::WindowEvent::CloseRequested => window_target.exit(),
            winit::event::WindowEvent::Resized(window_size) => {
                display.resize(window_size.into());
            },
            winit::event::WindowEvent::RedrawRequested => {

                //time += 0.02;

                //let x_off = time.sin() * 0.5;

                let uniforms = uniform! {
                    matrix: [
                        [0.5, 0.0, 0.0, 0.0],
                        [0.0, 0.5, 0.0, 0.0],
                        [0.0, 0.0, 0.0, 0.0],
                        [ 0.0, 0.0, 0.0, 1.0f32],
                    ]
                };

                let mut target = display.draw();
                target.clear_color(0.0, 0.7, 0.7, 1.0);

                hex_renderer.draw(&mut target, None);
                trig_renderer.draw(&mut target, None);

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
