#[macro_use]
extern crate glium;
extern crate winit;
use rand::{distr::{Distribution, Uniform}, Rng};
use glam::Vec3;
use util::{input_handler::{self, InputHandler}, read_model};
use winit::{event_loop::{self, ControlFlow, EventLoop}, keyboard, window::{self, Fullscreen, Window}};
use glium::{backend::Facade, glutin::{api::egl::{device, display}, surface::WindowSurface}, implement_vertex, Display, Surface};
use world::{draw_functions::{self, cantor_2}, hex::{FractionalHex, Hex}, layout::{self, Hex_Layout, Point, SQRT3}, tile::Tile, world_camera::WorldCamera, NUM_COLMS, NUM_ROWS};
use std::{alloc::Layout, io::stdout, mem::{self, size_of}, time::{Duration, Instant}};
use glium::PolygonMode::Line;
mod rendering;
use rendering::{render::{array_to_VBO, Vertex_Simple}, render_camera::{self, RenderCamera}};

mod util;

mod world;


#[derive(Copy, Clone, Debug)]
struct Attr {
    world_position: [f32; 3],
    colour: [f32; 3] // Changed to array
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

    let mut world_vec: Vec<Vec<Tile>> = vec![vec![]];
    let mut rng = rand::thread_rng();
    let die = Uniform::new_inclusive(0, 5).unwrap();
    for i in 0..NUM_COLMS{
        for _ in 0..NUM_ROWS{
            world_vec[i].push(Tile::new(die.sample(&mut rng), 0));
        }
        if i != NUM_COLMS-1{
            world_vec.push(vec![]);
        }
    }
    //world_vec[0][0].set_biome(7);
    for val in 0..world_vec.len(){
        world_vec[val][0].set_biome(7);
    }

    //println!("world vec is now {:#?}", world_vec);
    println!("world vec length is {:#?} x {:#?}", world_vec.len(), world_vec[0].len());

    //let mut camera = RenderCamera::new(Vec3::new(0.0,0.0,0.5), Vec3::new(0.0,0.0,0.0));
    let mut camera = RenderCamera::new(Vec3::new(0.0,0.0,4.5), Vec3::new(0.0,0.0,0.0), Vec3::new(0.0, 1.0, 0.0), Vec3::new(0.0,0.0,-1.0));

    //Camera constants

    const CAMERA_SPEED:f32 = 0.0005;

    // Input handler

    let mut input_handler = InputHandler::new();

    let mut camera_matrix: glam::Mat4 = camera.look_at(camera.get_pos()+camera.get_front());
    //println!("camera matrix glm is {:#?}", RenderCamera::look_at_glm(Vec3::new(2.0,-1.0,1.0), Vec3::new(-2.0,1.0,1.0),Vec3::new(0.0,1.0,0.0)));
    //println!("camera matrix is: {:#?}", camera_matrix);
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
    let hex_size = 0.04;

    let constant_factor = hex_size/0.04;

    let hex = Hex::new(0, 0, 0);
    let layout = Hex_Layout::new_flat(Point{x:hex_size/hex_scale,y:hex_size/hex_scale},Point{x:0.0,y:0.0});
    let corners = layout.polygon_corners(&hex); 
    let mut world_camera = WorldCamera::new();

    println!("New hex each {} x", layout.size.x as f32*(0.01+hex_scale));

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
    
