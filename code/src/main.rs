#[macro_use]
extern crate glium;
extern crate winit;
use rand::{distr::{Distribution, Uniform}, Rng};
use glam::{Mat4, Vec2, Vec3, Vec4, Vec4Swizzles};
use util::{input_handler::{self, InputHandler}, ray_library::ray_plane_intersect, read_model};
use winit::{event_loop::{self, ControlFlow, EventLoop}, keyboard, window::{self, Fullscreen, Window}};
use glium::{backend::Facade, glutin::{api::egl::{device, display}, surface::WindowSurface}, implement_vertex, Display, Surface};
use world::{draw_functions::{self, cantor_2}, hex::{FractionalHex, Hex}, layout::{self, Hex_Layout, Point, EVEN, ODD, SQRT3}, offset_coords::{self, qoffset_from_cube}, tile::Tile, world_camera::WorldCamera, NUM_COLMS, NUM_ROWS};
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
    //First value is what row, second value is what column
    // 0,0 is bottom left corner
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


    //println!("world vec is now {:#?}", world_vec);
    println!("world vec length is {:#?} x {:#?}", world_vec.len(), world_vec[0].len());

    //let mut camera = RenderCamera::new(Vec3::new(0.0,0.0,0.5), Vec3::new(0.0,0.0,0.0));
    world_vec[12][25].set_biome(6);
    world_vec[26][13].set_biome(6);

    // Closest camera can be: z = 2.15
    // Furtherst camera can be: z = 4.85

    /*
    
    y = 4.5 =; NDC = 0.095 width 0.05 height; PXS = 38 width 34 height
    y = 2.15 =;  NDC = 0.52 width 0.8 height; PXS = 664 width 576 height
    y = 4.84 =; NDC = 0.0275 width 0.04 height; PXS = 34 width 28 height

     */
    let mut camera = RenderCamera::new(Vec3::new(0.0,0.0,4.5), Vec3::new(0.0,0.0,0.0), Vec3::new(0.0, 1.0, 0.0), Vec3::new(0.0,0.0,-1.0));

    //Camera constants

    const CAMERA_SPEED:f32 = 0.002;

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
    println!("Inner size is: {:#?}", window.inner_size());
    println!("widht_scale is: {}", width_scale);
    println!("hejgut scale is: {}", height_scale);



    let hex_scale: f32 = 1.0;
    let hex_size = 0.16;
    let hex_size_mat = [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0f32]
    ];

    let constant_factor = 1.0;

    println!("constant_factor is {}", constant_factor);
    

    let hex = Hex::new(0, 0, 0);
    let layout = Hex_Layout::new_flat(Point{x:hex_size/hex_scale,y:hex_size/hex_scale},Point{x:0.0,y:0.0});
    let corners = layout.polygon_corners(&hex); 
    let mut world_camera = WorldCamera::new((NUM_ROWS, NUM_COLMS));

    println!("New hex each {} x", layout.size.x as f32*(0.01+hex_scale));
    let FOV = std::f32::consts::PI / 3.0;
    let screen_hex_width = (layout.get_width()*1920.0)/(2.0*camera.get_pos().z*(FOV/2.0).tan());
    println!("screen hex width is: {}", screen_hex_width);


    let hex_vert_2 = array_to_VBO(corners);
    
    //println!("hexvert is {:#?}", hex_vert_2);
    //println!("hexvert is {:#?}", hex_vert_2.len());

    let hex_indecies_fan: [u16; 9] = [ 
        6, 0, 1, 2, 3, 4 , 5, 0, 6
        ];

    let cup_verts = util::read_model("./models/hex.obj");
    let vert_shad = util::read_shader("./shaders/vert1.4s");
    let vert_shad_2 = util::read_shader("./shaders/vert2.4s");
    let frag_shad_1 = util::read_shader("./shaders/frag1.4s");
    let frag_shad_2 = util::read_shader("./shaders/frag2.4s");
    //println!("{:#?}", &hex_vert_2);
    let hex_renderer = rendering::render::Renderer::new(hex_vert_2, hex_indecies_fan.to_vec(), Some(glium::index::PrimitiveType::TriangleFan), &vert_shad, &frag_shad_1, None, &display).unwrap();
    let trig_renderer = rendering::render::Renderer::new(cup_verts, vec![], None, &vert_shad_2, &frag_shad_2, None, &display).unwrap();
    
    let light = [-1.0, 0.4, 0.9f32];

    let mut perspective = rendering::render::calculate_perspective(window.inner_size().into());
    let mut frames:f32 = 0.0;

    let mut transformation_mat = (Mat4::from_cols_array_2d(&perspective)*camera_matrix*Mat4::from_cols_array_2d(&hex_size_mat));
    let mut inverse_transformation_mat = Mat4::inverse(&transformation_mat);

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

    let needed_hexes_x = (((800.0) / (2.0*(100.0*layout.get_width()))) * 1.5) as i32;
    let needed_hexes_y = (((480.0) / (100.0*layout.get_height())) * 1.5) as i32;

    let mut amount_of_hexes = 0;

    // Not the most efficient or pretty way but it works..
    let mut data: Vec<Attr> = vec![];
    let left:i32 = -needed_hexes_y/2;
    let top:i32 = -((needed_hexes_x));
    let right:i32 = left.abs();
    let bottom:i32 = top.abs();
    let screen_size = (bottom*2,right*2);
    println!("Screen size {:#?}", screen_size);
    //Börjar med att köra en column i taget.
    for q in top..=bottom{
        let q_offset = q>>1;
        for r in left-q_offset..=right-q_offset{
            let coords = layout.hex_to_pixel(&Hex::new(q, r, -q-r));

            let color_choose = (((q-r) % 3) + 3) % 3;
            let color = if color_choose == 0{
                0.0
            }else if color_choose == 1 {
                0.5
            }else{
                1.0
            };
            //println!("Posistion of this hex {}, {} is {:#?}", q, r,coords);
            let val = Attr {
                world_position: [coords.x, coords.y, 0.0],
                colour: [color/2.0,color, 0.0],
            };
            amount_of_hexes += 1;
            data.push(val);
        }
    }



    println!("Layout size is: {:#?}", layout.size);
    println!("Expected layout size is {}w, {}h",layout.get_width(),layout.get_height());
    println!("data length is: {}", data.len());
    //println!("data is: {:#?}", data);

    
    //println!("{:#?}", data[0]);
    println!("Amount of true hexes are: {:#?}", amount_of_hexes);

    // Maybe try to have a double buffer of some kind..
    // See: https://stackoverflow.com/questions/14155615/opengl-updating-vertex-buffer-with-glbufferdata
    let mut per_instance = glium::vertex::VertexBuffer::persistent(&display, &data).unwrap();

    draw_functions::update_hex_map_colors(&mut per_instance, &world_vec, world_camera.offsets(),screen_size);

    let mut mouse_pos: Point = Point{x:0.0,y:0.0};

    
    let radius = 5.0;
    let mut timer2 = Instant::now();
    let mut timer = Instant::now();
    let _ = event_loop.run(move |event, window_target| {

        //println!("timer: {}", timer2.elapsed().as_millis());

        //Delta time calculation may be wrong...
        let delta_time = (timer.elapsed().as_micros() as f32/1000.0).clamp(0.18, 10.0);
        timer = Instant::now();
        //println!("Delta time is: {}", delta_time);

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
            if y_pos < -constant_factor*(3.0*(layout.get_height())){
                camera.set_y(0.0);
                world_camera.move_camera(0, -3);
                traveresed_whole_hex = true;
            } else if y_pos > constant_factor*(3.0*(layout.get_height())){
                camera.set_y(0.0);
                world_camera.move_camera(0, 3);
                traveresed_whole_hex = true;
            }        
            camera.r#move(delta_time*movement[0]*CAMERA_SPEED*(camera.get_front().cross(camera.get_up())).normalize());
            let x_pos = camera.get_pos()[0];
                        //Kom på varför det är 0.12 här och inget annat nummer...
                        //Verkar ju bara bero på hex_size och inte scale....
            if x_pos < -constant_factor*2.0*(layout.get_width()){
                camera.set_x(0.0);
                world_camera.move_camera(-2, 0);
                traveresed_whole_hex = true;
            }else if x_pos > constant_factor*2.0*(layout.get_width()){
                camera.set_x(0.0);
                world_camera.move_camera(2, 0);
                traveresed_whole_hex = true;
            }
            //println!("Camera is: {}", camera.get_pos());
            //Gör så kameran bara uppdateras när man faktiskt rör på sig...
            if traveresed_whole_hex{
                draw_functions::update_hex_map_colors(&mut per_instance, &world_vec, world_camera.offsets(),screen_size);
            }
            println!("Camera pos is: {:#?}", world_camera.offsets());
            camera_matrix = camera.look_at(camera.get_pos()+camera.get_front());
            //inverse_mat = Mat4::inverse(&(Mat4::from_cols_array_2d(&perspective)*camera_matrix*Mat4::IDENTITY));
        }


        match event {
            winit::event::Event::WindowEvent { event, .. } => match event {
            winit::event::WindowEvent::CloseRequested => window_target.exit(),
            winit::event::WindowEvent::CursorMoved { device_id, position } => {
                //let mouse_y_flipped =  window.inner_size().height as f32 - position.y as f32;
                let mouse_ndc = Vec2::new(
                    
                    (position.x as f32 / window.inner_size().width as f32) * 2.0 - 1.0,
                    -((position.y as f32 / window.inner_size().height as f32) * 2.0 - 1.0),
                );

                //let inverse = Mat4::inverse(&(Mat4::from_cols_array_2d(&perspective)*camera_matrix*Mat4::from_cols_array_2d(&hex_size_mat)));
                let eyespace = Mat4::inverse(&Mat4::from_cols_array_2d(&perspective));
                let eye_space_vector = Vec4::new(mouse_ndc.x, mouse_ndc.y, -1.0, 1.0);
                let mut eye_vector = eyespace*eye_space_vector;
                eye_vector.z = -1.0;
                eye_vector.w = 0.0;
                let worldspace = Mat4::inverse(&camera_matrix);
                let mut world_vector = worldspace*eye_vector;
                let norm_world:Vec3 = (world_vector.xyz().normalize());
                let intersect = ray_plane_intersect(Vec3::new(0.0,0.0,camera.get_pos().z), norm_world, Vec3::new(0.0,0.0,0.0), Vec3::new(0.0,0.0,1.0));


                //println!("object space is: {}", Mat4::inverse(&Mat4::from_cols_array_2d(&hex_size_mat))*world_vector);
                mouse_pos.x = intersect.x as f32;
                mouse_pos.y = intersect.y as f32;
                //println!("Mouse posistion became {:#?}", mouse_pos);
            }
            winit::event::WindowEvent::MouseInput { device_id, state, button } =>{
                println!("Clicked {:#?}", mouse_pos);
                //println!("Dimension is: {:#?}", window.inner_size());
                let frac_hex = layout.pixel_to_hex(&mouse_pos);
                let clicked_hex = frac_hex.hex_round();
                let parity:i32 = 1 - 2 * (clicked_hex.get_r() & 1);
                println!("Clicked hex is: {:#?}, is it EVEN or ODD: {}", clicked_hex, ODD);
                
                let (mut clicked_y, mut clicked_x) = qoffset_from_cube(EVEN,&clicked_hex);
                println!("{}, {}", (needed_hexes_x/2), (needed_hexes_y/2));
                clicked_y = 25 - clicked_y as isize;
                clicked_x = 12 - clicked_x as isize;

                //But these are the coordinates on the screen..
                //Now they have to be translated into world coordinates
                //Which I dont really know how to do right now...


                //camera_offsets should update where the bottom left corner is in relation 

                let camera_offsets = world_camera.offsets();

                //Make these then loop when crossing over the boundary.
                clicked_x += camera_offsets.1; 
                clicked_y += camera_offsets.0;

                println!("offset coord of hex is: {}, {}", clicked_y, clicked_x);

                println!("offset coord of hex is: {}, {}", clicked_y, clicked_x);

                //world_vec[(clicked_x) as usize][(clicked_y-2) as usize].set_biome(6);
                //world_vec[(clicked_x) as usize][(clicked_y+2) as usize].set_biome(6);
                world_vec[(clicked_x) as usize][(clicked_y) as usize].set_biome(7);
                draw_functions::update_hex_map_colors(&mut per_instance, &world_vec, world_camera.offsets(),screen_size);
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
                    println!("Camera pos is: {:#?}", camera.get_pos());
                    //inverse_mat = Mat4::inverse(&(Mat4::from_cols_array_2d(&perspective)*camera_matrix*Mat4::IDENTITY));
                }
                else if event.physical_key == keyboard::KeyCode::KeyE{
                    camera.r#move(50.0*CAMERA_SPEED*camera.get_front());
                    camera_matrix = camera.look_at(camera.get_pos()+camera.get_front());
                    println!("Camera pos is: {:#?}", camera.get_pos());
                    //inverse_mat = Mat4::inverse(&(Mat4::from_cols_array_2d(&perspective)*camera_matrix*Mat4::IDENTITY));
                }else if event.physical_key == keyboard::KeyCode::KeyU && event.state.is_pressed(){
                    world_camera.move_camera(0, 1);
                    draw_functions::update_hex_map_colors(&mut per_instance, &world_vec, world_camera.offsets(),screen_size);
                }
                else if event.physical_key == keyboard::KeyCode::KeyH && event.state.is_pressed(){
                    world_camera.move_camera(2, 0);
                    draw_functions::update_hex_map_colors(&mut per_instance, &world_vec, world_camera.offsets(),screen_size);
                }
                else if event.physical_key == keyboard::KeyCode::KeyJ && event.state.is_pressed(){
                    world_camera.move_camera(0, -1);
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
                //inverse_mat = Mat4::inverse(&(Mat4::from_cols_array_2d(&perspective)*camera_matrix*Mat4::IDENTITY));
                display.resize(window_size.into());
                width_scale = window_size.width as f64/ std_width;
                height_scale = window_size.height as f64/ std_height;
                println!("Scale factors are: {} and {}", width_scale, height_scale);
            },
            winit::event::WindowEvent::RedrawRequested => {
                //println!("Redraw requested");
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

                target.clear_color_and_depth((0.1, 0.4, 0.2, 1.0), 1.0);
                target.draw(
                    (&hex_renderer.vbo, per_instance.per_instance().unwrap()),
                    &hex_renderer.indicies,
                    &hex_renderer.program,
                    &uniform! { model: hex_size_mat, projection: perspective, view:camera_matrix.to_cols_array_2d()},
                    &glium::DrawParameters {
                        depth: glium::Depth {
                            test: glium::DepthTest::IfLess,
                            write: true,
                            .. Default::default()
                        },
                      //polygon_mode: glium::draw_parameters::PolygonMode::Line,
                      line_width: Some(3.0),
                      point_size: Some(4.0),
                        .. Default::default()
                    },
                ).unwrap();
                //trig_renderer.draw(&mut target, Some(&params), Some(&uniform! { model: obj_size, projection: perspective, view:camera_matrix.to_cols_array_2d(), u_light:light}));
                //hex_renderer.draw(&mut target, Some(&params), Some(&uniform!{matrix: hex_size, perspective: perspective}));


                target.finish().unwrap();
                //println!("Time for drawing frame: {} ms", dur2.elapsed().as_millis());
            },
            _ => (),
            },
            winit::event::Event::AboutToWait => {
                window.request_redraw();
            },
            _ => (),
        };

        // I think this solution is broken. 
        // Can get stuck in infinite screen or something
        // Works for now but needs to be fixed...
        //println!("One frame took {} ms\n", now.elapsed().as_millis());
        frames += 1.0;

    });
}
