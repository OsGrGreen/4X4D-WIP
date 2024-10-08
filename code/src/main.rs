#[macro_use]
extern crate glium;
extern crate winit;
use entities::{entity_vertex_buffer::{EntityPosAttr, EntityVBO}, improvements, units::{self, unit::UnitType}, EntityHandler, EntityMap};
use improvements::{city::City, resource::Resource};
use rand::distr::{Distribution, Uniform};
use glam::{Vec3};
use units::unit::BaseUnit;
use util::{input_handler::{self, InputHandler}, ray_library::ndc_to_intersection};
use winit::{event_loop::{ControlFlow, EventLoop}, keyboard, window::{Fullscreen, Window}};
use glium::{glutin::surface::WindowSurface, implement_vertex, uniforms::{MagnifySamplerFilter, MinifySamplerFilter}, Display, Surface, VertexBuffer};
use world::{draw_functions::{self, World_Attr, World_Pos, BIOME_TO_TEXTURE}, hex::Hex, layout::{HexLayout, Point, EVEN}, offset_coords::{qoffset_from_cube, qoffset_from_cube_offsets, qoffset_to_cube, qoffset_to_cube_offsets}, tile::Tile, world_camera::WorldCamera, OffsetTile, NUM_COLMS, NUM_ROWS};
use std::{any::Any, collections::HashMap, mem, os::windows::thread, thread::sleep, time::{Duration, Instant}};


mod rendering;
use rendering::{render::{self, array_to_vbo, Vertex}, render_camera::RenderCamera, text::{format_to_exact_length, RenderedText, TextVbo}};


mod util;
mod player;
mod world;
mod UI;

mod entities;

const MAX_UNITS:usize = 100;


#[derive(Copy, Clone, Debug)]
pub struct Attr {
    world_position: [f32; 3],
    colour: [f32; 3], // Changed to array
    tex_offsets: [f32;3], //x offset, y offset, scaling factor          For reading in texture atlas
}
implement_vertex!(Attr, world_position, colour, tex_offsets);

impl Attr{
    pub fn is_zero(&self) -> bool{
        if self.colour == [0.0,0.0,0.0]{
            return true
        }else{
            return false
        }
    }
}

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

//Camera constants

const CAMERA_SPEED:f32 = 2.0;

