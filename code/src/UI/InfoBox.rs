use crate::{rendering::render::Renderer, RenderedText};

pub struct InfoBox{
    id: u32,
    pub texts: Vec<RenderedText>,
    screen_pos: (f32,f32),
    width: f32,
    height: f32,
    texture_id: u32,
    pub vertex_start: u32,
    pub vertex_end: u32,
}   


impl InfoBox{

    pub fn new(id: u32, screen_pos: (f32,f32), width: f32, height: f32, texture_id: u32) -> InfoBox{
        InfoBox{
            id: id,
            texts: vec![],
            screen_pos: screen_pos,
            width: width,
            height: height,
            texture_id: texture_id,
            vertex_start: 0,
            vertex_end: 0,
        }
    }

    //Make this code create a RenderedText instead of taking one...
    pub fn add_text(&mut self, text:RenderedText){
        let mut new_text = text.clone();
        new_text.screen_pos.0 = self.screen_pos.0 + self.width*text.screen_pos.0;
        new_text.screen_pos.1 = self.screen_pos.1 + self.height*text.screen_pos.1;
        self.texts.push(text);
    }

    pub fn render_text(&mut self, color:[f32;3],text_renderer: &mut Renderer){
        self.texts[0].id = 2;
    }

    pub fn render(&mut self,color_box: [f32;3], color_text: [f32;3],ui_renderer: &mut Renderer, text_renderer: &mut Renderer){

        (self.vertex_start, self.vertex_end) = ui_renderer.draw_rectangle_with_texture(self.screen_pos,self.width, self.height, Some(color_box), self.texture_id);
        
        for text in self.texts.as_mut_slice(){
            text_renderer.render_text( Some(color_text), text);
        }
    }

}