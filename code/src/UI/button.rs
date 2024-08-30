
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
    button_id: u16,
    button_type: ButtonType,
    is_pressed: bool,
}

impl Button{
    pub fn new(pos: (f32,f32), width: f32, height:f32,texture_id: u32, button_id: u16, button_type: ButtonType) -> Button{
        Button { pos: pos,width:width,height:height, texture_id: texture_id, button_id: button_id, button_type: button_type, is_pressed: false }
    }

    pub fn press_action(&self) -> u32{
        println!("Button {} was pressed", self.button_id);

        return 0
    }
}