use crate::rendering::render::Renderer;

use super::button::{Button, ButtonType};

pub struct ButtonHandler{
    buttons: Vec<Button>,
}

impl ButtonHandler{
    pub fn new() -> ButtonHandler{
        ButtonHandler{
            buttons: vec![],
        }
    }

    pub fn add_button(&mut self,pos:(f32,f32), width: f32, height: f32, texture_id: u32, button_type: ButtonType){
        self.buttons.push(Button::new(pos, width, height,texture_id, self.buttons.len() as u16, button_type));
    }

    pub fn get_pressed_button(&self, mouse_pos: &(f32,f32)) -> Option<&Button>{
        //Change to better algorithm
        for button in &self.buttons {
            if mouse_pos.0 >= button.pos.0 && mouse_pos.0 <= button.pos.0 + button.width &&
               mouse_pos.1 >= button.pos.1 && mouse_pos.1 <= button.pos.1 + button.height {
                button.press_action();
                return Some(button); // Return the button that was pressed
            }
        }
        return None;
    }

    pub fn render(&self, color: Option<[f32;3]>,ui_renderer: &mut Renderer){
        for button in &self.buttons{
            ui_renderer.draw_rectangle_with_texture(button.pos,button.width, button.height, color, button.texture_id);
        }

    }
}