    let light = [-1.0, 0.4, 0.9f32];

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
        depth: glium::Depth {
            test: glium::draw_parameters::DepthTest::IfLess,
            write: true,
            .. Default::default()
        },
        //backface_culling: glium::draw_parameters::BackfaceCullingMode::CullClockwise,
        .. Default::default()
    };

    //println!("Window size is: {:?}", window.inner_size().width);
    //println!("Frame buffer size is: {:?}", display.get_framebuffer_dimensions().0);

    let needed_hexes_x = ((800.0) / (2.0*(layout.size.x))) as i32;
    let needed_hexes_y = ((480.0) / (layout.size.y)) as i32;

    let screen_size = (needed_hexes_x as usize, needed_hexes_y as usize);

    let mut q = -needed_hexes_x+5;
    let mut r = needed_hexes_y-8;
    let mut max_r = needed_hexes_y-9;
    let mut amount_of_hexes = 0;
    //println!("Needed hexes are {:#?}, {:#?}", needed_hexes_x, needed_hexes_y);

    let mut color1: Vec<[f32;3]> = vec![];
    let mut color2: Vec<[f32;3]> = vec![];
    //Does a 300 iterations to many
    let data = (0..(needed_hexes_x*r*2) as usize)
        .map(|debug_val| {
            //Gör så att denna börjar längre ned, är nödigt att ha massor över och för lite under...
            let s = -q-r;

            println!("Cantor of hex {}, {}, {} is {}", q, r, s, draw_functions::cantor_3(q as f64, r as f64,s as f64));
            
            let coords = layout.hex_to_pixel(&Hex::new(q, r, s));

            let mut color_x = 0.0;
            let mut color_y = 0.0;
            if q == 0 && r == 0 {
                color_x = 1.0;
                color_y = 1.0;
                //println!("Size is: {:#?}", layout.size);
                //println!("Coords are: {:#?}", coords);
            }else{
                color_x = (q+20) as f32/40.0;
                color_y = (r+20) as f32/40.0;
            }


            let color_choose = (((q-r) % 3) + 3) % 3;
            let mut color = if color_choose == 0{
                0.0
            }else if color_choose == 1 {
                0.5
            }else{
                1.0
            };

            if q < needed_hexes_x-5{
                q += 1;
                if q % 2 == 0 && r > -needed_hexes_y{
                    r -= 1;
                }
                amount_of_hexes += 1;
            }else if max_r > -2{
                q = -needed_hexes_x+5;
                r = max_r;
                max_r -= 1;
                amount_of_hexes += 1;
            }
            color1.push([color_x,color_y, 1.0]);
            color2.push([color_y,color_x, 1.0]);
            Attr {
                world_position: [coords.x, coords.y, -1.0],
                colour: [color/2.0,color, 0.0],
            }
        })
        .collect::<Vec<_>>();

    println!("data length is: {}", data.len());

    
    //println!("{:#?}", data[0]);
    println!("Amount of true hexes are: {:#?}", amount_of_hexes);

    // Maybe try to have a double buffer of some kind..
    // See: https://stackoverflow.com/questions/14155615/opengl-updating-vertex-buffer-with-glbufferdata
    let mut per_instance = glium::vertex::VertexBuffer::persistent(&display, &data).unwrap();

    let timer = Instant::now();
    draw_functions::update_hex_map_colors(&mut per_instance, &world_vec, (0,0), screen_size);
    println!("Elapsed for updating {} x {} world took: {} ms", NUM_COLMS, NUM_ROWS, timer.elapsed().as_millis());

    let mut mouse_pos: Point = Point{x:30.0,y:17.0};
    let frac_hex = layout.pixel_to_hex(&mouse_pos);
    let clicked_hex = frac_hex.hex_round();
    //println!("Clicked hex is: {:#?}", clicked_hex);
    //println!("Hex 1, -1 is at pixel: {:#?}", layout.hex_to_pixel(&Hex::new(0,0,0)));
    //println!("Dimension is: {:#?}", window.inner_size());
    //println!("Scale factors are: {} and {}", width_scale, height_scale);
    //println!("{:#?}", per_instance);

    let mut what_color:bool = false;

    
    let radius = 5.0;

    let mut timer = Instant::now();
    let _ = event_loop.run(move |event, window_target| {
        let now = Instant::now();
        //Delta time calculation may be wrong...
        let delta_time = now.duration_since(timer).as_millis() as f32;
        //println!("Delta time is: {}", delta_time);
        timer = now;
        /*let duration = now.duration_since(timer);
        if duration.as_millis() >= 1{
            //println!("FPS: {}", (frames*1000.0) / duration.as_millis() as f32);
            frames = 0.0;
            timer = Instant::now();
        }*/

        //let camX = (timer.elapsed().as_millis() as f32 / 1000.0).sin()*radius;
        //let camZ = (timer.elapsed().as_millis() as f32 / 1000.0).cos()*radius;
        //println!("CamX is {}, camZ is: {}", camX, camZ);
        //camera_matrix = RenderCamera::look_at_glm(Vec3::new(camX,0.0,camZ), Vec3::new(0.0,0.0,0.0),Vec3::new(0.0,1.0,0.0));

    
        //let mut change_x = 0.0;
        //let mut change_y = 0.0;

        //Update movement (Kanske göra efter allt annat... possibly):
        let mut movement = input_handler.get_movement();
        if movement.length() > 0.0{
            let mut traveresed_whole_hex = false;
            movement = movement.normalize();
            //Flytta en i taget...
            camera.r#move(delta_time*movement[1]*CAMERA_SPEED*camera.get_up());
            let y_pos = camera.get_pos()[1];
            //Inte helt perfekt än måste fixa till lite....
            if y_pos < constant_factor*-0.206{
                camera.set_y(0.0);
                world_camera.move_camera(0, 3);
                traveresed_whole_hex = true;
            } else if y_pos > constant_factor*0.206{
                camera.set_y(0.0);
                world_camera.move_camera(0, -3);
                traveresed_whole_hex = true;
            }        
            camera.r#move(delta_time*movement[0]*CAMERA_SPEED*(camera.get_front().cross(camera.get_up())).normalize());
            let x_pos = camera.get_pos()[0];
                        //Kom på varför det är 0.12 här och inget annat nummer...
                        //Verkar ju bara bero på hex_size och inte scale....
            if x_pos < constant_factor*-0.12{
                camera.set_x(0.0);
                world_camera.move_camera(-2, 0);
                traveresed_whole_hex = true;
            }else if x_pos > constant_factor*0.12{
                camera.set_x(0.0);
                world_camera.move_camera(2, 0);
                traveresed_whole_hex = true;
            }
            //println!("Camera is: {}", camera.get_pos());
            //Gör så kameran bara uppdateras när man faktiskt rör på sig...
            if traveresed_whole_hex{
                draw_functions::update_hex_map_colors(&mut per_instance, &world_vec, world_camera.offsets(),screen_size);
            }
            camera_matrix = camera.look_at(camera.get_pos()+camera.get_front());
        }


        match event {
            winit::event::Event::WindowEvent { event, .. } => match event {
            winit::event::WindowEvent::CloseRequested => window_target.exit(),
            winit::event::WindowEvent::CursorMoved { device_id, position } => {
                let new_pos_x = ((position.x as f32) - window.inner_size().width as f32/2.0 - world_camera.offsets().0 as f32)/width_scale as f32;
                let new_pos_y =  ((-position.y as f32) + window.inner_size().height as f32/2.0 + world_camera.offsets().0 as f32)/height_scale as f32;
                //change_x = mouse_pos.x-new_pos_x;
                //change_y = mouse_pos.y-new_pos_y;
                mouse_pos.x = new_pos_x;
                mouse_pos.y = new_pos_y;
                //println!("Mouse posistion became {:#?}", mouse_pos);
            }
            winit::event::WindowEvent::MouseInput { device_id, state, button } =>{
                println!("Clicked {:#?}", mouse_pos);
                //println!("Dimension is: {:#?}", window.inner_size());
                let frac_hex = layout.pixel_to_hex(&mouse_pos);
                let clicked_hex = frac_hex.hex_round();
                println!("Clicked hex is: {:#?}", clicked_hex);
            }

            // TODO
            // Make input a little bit nicer
            winit::event::WindowEvent::KeyboardInput { device_id, event, is_synthetic: _ } =>{

                //Handle other inputs
                if event.physical_key == keyboard::KeyCode::Escape && event.state.is_pressed(){
                    window_target.exit()
                } 
                else if event.physical_key == keyboard::KeyCode::KeyQ && event.state.is_pressed(){
                    camera.r#move(50.0*-CAMERA_SPEED*camera.get_front());
                    camera_matrix = camera.look_at(camera.get_pos()+camera.get_front());
                }
                else if event.physical_key == keyboard::KeyCode::KeyE{
                    camera.r#move(50.0*CAMERA_SPEED*camera.get_front());
                    camera_matrix = camera.look_at(camera.get_pos()+camera.get_front());
                }else if event.physical_key == keyboard::KeyCode::KeyU && event.state.is_pressed(){
                    world_camera.move_camera(0, 2);
                    draw_functions::update_hex_map_colors(&mut per_instance, &world_vec, world_camera.offsets(),screen_size);
                }
                else if event.physical_key == keyboard::KeyCode::KeyH && event.state.is_pressed(){
                    world_camera.move_camera(2, 0);
                    draw_functions::update_hex_map_colors(&mut per_instance, &world_vec, world_camera.offsets(),screen_size);
                }
                else if event.physical_key == keyboard::KeyCode::KeyJ && event.state.is_pressed(){
                    world_camera.move_camera(0, -2);
                    draw_functions::update_hex_map_colors(&mut per_instance, &world_vec, world_camera.offsets(),screen_size);
                }
                else if event.physical_key == keyboard::KeyCode::KeyK && event.state.is_pressed(){
                    world_camera.move_camera(-2, 0);
                    draw_functions::update_hex_map_colors(&mut per_instance, &world_vec, world_camera.offsets(),screen_size);
                }
                //Handle WASD

                input_handler.update_input(event);

            },
            winit::event::WindowEvent::Resized(window_size) => {
                perspective = rendering::render::calculate_perspective(window_size.into());
                display.resize(window_size.into());
                width_scale = window_size.width as f64/ std_width;
                height_scale = window_size.height as f64/ std_height;
                println!("Scale factors are: {} and {}", width_scale, height_scale);
            },
            winit::event::WindowEvent::RedrawRequested => {
                let dur2 = Instant::now();
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
                    [1.0*hex_scale, 0.0, 0.0, 0.0],
                    [0.0, 1.0*hex_scale, 0.0, 0.0],
                    [0.0, 0.0, 1.0*hex_scale, 0.0],
                    [0.0, 0.0, 2.0, 1.0f32]
                ];

                target.clear_color_and_depth((0.1, 0.4, 0.2, 1.0), 1.0);

                target.draw(
                    (&hex_renderer.vbo, per_instance.per_instance().unwrap()),
                    &hex_renderer.indicies,
                    &hex_renderer.program,
                    &uniform! { model: hex_size, projection: perspective, view:camera_matrix.to_cols_array_2d()},
                    &glium::DrawParameters {
                        depth: glium::Depth {
                            test: glium::DepthTest::IfLess,
                            write: true,
                            .. Default::default()
                        },
                        //polygon_mode: Line,
                        .. Default::default()
                    },
                ).unwrap();
                //trig_renderer.draw(&mut target, Some(&params), Some(&uniform! { model: obj_size, projection: perspective, view:camera_matrix.to_cols_array_2d(), u_light:light}));
                //hex_renderer.draw(&mut target, Some(&params), Some(&uniform!{matrix: hex_size, perspective: perspective}));


                target.finish().unwrap();
                //println!("Time for drawing frame: {}", dur2.elapsed().as_millis());
            },
            _ => (),
            },
            winit::event::Event::AboutToWait => {
                window.request_redraw();
            },
            _ => (),
        };


        println!("Time for one frame is: {} ms", now.elapsed().as_millis());
        frames += 1.0;

    });
}
