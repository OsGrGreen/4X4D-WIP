#[macro_use]
extern crate glium;
extern crate winit;
use util::read_model;
use winit::{event_loop::{self, ControlFlow, EventLoop}, keyboard, window::{self, Window}};
use glium::{backend::Facade, glutin::{api::egl::display, surface::WindowSurface}, implement_vertex, Display, Surface};
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

fn init_window()-> (EventLoop<()>, Window, Display<WindowSurface>) {
    let event_loop = winit::event_loop::EventLoopBuilder::new().build().expect("event loop building"); 
    event_loop.set_control_flow(ControlFlow::Poll);
    let (window, display) = glium::backend::glutin::SimpleWindowBuilder::new().with_title("4X4D-WIP").build(&event_loop);
    (event_loop, window, display)
}

fn main() {
    // 1. The **winit::EventLoop** for handling events.
    let (event_loop, window, display) = init_window();
    // Check if windows then: 
    //window.set_window_icon(window_icon);
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
    
    let mut perspective = rendering::render::calculate_perspective(window.inner_size().into());
    //let mut frames = 0;

    let params = glium::DrawParameters {
        //To enable backfaceculling uncomment this
        /* depth: glium::Depth {
            test: glium::DepthTest::IfLess,
            write: true,
            .. Default::default()
        },
        backface_culling: glium::draw_parameters::BackfaceCullingMode::CullClockwise, */
        .. Default::default()
    };

    //let mut timer = Instant::now();
    let _ = event_loop.run(move |event, window_target| {
        /* if frames >= 10{
            let now = Instant::now();
            let duration = now.duration_since(timer);
            if duration.as_millis() >= 1{
                println!("FPS: {}", (frames*1000) / duration.as_millis());
                frames = 0;
                timer = Instant::now();
            }
        }  */
        match event {
            winit::event::Event::WindowEvent { event, .. } => match event {
            winit::event::WindowEvent::CloseRequested => window_target.exit(),
            winit::event::WindowEvent::KeyboardInput { device_id:_, event, is_synthetic: _ } =>{
                //println!("Event was: {:#?}", event);
                //println!("Device Id was: {:#?}\n", device_id);
                if event.physical_key == keyboard::KeyCode::Escape{
                    window_target.exit()
                }
            },
            winit::event::WindowEvent::Resized(window_size) => {
                perspective = rendering::render::calculate_perspective(window_size.into());
                display.resize(window_size.into());
            },
            winit::event::WindowEvent::RedrawRequested => {

                //time += 0.02;

                //let x_off = time.sin() * 0.5;

                let mut target = display.draw();

                let obj_size = [
                    [0.01, 0.0, 0.0, 0.0],
                    [0.0, 0.01, 0.0, 0.0],
                    [0.0, 0.0, 0.01, 0.0],
                    [0.0, 0.0, 2.0, 1.0f32]
                ];
                let hex_size = [
                    [0.025, 0.0, 0.0, 0.0],
                    [0.0, 0.025, 0.0, 0.0],
                    [0.0, 0.0, 0.025, 0.0],
                    [0.0, 0.0, 2.0, 1.0f32]
                ];

                target.clear_color(0.0, 0.7, 0.7, 1.0);

                trig_renderer.draw(&mut target, Some(&params), Some(&uniform!{matrix: obj_size, perspective: perspective}));
                hex_renderer.draw(&mut target, Some(&params), Some(&uniform!{matrix: hex_size, perspective: perspective}));


                target.finish().unwrap();

            },
            _ => (),
            },
            winit::event::Event::AboutToWait => {
                window.request_redraw();
            },
            _ => (),
        };
        //frames += 1;
    });
}