const CONSTANT_FACTOR:f32 = 1.0;
fn main() {

    /*println!("Base Entity: {:?} bytes",mem::size_of::<BaseEntity>());
    println!("Unit: {:?} bytes",mem::size_of::<BaseUnit>());
    println!("City: {:?} bytes",mem::size_of::<City>());
    println!("UnitVBO: {:?} bytes",mem::size_of::<UnitVbo>());
    println!("Resource: {:?} bytes",mem::size_of::<Resource>());
    */


    //First value is what row, second value is what column
    // 0,0 is bottom left corner


    let mut world_vec: Vec<Vec<Tile>> = vec![vec![]];
    let mut rng: rand::prelude::ThreadRng = rand::thread_rng();
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
    //world_vec[12][25].set_biome(6);
    world_vec[0][0].set_biome(6);
    //world_vec[10][0].set_biome(6);
    //world_vec[13][25].improve();
    //world_vec[12][26].improve();

    

    // Closest camera can be: z = 2.15
    // Furtherst camera can be: z = 4.85

    let mut camera = RenderCamera::new(Vec3::new(0.0,0.0,4.5), Vec3::new(0.0,0.0,0.0), Vec3::new(0.0, 1.0, 0.0), Vec3::new(0.0,0.0,-1.0));

    // Input handler

    let mut input_handler = InputHandler::new();

    camera.camera_matrix = camera.look_at(camera.get_pos()+camera.get_front());
    //println!("camera matrix glm is {:#?}", RenderCamera::look_at_glm(Vec3::new(2.0,-1.0,1.0), Vec3::new(-2.0,1.0,1.0),Vec3::new(0.0,1.0,0.0)));
    //println!("camera matrix is: {:#?}", camera.camera_matrix);
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

    println!("constant_factor is {}", CONSTANT_FACTOR);
    

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

    let unit_vert_shader = util::read_shader("./shaders/unit_vert.glsl");
    let unit_frag_shader = util::read_shader("./shaders/unit_frag.glsl");

    let text_vert_shader  = util::read_shader("./shaders/text_vert.glsl");
    let text_frag_shader  = util::read_shader("./shaders/text_frag.glsl");

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
        //Tile textures
    let tile_texture_atlas_image = image::load(std::io::Cursor::new(&include_bytes!(r"textures\texture_atlas_tiles.png")),
                        image::ImageFormat::Png).unwrap().to_rgba8();
    let image_dimensions = tile_texture_atlas_image.dimensions();
    let tile_texture_atlas_image = glium::texture::RawImage2d::from_raw_rgba_reversed(&tile_texture_atlas_image.into_raw(), image_dimensions);
    let tile_texture_atlas = glium::texture::Texture2d::new(&display, tile_texture_atlas_image).unwrap();
    
        //Font textures
            // Font chars are of size 12 x 6
    let font_raw_image = image::load(std::io::Cursor::new(&include_bytes!(r"textures\standard_font.png")),
    image::ImageFormat::Png).unwrap().to_rgba8();
    let font_dimensions = font_raw_image.dimensions();
    let font_image = glium::texture::RawImage2d::from_raw_rgba_reversed(&font_raw_image.into_raw(), font_dimensions);
    let font_atlas = glium::texture::Texture2d::new(&display, font_image).unwrap();

        //Unit textures
    let unit_raw_image = image::load(std::io::Cursor::new(&include_bytes!(r"textures\unit_atlas.png")), image::ImageFormat::Png).unwrap().to_rgba8();
    let unit_dimensions = unit_raw_image.dimensions();
    let unit_image = glium::texture::RawImage2d::from_raw_rgba_reversed(&unit_raw_image.into_raw(), unit_dimensions);
    let unit_atlas = glium::texture::Texture2d::new(&display, unit_image).unwrap();
    

    //Shape of quad
    let quad_shape:Vec<Vertex> = vec![
        Vertex{position: [-1.0*hex_size, -1.0*hex_size, 0.0], normal: [0.0,0.0,0.0], tex_coords: [0.0, 0.0]}, 
        Vertex{position: [1.0*hex_size, -1.0*hex_size, 0.0], normal: [0.0,0.0,0.0], tex_coords: [1.0, 0.0]},
        Vertex{position: [1.0*hex_size, 1.0*hex_size, 0.0], normal: [0.0,0.0,0.0], tex_coords: [1.0, 1.0]},
        Vertex{position: [-1.0*hex_size, 1.0*hex_size, 0.0], normal: [0.0,0.0,0.0], tex_coords: [0.0, 1.0]}
        ];
    
    let quad_indicies = vec![0, 2, 1, 0, 2, 3];

    //Setup render programs
    let hex_renderer = rendering::render::Renderer::new(&hex_vert_2, &hex_indecies_fan.to_vec(), Some(glium::index::PrimitiveType::TriangleFan), &vert_shad, &frag_shad_1, None, &display, None).unwrap();
    let _trig_renderers = rendering::render::Renderer::new(&cup_verts, &vec![], None, &vert_shad_2, &frag_shad_2, None, &display, None).unwrap();
    
    let mut unit_renderer = rendering::render::Renderer::new(&quad_shape, &quad_indicies, Some(glium::index::PrimitiveType::TrianglesList), &unit_vert_shader, &unit_frag_shader, None, &display, Some(
        glium::DrawParameters {
            depth: glium::Depth {
                test: glium::DepthTest::IfMoreOrEqual,
                write: true,
                .. Default::default()
            },
            blend: glium::Blend::alpha_blending(),
                .. Default::default()
        })).unwrap();
    
    let mut line_renderer = rendering::render::Renderer::new_empty_dynamic(100, Some(glium::index::PrimitiveType::LinesList), &line_vert_shader, &line_frag_shader, None, &display, Some(line_params)).unwrap();
    let mut ui_renderer = rendering::render::Renderer::new_empty_dynamic(100, Some(glium::index::PrimitiveType::TrianglesList), &line_vert_shader, &line_frag_shader, None, &display, None).unwrap();
    let mut text_renderer = rendering::render::Renderer::new(&quad_shape, &quad_indicies, Some(glium::index::PrimitiveType::TrianglesList), &text_vert_shader, &text_frag_shader, None, &display, Some(text_params)).unwrap();
    
    
    let mut fps_text = RenderedText::new(String::from("00000fps"));
    let mut text_vbo = TextVbo::new(100, &display);
    text_vbo.add_text((0.78,0.95), 0.085, Some([1.0,0.5,1.0]), &mut fps_text);
    // Uniform setup
        // Text uniforms
    let text_behavior = glium::uniforms::SamplerBehavior {
        minify_filter: MinifySamplerFilter::Nearest,
        magnify_filter: MagnifySamplerFilter::Nearest,
        ..Default::default()
    };

    let _light = [-1.0, 0.4, 0.9f32];

    camera.perspective = rendering::render::calculate_perspective(window.inner_size().into());
    let mut frames:f32 = 0.0;


    //println!("Window size is: {:?}", window.inner_size().width);
    //println!("Frame buffer size is: {:?}", display.get_framebuffer_dimensions().0);

    let needed_hexes_x = (((800.0) / (2.0*(100.0*layout.get_width()))) * 1.5) as i32;
    let needed_hexes_y = (((480.0) / (100.0*layout.get_height())) * 1.5) as i32;

    let mut amount_of_hexes = 0;

    // Not the most efficient or pretty way but it works..
    let mut world_pos_data: Vec<World_Pos> = vec![];
    let mut world_attr_data: Vec<World_Attr> = vec![];
    let left:i32 = -needed_hexes_y/2; //Is possible to multiply by 2, I think that will be nicer.. //However have to make draw_function faster
    let top:i32 = -((needed_hexes_x)); //Is possible to multiply by 2
    let right:i32 = left.abs();
    let bottom:i32 = top.abs();
    let mut screen_size = (bottom*2,right*2);
    println!("Screen size {:#?}", screen_size);
    //Börjar med att köra en column i taget.
    let mut unit_data: Vec<EntityPosAttr> = vec![];

    for q in top..=bottom{
        let q_offset = q>>1;
        for r in left-q_offset..=right-q_offset{

            let test_hex = Hex::new(q,r,-q-r);
            let convert = qoffset_from_cube_offsets(EVEN, &test_hex);
            let convert_convert: (u32,u32) = (convert.0 as u32, convert.1 as u32);
            let converted_hex = qoffset_to_cube_offsets(EVEN, convert_convert);

            assert!(test_hex.get_q() == converted_hex.get_q());
            assert!(test_hex.get_r() == converted_hex.get_r());

            println!("{:?} == {:?}", test_hex, converted_hex);


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
            let val_pos = World_Pos {
                world_position: [coords.x, coords.y, 0.0],
            };
            let val_data = World_Attr{
                colour: [color/2.0,color, 0.0],
                tex_offsets: [0.0,0.0,1.0],
            };
            amount_of_hexes += 1;
            world_pos_data.push(val_pos);
            world_attr_data.push(val_data);
        }
    }
    let mut entity_handler: EntityHandler = EntityHandler::new(100, &display);
    
    // Having a unit in every tile, has minimal impact on performance... 
    // 141 vs 148 average fps (not really very good measurements, maybe test this more)
    // However, having a too big of a VBO significally slows down the program
    // Solution must be to have dynamic size of the VBO, and create a bigger one if it is needed...
    println!("Layout size is: {:#?}", layout.size);
    println!("Expected layout size is {}w, {}h",layout.get_width(),layout.get_height());
    println!("data length is: {}", world_pos_data.len());
    //println!("data is: {:#?}", data);
    entity_handler.create_unit(&mut world_vec, OffsetTile::new(26,13), UnitType::worker, 0,0,0,0,0);
    entity_handler.select(OffsetTile::new(26,13));
    world_vec[26][13].set_biome(7);
    
    //println!("{:#?}", data[0]);
    println!("Amount of true hexes are: {:#?}", amount_of_hexes);

    // Maybe try to have a double buffer of some kind..
    // See: https://stackoverflow.com/questions/14155615/opengl-updating-vertex-buffer-with-glbufferdata
    let hex_pos: VertexBuffer<World_Pos> = glium::vertex::VertexBuffer::persistent(&display, &world_pos_data).unwrap();
    let mut hex_attr: VertexBuffer<World_Attr> = glium::vertex::VertexBuffer::dynamic(&display, &world_attr_data).unwrap();



    draw_functions::update_hex_map_colors(&hex_pos,&mut hex_attr, &world_vec,&mut entity_handler, world_camera.offsets(),screen_size);

    let mut mouse_pos: Point = Point{x:0.0,y:0.0};
    let mut mouse_ndc: Vec3 = Vec3::ZERO;

    let mut t: f32 = 0.0;
    let mut dt: f32 = 0.0167;

    let mut current_time = Instant::now();
    let mut accumulator: f32 = 0.0;

    let mut total_fps: usize = 0;

    let mut timer = Instant::now();
    let mut overall_fps = 0.0;
    let smoothing = 0.6; // larger=more smoothing
    let _ = event_loop.run(move |event, window_target| {
        match event {
            winit::event::Event::WindowEvent { event, .. } => match event {
            winit::event::WindowEvent::CloseRequested => {
                println!("Average fps was: {}", total_fps/frames as usize);
                window_target.exit()
            },
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

                let intersect = ndc_to_intersection(&mouse_ndc,&camera.camera_matrix,camera.get_pos(),&camera.perspective);


                mouse_pos.x = intersect.x as f32;
                mouse_pos.y = intersect.y as f32;
            }
            winit::event::WindowEvent::MouseInput { device_id: _, state, button } =>{
                if state.is_pressed(){

                    let clicked_tile = get_clicked_pos(&layout, &mut mouse_pos, &mut world_camera);

                    // Do not do the update here (add it to the job queue)
                    entity_handler.select(clicked_tile);

                    
                    if entity_handler.get_selected_unit().is_some(){
                        let unit = entity_handler.get_selected_unit().unwrap();
                        println!("Clicked pos is: {:#?}, and unit pos is: {:#?}", (clicked_tile.getX(), clicked_tile.getY()), unit.get_pos());
                        // qoffset_from_cube(EVEN,&clicked_hex);  
                        let neighbors = Hex::neighbors_in_range_offset(unit.get_pos(), unit.get_movement());
                        for neighbor in neighbors.as_slice(){
                            //Check that target_pos is not forbidden tile...
                            world_vec[neighbor.getX() as usize][neighbor.getY() as usize].set_improved(1);
                        }
                        input_handler.affected_tiles = neighbors;
                    }
                    //let clicked_tile = world_vec[(clicked_x) as usize][(clicked_y) as usize];
                    println!("Clicked tile was: {:?}, {:?}", qoffset_to_cube(EVEN, clicked_tile), (clicked_tile.getX(), clicked_tile.getY()));
                    //world_vec[(clicked_x) as usize][(clicked_y) as usize].set_improved(!clicked_tile.get_improved());
                    draw_functions::update_hex_map_colors(&hex_pos,&mut hex_attr, &world_vec,&mut entity_handler, world_camera.offsets(),screen_size);
                    //println!("Biome is: {} and texture coords are {:#?}", clicked_tile.get_biome(), BIOME_TO_TEXTURE[clicked_tile.get_biome() as usize]);

                    //line_renderer.draw_line((0.0,0.0),(mouse_ndc.x,mouse_ndc.y), None);
                }else{
                    let clicked_tile = get_clicked_pos(&layout, &mut mouse_pos, &mut world_camera);
                    entity_handler.move_unit(clicked_tile, &mut world_vec);
                    println!("{:#?}", entity_handler.entity_map.entities);
                    let _ = input_handler.affected_tiles.drain(..).for_each(|tile: OffsetTile|{
                        world_vec[tile.getX() as usize][tile.getY() as usize].set_improved(0);
                    });
                    draw_functions::update_hex_map_colors(&hex_pos,&mut hex_attr, &world_vec,&mut entity_handler, world_camera.offsets(),screen_size);
                }
            }

            // TODO
            // Make input a little bit nicer
            winit::event::WindowEvent::KeyboardInput { device_id: _, event, is_synthetic: _ } =>{

                //Handle other inputs
                if event.physical_key == keyboard::KeyCode::Escape && event.state.is_pressed(){
                    println!("Average fps was: {}", total_fps/frames as usize);
                    window_target.exit()
                } 
                else if event.physical_key == keyboard::KeyCode::KeyQ && event.state.is_pressed(){
                    camera.r#move(-CAMERA_SPEED*camera.get_front());
                    camera.camera_matrix = camera.look_at(camera.get_pos()+camera.get_front());
                    //inverse_mat = Mat4::inverse(&(Mat4::from_cols_array_2d(&camera.perspective)*camera.camera_matrix*Mat4::IDENTITY));
                }
                else if event.physical_key == keyboard::KeyCode::KeyE{
                    camera.r#move(CAMERA_SPEED*camera.get_front());
                    camera.camera_matrix = camera.look_at(camera.get_pos()+camera.get_front());
                    //inverse_mat = Mat4::inverse(&(Mat4::from_cols_array_2d(&camera.perspective)*camera.camera_matrix*Mat4::IDENTITY));
                }else if event.physical_key == keyboard::KeyCode::KeyU && event.state.is_pressed(){
                    world_camera.move_camera(0, 1);
                    draw_functions::update_hex_map_colors(&hex_pos,&mut hex_attr, &world_vec,&mut entity_handler, world_camera.offsets(),screen_size);
                }
                else if event.physical_key == keyboard::KeyCode::KeyH && event.state.is_pressed(){
                    world_camera.move_camera(2, 0);
                    draw_functions::update_hex_map_colors(&hex_pos,&mut hex_attr, &world_vec,&mut entity_handler, world_camera.offsets(),screen_size);
                }
                else if event.physical_key == keyboard::KeyCode::KeyJ && event.state.is_pressed(){
                    world_camera.move_camera(0, -1);
                    draw_functions::update_hex_map_colors(&hex_pos,&mut hex_attr, &world_vec,&mut entity_handler, world_camera.offsets(),screen_size);
                }
                else if event.physical_key == keyboard::KeyCode::KeyK && event.state.is_pressed(){
                    world_camera.move_camera(-2, 0);
                    draw_functions::update_hex_map_colors(&hex_pos,&mut hex_attr, &world_vec,&mut entity_handler, world_camera.offsets(),screen_size);
                }
                //Handle WASD

                input_handler.update_input(event);

            },
            winit::event::WindowEvent::Resized(window_size) => {
                camera.perspective = rendering::render::calculate_perspective(window_size.into());
                //inverse_mat = Mat4::inverse(&(Mat4::from_cols_array_2d(&camera.perspective)*camera.camera_matrix*Mat4::IDENTITY));
                display.resize(window_size.into());
                width_scale = window_size.width as f64/ std_width;
                height_scale = window_size.height as f64/ std_height;
                println!("Scale factors are: {} and {}", width_scale, height_scale);
            },
            winit::event::WindowEvent::RedrawRequested => {
                //Physics step
                
                let new_time = Instant::now();
                let mut frame_time = current_time.elapsed().as_secs_f32() - new_time.elapsed().as_secs_f32();

                if frame_time > 0.25{
                    frame_time = 0.25;
                }
                current_time = new_time;

                accumulator += frame_time;
                //Looks more stuttery, which I do not like
                //If we had some way to compare and interpolate states it would probably be fine but alas.
                // Could interpolate camera posistion (as long as there hasn't been a jump, is still possible but will be a little bit harder)?
                //println!("Before physics: {} ms",current_time.elapsed().as_millis());
                while accumulator >= dt {
                    //println!("Clicked Unit has ID:{:?}", entity_handler.get_selected_unit());
                    let time_update = Instant::now();

                    update_game_logic(dt, &mut camera, &mut world_camera, &layout, &world_vec, &input_handler,&hex_pos,&mut hex_attr, mouse_ndc, &mut mouse_pos, screen_size, &mut entity_handler); 
                    //println!("Update game: {} ms", time_update.elapsed().as_millis());
                    t += dt;
                    accumulator -= dt;
                }
                

                //Render step

                let time_update = Instant::now();
                //println!("Before fps-counter: {} ms",current_time.elapsed().as_millis());
                //Linear interpolation between states, cant really do it but yeah...
                //State state = currentState * alpha +  previousState * ( 1.0 - alpha );
                let delta_time = timer.elapsed().as_secs_f32();
                timer = Instant::now();
                // Get fps 
                    //This has to be done faster (is very slow now...)
                let current = 1.0 / delta_time;
                overall_fps = ((overall_fps * smoothing) + (current * (1.0-smoothing))).min(50_000.0);
                total_fps += overall_fps as usize;
                let fps_as_text = format_to_exact_length(overall_fps as u32, 5) + "fps";
                fps_text.change_text(fps_as_text);

                //It is this that takes the majority of the time
                text_vbo.replace_text(&fps_text);           
                
                //println!("Redraw requested");´
                //println!("Time for updating fps counter {}", dur2.elapsed().as_secs_f32());
                //dur2 = Instant::now();
                //println!("Time for updating game logic {}", dur2.elapsed().as_secs_f32());
                //dur2 = Instant::now();
                //time += 0.02;

                //let x_off = time.sin() * 0.5;
                //println!("Before clearing: {} ms",current_time.elapsed().as_millis());
                let mut target = display.draw();

                target.clear_color_and_depth((0.0, 0.1, 1.0, 1.0), 1.0);
                //println!("Before drawing: {} ms",current_time.elapsed().as_millis());

                target.draw(
                    (&hex_renderer.vbo,hex_pos.per_instance().unwrap(), hex_attr.per_instance().unwrap()),
                    &hex_renderer.indicies,
                    // For different hexes make a texture atlas so a specific tile has a texture in the atlas
                    // Then each instance have different UV coords! 
                    &hex_renderer.program,
                    &uniform! { model: hex_size_mat, projection: camera.perspective.to_cols_array_2d(), view:camera.camera_matrix.to_cols_array_2d(), tex: glium::uniforms::Sampler(&tile_texture_atlas, text_behavior), u_time: t},
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
                let shader_time = (t*8.0).floor()%8.0;
                //println!("Time is: {}", shader_time);
                let un_modded_pos = 0.0+0.125*shader_time;
                //println!("Pos is: {}", un_modded_pos);
                //    float animation_step = mod(tex_offsets.x+1.0*tex_offsets.z*time,animation_length);

                target.draw((&unit_renderer.vbo,entity_handler.entity_vbo.vbo.per_instance().unwrap(), entity_handler.entity_vbo.tex_vbo.per_instance().unwrap()), &unit_renderer.indicies, &unit_renderer.program, &uniform! { model: hex_size_mat, projection: camera.perspective.to_cols_array_2d(), view:camera.camera_matrix.to_cols_array_2d(), tex: glium::uniforms::Sampler(&unit_atlas, text_behavior), time: (t*8.0).floor()%8.0}, &unit_renderer.draw_params).unwrap();
                
                target.draw(&line_renderer.vbo, &line_renderer.indicies, &line_renderer.program, &uniform! {}, &line_renderer.draw_params).unwrap();
                
                target.draw(&ui_renderer.vbo, &ui_renderer.indicies, &ui_renderer.program, &uniform! {tex:&font_atlas}, &Default::default()).unwrap();
                target.draw((&text_renderer.vbo, text_vbo.vbo.per_instance().unwrap()), &text_renderer.indicies, &text_renderer.program, &uniform! {tex:glium::uniforms::Sampler(&font_atlas, text_behavior)}, &text_renderer.draw_params).unwrap();
                
                //trig_renderer.draw(&mut target, Some(&params), Some(&uniform! { model: obj_size, projection: perspective, view:camera.camera_matrix.to_cols_array_2d(), u_light:light}));
                //hex_renderer.draw(&mut target, Some(&params), Some(&uniform!{matrix: hex_size, perspective: perspective}));
                //println!("\t\tUploading info to GPU took: {} ms", dur2.elapsed().as_millis());
                //sleep(Duration::from_millis(14));
                //println!("Time for drawing {}", dur2.elapsed().as_secs_f32());
                //dur2 = Instant::now();
                //println!("Before finishing: {} ms",current_time.elapsed().as_millis());
                target.finish().unwrap();
                //println!("In total: {} ms\n",current_time.elapsed().as_millis());
                frames = frames + 1.0;
                //println!("Time for rendering to screen {}", dur2.elapsed().as_secs_f32());
                //println!("\t\tTime for drawing frame: {} ms\n", dur2.elapsed().as_millis());
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
    });
}


fn update_game_logic(delta_time: f32, camera: &mut RenderCamera,world_camera: &mut WorldCamera, layout: &HexLayout, world_vec: &Vec<Vec<Tile>>,input_handler: &InputHandler,hex_pos: &VertexBuffer<World_Pos>,hex_tiles:&mut VertexBuffer<World_Attr>,mouse_ndc:Vec3, mouse_pos: &mut Point, screen_size: (i32,i32), entity_handler: &mut EntityHandler){
    //Update movement (Kanske göra efter allt annat... possibly):
    let mut movement = input_handler.get_movement();
    if movement.length() > 0.0{
        let mut traveresed_whole_hex = false;
        movement = movement.normalize();
        //Flytta en i taget...
        camera.r#move(delta_time*movement[1]*CAMERA_SPEED*camera.get_up());
        let y_pos = camera.get_pos()[1];
        //Inte helt perfekt än måste fixa till lite....
        if y_pos < -CONSTANT_FACTOR*(3.0*(layout.get_height())){
            camera.set_y(0.0);
            world_camera.move_camera(0, -3);
            traveresed_whole_hex = true;
        } else if y_pos > CONSTANT_FACTOR*(3.0*(layout.get_height())){
            camera.set_y(0.0);
            world_camera.move_camera(0, 3);
            traveresed_whole_hex = true;
        }        
        camera.r#move(delta_time*movement[0]*CAMERA_SPEED*(camera.get_front().cross(camera.get_up())).normalize());
            let x_pos = camera.get_pos()[0];
                //Kom på varför det är 0.12 här och inget annat nummer...
                //Verkar ju bara bero på hex_size och inte scale....
            if x_pos < -CONSTANT_FACTOR*2.0*(layout.get_width()){
                camera.set_x(0.0);
                world_camera.move_camera(-2, 0);
                traveresed_whole_hex = true;
            }else if x_pos > CONSTANT_FACTOR*2.0*(layout.get_width()){
                camera.set_x(0.0);
                world_camera.move_camera(2, 0);
                traveresed_whole_hex = true;
            }
            //println!("Camera is: {}", camera.get_pos());
            //Gör så kameran bara uppdateras när man faktiskt rör på sig...
            if traveresed_whole_hex{
                let update_hex_map_timer = Instant::now();
                draw_functions::update_hex_map_colors(hex_pos,hex_tiles, world_vec,entity_handler, world_camera.offsets(),screen_size);
                //println!("Updating hex map took {} ms", update_hex_map_timer.elapsed().as_millis());
            }
            let intersect = ndc_to_intersection(&mouse_ndc,&camera.camera_matrix,camera.get_pos(),&camera.perspective);
            mouse_pos.x = intersect.x as f32;
            mouse_pos.y = intersect.y as f32;
            camera.camera_matrix = camera.look_at(camera.get_pos()+camera.get_front());
                //inverse_mat = Mat4::inverse(&(Mat4::from_cols_array_2d(&perspective)*camera_matrix*Mat4::IDENTITY));
        }
}

pub fn get_clicked_pos(layout: &HexLayout, mouse_pos: &mut Point, world_camera: &mut WorldCamera) -> OffsetTile{
    //println!("Dimension is: {:#?}", window.inner_size());
    let frac_hex = layout.pixel_to_hex(&mouse_pos);
    let clicked_hex = frac_hex.hex_round();
    
    let (mut clicked_y, mut clicked_x) = qoffset_from_cube_offsets(EVEN,&clicked_hex);                    
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

    return OffsetTile::new(clicked_y as u32, clicked_x as u32)
}