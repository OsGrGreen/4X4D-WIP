#[macro_use]
extern crate glium;
extern crate winit;
use rand::distr::{Distribution, Uniform};
use glam::{Vec3};
use util::{input_handler::InputHandler, ray_library::ndc_to_intersection};
use winit::{event_loop::{ControlFlow, EventLoop}, keyboard, window::{Fullscreen, Window, WindowBuilder}};
use glium::{glutin::surface::WindowSurface, implement_vertex, uniforms::{MagnifySamplerFilter, MinifySamplerFilter}, Display, Surface};
use world::{draw_functions::{self, BIOME_TO_TEXTURE}, hex::Hex, layout::{HexLayout, Point, EVEN}, offset_coords::qoffset_from_cube, tile::Tile, world_camera::WorldCamera, NUM_COLMS, NUM_ROWS};
use UI::{button::ButtonType, button_handler::ButtonHandler, InfoBox::InfoBox};
use std::{os::windows::thread, thread::sleep, time::{Duration, Instant}};


mod rendering;
use rendering::{render::{self, array_to_vbo, Vertex}, render_camera::RenderCamera, text::{format_to_exact_length, RenderedText}};


mod improvements;
mod util;
mod player;
mod world;
mod units;
mod UI;
mod states;


#[derive(Copy, Clone, Debug)]
struct Attr {
    world_position: [f32; 3],
    colour: [f32; 3], // Changed to array
    tex_offsets: [f32;3], //x offset, y offset, scaling factor          For reading in texture atlas
}
implement_vertex!(Attr, world_position, colour, tex_offsets);


