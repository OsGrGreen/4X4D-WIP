use crate::Attr;



//Make this slightly more efficient...
pub fn update_hex_map_colors(vertex_buffer: &mut glium::VertexBuffer<Attr>, new_colors: &Vec<[f32; 3]>) {
    let vertex_copy = vertex_buffer.read().unwrap();
    let mut mapping = vertex_buffer.map_write();
    
    for (i, hex) in vertex_copy.iter().enumerate() {
        mapping.set(i,Attr {
            world_position: hex.world_position,
            colour: new_colors[i],
        });
    }
}