#[macro_use]
extern crate glium;
extern crate winit;
use util::read_model;
use winit::{event_loop::{self, ControlFlow, EventLoop}, keyboard, window::{self, Fullscreen, Window}};
use glium::{backend::Facade, glutin::{api::egl::display, surface::WindowSurface}, implement_vertex, Display, Surface};
use world::{hex::{FractionalHex, Hex}, layout::{self, Hex_Layout, Point}};
use std::{alloc::Layout, io::stdout, time::{Duration, Instant}};

mod rendering;
use rendering::render::{array_to_VBO, Vertex_Simple};

mod util;

mod world;


#[derive(Copy, Clone, Debug)]
struct Attr {
    world_position: [f32; 2],
    colour: [f32; 2] // Changed to array
}
implement_vertex!(Attr, world_position, colour);


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
    let monitor_handle = window.primary_monitor();
    let std_width = 800.0;
    let std_height = 480.0;
    window.set_fullscreen(Some(Fullscreen::Borderless(monitor_handle)));
    let mut width_scale:f64 = window.inner_size().width as f64 / std_width;
    let mut height_scale:f64 = window.inner_size().height as f64 / std_height;


    let hex_scale: f32 = 0.005;
    let hex_size = 0.1;

    let hex = Hex::new(0, 0, 0);
    let layout = Hex_Layout::new_flat(Point{x:hex_size/hex_scale,y:(hex_size)/hex_scale},Point{x:0.0,y:0.0});
    let corners = layout.polygon_corners(&hex); 

    let hex_vert_2 = array_to_VBO(corners);
    let hex_indecies_fan: [u16; 8] = [ 
        0, 1, 2, 3, 4 , 5, 6, 1];

    let cup_verts = util::read_model("./models/hex.obj");
    let vert_shad = util::read_shader("./shaders/vert1.4s");
    let vert_shad_2 = util::read_shader("./shaders/vert2.4s");
    let frag_shad_1 = util::read_shader("./shaders/frag1.4s");
    let frag_shad_2 = util::read_shader("./shaders/frag2.4s");

    let hex_renderer = rendering::render::Renderer::new(hex_vert_2, hex_indecies_fan.to_vec(), Some(glium::index::PrimitiveType::TriangleFan), &vert_shad, &frag_shad_1, None, &display).unwrap();
    let trig_renderer = rendering::render::Renderer::new(cup_verts, vec![], None, &vert_shad_2, &frag_shad_2, None, &display).unwrap();
    
    let mut perspective = rendering::render::calculate_perspective(window.inner_size().into());
    let mut frames:f32 = 0.0;

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

    println!("Window size is: {:?}", window.inner_size().width);
    println!("Frame buffer size is: {:?}", display.get_framebuffer_dimensions().0);

    let mut q = -14;
    let mut r = 15;
    let mut max_r = 14;
    let data = (0..96 * 54)
        .map(|_| {
            let s = -q-r;
            let mut coords = layout.hex_to_pixel(&Hex::new(q, r, s));
            if q < 14{
                q += 1;
                if q % 2 == 0 && r > -13{
                    r -= 1;
                }
            }else if max_r > 0{
                q = -14;
                r = max_r;
                max_r -= 1;
            }
            let mut colorX = 0.0;
            let mut colorY = 0.0;
            if q == 0 && r == 0 {
                colorX = 1.0;
                colorY = 1.0;
                println!("Size is: {:#?}", layout.size);
                println!("Coords are: {:#?}", coords);
            }else{
                colorX = ((q+14) as f32/28.0);
                colorY = ((r+14) as f32/28.0);
            }

            Attr {
                world_position: [coords.x, coords.y-layout.size.y*2.0],
                colour: [colorX,colorY],
            }
        })
        .collect::<Vec<_>>();
    println!("{:#?}", data[0]);
    let per_instance = glium::vertex::VertexBuffer::persistent(&display, &data).unwrap();
    let mut mouse_pos: Point = Point{x:30.0,y:17.0};
    let frac_hex = layout.pixel_to_hex(&mouse_pos);
    let clicked_hex = frac_hex.hex_round();
    println!("Clicked hex is: {:#?}", clicked_hex);
    println!("Hex 1, -1 is at pixel: {:#?}", layout.hex_to_pixel(&Hex::new(1,-1,0)));
    println!("Dimension is: {:#?}", window.inner_size());
    println!("Scale factors are: {} and {}", width_scale, height_scale);
    //println!("{:#?}", per_instance);
            
    //let mut timer = Instant::now();
    let _ = event_loop.run(move |event, window_target| {
        /* if frames >= 10{
            let now = Instant::now();
            let du  ration = now.duration_since(timer);
            if duration.as_millis() >= 1{
                println!("FPS: {}", (frames*1000) / duration.as_millis());
                frames = 0;
                timer = Instant::now();
            }
        }  */
        match event {
            winit::event::Event::WindowEvent { event, .. } => match event {
            winit::event::WindowEvent::CloseRequested => window_target.exit(),
            winit::event::WindowEvent::CursorMoved { device_id, position } => {
                mouse_pos.x = ((position.x as f32) - window.inner_size().width as f32/2.0)/width_scale as f32;
                mouse_pos.y = ((-position.y as f32) + window.inner_size().height as f32/2.0)/height_scale as f32;
                //println!("Mouse posistion became {:#?}", mouse_pos);
            }
            winit::event::WindowEvent::MouseInput { device_id, state, button } =>{
                println!("Clicked {:#?}", mouse_pos);
                //println!("Dimension is: {:#?}", window.inner_size());
                let frac_hex = layout.pixel_to_hex(&mouse_pos);
                let clicked_hex = frac_hex.hex_round();
                println!("Clicked hex is: {:#?}", clicked_hex);
            }
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
                width_scale = window_size.width as f64/ std_width;
                height_scale = window_size.height as f64/ std_height;
                println!("Scale factors are: {} and {}", width_scale, height_scale);
            },
            winit::event::WindowEvent::RedrawRequested => {

                //time += 0.02;

                //let x_off = time.sin() * 0.5;

                let mut target = display.draw();

                let obj_size = [
                    [0.01, 0.0, 0.0, 0.0],
                    [0.0, 0.01+frames.sin(), 0.0, 0.0],
                    [0.0, 0.0, 0.01, 0.0],
                    [0.0, 0.0, 2.0, 1.0f32]
                ];
                let hex_size = [
                    [1.0*hex_scale, 0.0, 0.0, 0.0],
                    [0.0, 1.0*hex_scale, 0.0, 0.0],
                    [0.0, 0.0, 1.0*hex_scale, 0.0],
                    [0.0, 0.0, 2.0, 1.0f32]
                ];

                target.clear_color(0.0, 0.7, 0.7, 1.0);

                //trig_renderer.draw(&mut target, Some(&params), Some(&uniform!{matrix: obj_size, perspective: perspective}));
                target.draw(
                    (&hex_renderer.vbo, per_instance.per_instance().unwrap()),
                    &hex_renderer.indicies,
                    &hex_renderer.program,
                    &uniform! { matrix: hex_size, perspective: perspective },
                    &Default::default(),
                ).unwrap();
                //hex_renderer.draw(&mut target, Some(&params), Some(&uniform!{matrix: hex_size, perspective: perspective}));


                target.finish().unwrap();

            },
            _ => (),
            },
            winit::event::Event::AboutToWait => {
                window.request_redraw();
            },
            _ => (),
        };
        frames += 1.0;
    });
}
