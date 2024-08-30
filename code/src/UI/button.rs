use crate::rendering::render::Renderer;


#[derive(Clone, Debug)]
pub enum ButtonType{
    Close,
    Open,
}

#[derive(Clone, Debug)]
pub struct Button{
    pub pos: (f32, f32),
    pub width: f32,
    pub height: f32,
    pub texture_id: u32,
    start_vbo: u32,
    button_id: u16,
    button_type: ButtonType,
    pub is_pressed: bool,
    pub color: [f32;3],
}

impl Button{
    pub fn new(pos: (f32,f32), width: f32, height:f32,texture_id: u32, button_id: u16, button_type: ButtonType, color: Option<[f32;3]>) -> Button{
        Button { pos: pos,width:width,height:height, texture_id: texture_id, start_vbo: 0,button_id: button_id, button_type: button_type, is_pressed: false,color:color.unwrap_or([1.0,1.0,1.0]) }
    }

    pub fn press_action(&mut self) -> u32{
        println!("Button {} was pressed", self.button_id);
        self.is_pressed = true;
        return 0
    }

    pub fn update_color(&mut self, ui_renderer: &mut Renderer){
        if self.is_pressed{
            self.color = [self.color[0]*0.5, self.color[1]*0.5,self.color[2]*0.5];
        }else{
            self.color = [self.color[0]*2.0, self.color[1]*2.0,self.color[2]*2.0];
        }

        ui_renderer.replace_color(self.color, self.start_vbo, self.start_vbo+4);
    }
}