fn _pointy_hex_corner(center: Point, size: usize, i: i32) -> Point {
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


    println!("world vec length is {:#?} x {:#?}", world_vec.len(), world_vec[0].len());
    world_vec[12][25].set_biome(6);
    world_vec[13][25].improve();
    world_vec[12][26].improve();

    // Closest camera can be: z = 2.15
    // Furtherst camera can be: z = 4.85

    let mut camera = RenderCamera::new(Vec3::new(0.0,0.0,4.5), Vec3::new(0.0,0.0,0.0), Vec3::new(0.0, 1.0, 0.0), Vec3::new(0.0,0.0,-1.0));

    //Camera constants

    const CAMERA_SPEED:f32 = 0.008;

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
    let layout = HexLayout::new_flat(Point{x:hex_size/hex_scale,y:hex_size/hex_scale},Point{x:0.0,y:0.0});
    let corners = layout.polygon_corners(&hex); 
    let mut world_camera = WorldCamera::new((NUM_ROWS, NUM_COLMS));


    let hex_vert_2 = array_to_vbo(corners);
    
    println!("verts for hex is {:#?}", hex_vert_2);
    //println!("hexvert is {:#?}", hex_vert_2.len());

    let hex_indecies_fan: [u16; 7] = [ 
        0, 1, 2, 3, 4 , 5, 0
        ];

    let cup_verts = util::read_model("./models/hex.obj");
    let vert_shad = util::read_shader("./shaders/vert1.glsl");
    let vert_shad_2 = util::read_shader("./shaders/vert2.glsl");
    let frag_shad_1 = util::read_shader("./shaders/frag1.glsl");
    let frag_shad_2 = util::read_shader("./shaders/frag2.glsl");
    let line_vert_shader = util::read_shader("./shaders/line_vert.glsl");
    let line_frag_shader = util::read_shader("./shaders/line_frag.glsl");

    // Setup specific parameters

    let line_params = glium::DrawParameters {
        depth: glium::Depth {
            test: glium::DepthTest::IfLess,
            write: true,
            .. Default::default()
        },
      polygon_mode: glium::draw_parameters::PolygonMode::Line,
      line_width: Some(5.0),
        .. Default::default()
    };

    let text_params = glium::DrawParameters {
        blend: glium::Blend::alpha_blending(),
        .. Default::default()
    };

    //Read textures
    let tile_texture_atlas_image = image::load(std::io::Cursor::new(&include_bytes!(r"textures\texture_atlas_tiles_2.png")),
                        image::ImageFormat::Png).unwrap().to_rgba8();
    let image_dimensions = tile_texture_atlas_image.dimensions();
    let tile_texture_atlas_image = glium::texture::RawImage2d::from_raw_rgba_reversed(&tile_texture_atlas_image.into_raw(), image_dimensions);
    let tile_texture_atlas = glium::texture::Texture2d::new(&display, tile_texture_atlas_image).unwrap();
    

            // Font chars are of size 12 x 6
    let font_raw_image = image::load(std::io::Cursor::new(&include_bytes!(r"textures\standard_font.png")),
    image::ImageFormat::Png).unwrap().to_rgba8();
    let font_dimensions = font_raw_image.dimensions();
    let font_image = glium::texture::RawImage2d::from_raw_rgba_reversed(&font_raw_image.into_raw(), font_dimensions);
    let font_atlas = glium::texture::Texture2d::new(&display, font_image).unwrap();

    //Setup render programs
    let hex_renderer = rendering::render::Renderer::new(hex_vert_2, hex_indecies_fan.to_vec(), Some(glium::index::PrimitiveType::TriangleFan), &vert_shad, &frag_shad_1, None, &display, None).unwrap();
    let _trig_renderer = rendering::render::Renderer::new(cup_verts, vec![], None, &vert_shad_2, &frag_shad_2, None, &display, None).unwrap();
    let mut line_renderer = rendering::render::Renderer::new_empty_dynamic(100, Some(glium::index::PrimitiveType::LinesList), &line_vert_shader, &line_frag_shader, None, &display, Some(line_params)).unwrap();
    let mut ui_renderer = rendering::render::Renderer::new_empty_dynamic(100, Some(glium::index::PrimitiveType::TrianglesList), &line_vert_shader, &line_frag_shader, None, &display, None).unwrap();
    let mut text_renderer = rendering::render::Renderer::new_empty_dynamic(256, Some(glium::index::PrimitiveType::TrianglesList), &line_vert_shader, &line_frag_shader, None, &display, Some(text_params)).unwrap();
   


    let mut button_handler = ButtonHandler::new();
    //button_handler.add_button((0.0,0.0), 0.1, 0.1, 0, ButtonType::Open);
    button_handler.render(None, &mut ui_renderer);
    let mut fps_box = InfoBox::new(0, (0.85,0.95), 0.05*8.0, 0.05, 0);
    fps_box.add_text(RenderedText::new(String::from("00000fps"), 0, 0.035, (0.85,0.95)));
    fps_box.render([0.0,0.0,0.0], [1.0, 0.5,1.0], &mut ui_renderer, &mut text_renderer);
    //text_renderer.render_text(Some([1.0,0.5,1.0]),&mut fps_text);




    /*text_renderer.render_text((-0.5,0.5), 0.1,Some([1.0,0.3,1.0]),&mut hello_world_text_2);
    println!("Text is: {:#?}", hello_world_text);
    hello_world_text.change_text("Gijjo Oskar???");
    println!("Text is: {:#?}", hello_world_text);
    text_renderer.replace_text(hello_world_text);
    */
    
    // Add some stuff to the renderers

    //ui_renderer.draw_rectangle_with_texture((0.0, 0.0), 0.2, 0.2, None, 0);


    // Uniform setup
        // Text uniforms
    let text_behavior = glium::uniforms::SamplerBehavior {
        minify_filter: MinifySamplerFilter::Nearest,
        magnify_filter: MagnifySamplerFilter::Nearest,
        ..Default::default()
    };

    let _light = [-1.0, 0.4, 0.9f32];

    let mut perspective = rendering::render::calculate_perspective(window.inner_size().into());
    let mut frames:f32 = 0.0;


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
                tex_offsets: [0.0,0.0,1.0],
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
    let mut mouse_ndc: Vec3 = Vec3::ZERO;

    let mut timer = Instant::now();
    let mut count_times_between_rendered_frames = Instant::now();
    let mut overall_fps = 0.0;
    let smoothing = 0.7; // larger=more smoothing
    
    let _ = event_loop.run(move |event, window_target| {

              
        //println!("timer: {}", timer2.elapsed().as_millis());

        //Delta time calculation may be wrong...
        //There is kinda of stuttery movement...
        //Could also be movement calculations...
        let delta_time = (timer.elapsed().as_micros() as f32/1000.0).clamp(0.005, 1000.0);
        //println!("{}",timer.elapsed().as_micros() as f32 / 1000000.0);
        // Get fps
        let current = 1.0 / (timer.elapsed().as_micros() as f32 / 1000000.0) ;
        overall_fps = ((overall_fps * smoothing) + (current * (1.0-smoothing))).min(50_000.0);
        let fps_as_text = format_to_exact_length(overall_fps as u32, 5) + "fps";
        fps_box.texts[0].change_text(fps_as_text);
        //println!("Is gonna replace");
        text_renderer.replace_text(&fps_box.texts[0]);
        timer = Instant::now();

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
                //let update_hex_map_timer = Instant::now();
                draw_functions::update_hex_map_colors(&mut per_instance, &world_vec, world_camera.offsets(),screen_size);
                //println!("Updating hex map took {} ms", update_hex_map_timer.elapsed().as_millis());
            }
            let intersect = ndc_to_intersection(&mouse_ndc,&camera_matrix,camera.get_pos(),&perspective);
            mouse_pos.x = intersect.x as f32;
            mouse_pos.y = intersect.y as f32;
            camera_matrix = camera.look_at(camera.get_pos()+camera.get_front());
            window.request_redraw();
            //inverse_mat = Mat4::inverse(&(Mat4::from_cols_array_2d(&perspective)*camera_matrix*Mat4::IDENTITY));
        }


        match event {
            winit::event::Event::WindowEvent { event, .. } => match event {
            winit::event::WindowEvent::CloseRequested => window_target.exit(),
            winit::event::WindowEvent::CursorMoved { device_id: _, position } => {
                

                // Still some problem with this code?
                // Could probably be some rounding errors...
                // How could one fix this?
                // Scale everything maybe to use bigger numbers?
                mouse_ndc = Vec3::new(
                    (position.x as f32 / window.inner_size().width as f32) * 2.0 - 1.0,
                    -((position.y as f32 / window.inner_size().height as f32) * 2.0 - 1.0),
                    0.0,
                );

                let intersect = ndc_to_intersection(&mouse_ndc,&camera_matrix,camera.get_pos(),&perspective);


                mouse_pos.x = intersect.x as f32;
                mouse_pos.y = intersect.y as f32;
            }
            winit::event::WindowEvent::MouseInput { device_id: _, state, button } =>{
                if state.is_pressed(){

                    let pressed_button = button_handler.get_pressed_button(&(mouse_ndc.x,mouse_ndc.y));
                    if pressed_button.is_some(){
                        let but = pressed_button.unwrap();
                        println!("{:?}", but);                        
                    }else{
                        //println!("Dimension is: {:#?}", window.inner_size());
                        let frac_hex = layout.pixel_to_hex(&mouse_pos);
                        let clicked_hex = frac_hex.hex_round();
                        
                        let (mut clicked_y, mut clicked_x) = qoffset_from_cube(EVEN,&clicked_hex);                    
                        
                        //Make these not hard coded...
                        // And move out into seperate function
                        clicked_y = 25 - clicked_y as isize;
                        clicked_x = 12 - clicked_x as isize;
        
                        let camera_offsets = world_camera.offsets();
        
                        //Make these then loop when crossing over the boundary.
                        clicked_x += camera_offsets.1; 
                        clicked_y += camera_offsets.0;

                        if clicked_x <= 0{
                            clicked_x = ((NUM_COLMS) as isize + clicked_x) % NUM_COLMS as isize;
                        }else if clicked_x >= NUM_COLMS as isize{
                            clicked_x = (clicked_x - (NUM_COLMS) as isize) % NUM_COLMS as isize;
                        }  
                        

                        if clicked_y <= 0{
                            clicked_y = ((NUM_ROWS) as isize + clicked_y) % NUM_ROWS as isize;
                        }else if clicked_y >= NUM_ROWS as isize{
                            clicked_y = (clicked_y - (NUM_ROWS) as isize) % NUM_ROWS as isize;
                        }  

                        // Do not do the update here (add it to the job queue)
                        let clicked_tile = world_vec[(clicked_x) as usize][(clicked_y) as usize];
                        world_vec[(clicked_x) as usize][(clicked_y) as usize].set_improved(!clicked_tile.get_improved());
                        draw_functions::update_hex_map_colors(&mut per_instance, &world_vec, world_camera.offsets(),screen_size);
                        println!("Biome is: {} and texture coords are {:#?}", clicked_tile.get_biome(), BIOME_TO_TEXTURE[clicked_tile.get_biome() as usize]);
                    }
                    

                    //line_renderer.draw_line((0.0,0.0),(mouse_ndc.x,mouse_ndc.y), None);
                }
            }

            // TODO
            // Make input a little bit nicer
            winit::event::WindowEvent::KeyboardInput { device_id: _, event, is_synthetic: _ } =>{

                //Handle other inputs
                if event.physical_key == keyboard::KeyCode::Escape && event.state.is_pressed(){
                    window_target.exit()
                } 
                else if event.physical_key == keyboard::KeyCode::KeyQ && event.state.is_pressed(){
                    camera.r#move(50.0*-CAMERA_SPEED*camera.get_front());
                    camera_matrix = camera.look_at(camera.get_pos()+camera.get_front());
                    //inverse_mat = Mat4::inverse(&(Mat4::from_cols_array_2d(&perspective)*camera_matrix*Mat4::IDENTITY));
                }
                else if event.physical_key == keyboard::KeyCode::KeyE{
                    camera.r#move(50.0*CAMERA_SPEED*camera.get_front());
                    camera_matrix = camera.look_at(camera.get_pos()+camera.get_front());
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
                println!("Time since last rendered frame: {}", count_times_between_rendered_frames.elapsed().as_secs_f32());
                
                let dur2 = Instant::now();
                //time += 0.02;

                //let x_off = time.sin() * 0.5;

                let mut target = display.draw();

                target.clear_color_and_depth((0.1, 0.4, 0.2, 1.0), 1.0);
                target.draw(
                    (&hex_renderer.vbo, per_instance.per_instance().unwrap()),
                    &hex_renderer.indicies,
                    // For different hexes make a texture atlas so a specific tile has a texture in the atlas
                    // Then each instance have different UV coords! 
                    &hex_renderer.program,
                    &uniform! { model: hex_size_mat, projection: perspective.to_cols_array_2d(), view:camera_matrix.to_cols_array_2d(), tex: &tile_texture_atlas},
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
                target.draw(&line_renderer.vbo, &line_renderer.indicies, &line_renderer.program, &uniform! {}, &line_renderer.draw_params).unwrap();
                target.draw(&ui_renderer.vbo, &ui_renderer.indicies, &ui_renderer.program, &uniform! {tex:&font_atlas}, &Default::default()).unwrap();
                target.draw(&text_renderer.vbo, &text_renderer.indicies, &text_renderer.program, &uniform! {tex:glium::uniforms::Sampler(&font_atlas, text_behavior)}, &text_renderer.draw_params).unwrap();
                //trig_renderer.draw(&mut target, Some(&params), Some(&uniform! { model: obj_size, projection: perspective, view:camera_matrix.to_cols_array_2d(), u_light:light}));
                //hex_renderer.draw(&mut target, Some(&params), Some(&uniform!{matrix: hex_size, perspective: perspective}));
                //println!("\t\tUploading info to GPU took: {} ms", dur2.elapsed().as_millis());
                //sleep(Duration::from_millis(14));
                target.finish().unwrap();
                count_times_between_rendered_frames = Instant::now();
                //println!("\t\tTime for drawing frame: {} ms\n", dur2.elapsed().as_millis());
            },
            _ => (),
            },
            winit::event::Event::AboutToWait => {
                window.request_redraw();
            },
            _ => (),
        };

        //This breaks the text_renderer for some reason...
        /*if timer.elapsed().as_micros() as f32/ 1000.0 <= 16.7{
            let sleep_time = 16.7 - timer.elapsed().as_micros() as f32/ 1000.0;
            sleep(Duration::from_secs_f32(sleep_time/1000.0));
        }*/

        // I think this solution is broken. 
        // Can get stuck in infinite screen or something
        // Works for now but needs to be fixed...
        //println!("One frame took {} ms\n", now.elapsed().as_millis());
        frames = frames + 1.0;
    });
}
