#[derive(Clone, Debug)]
pub struct RenderedText{
    pub text: String,
    pub vertex_start: u32,
    pub index_start: u16,
    pub vertex_end: u32,
}

impl RenderedText{

    pub fn new(text: String) -> RenderedText{
        RenderedText{
            text:text,
            vertex_start: 0,
            index_start: 0,
            vertex_end: 0,
        }
    }

    pub fn change_text(&mut self, new_text: String){
        self.text = new_text;
    }

}

pub fn format_to_exact_length(number: u32, length: usize) -> String {
    let mut num_str = number.to_string();

    // Truncate if necessary
    if num_str.len() > length {
        num_str.truncate(length);
    }

    // Pad with the specified character if necessary
    if num_str.len() < length {
        num_str = format!("{:0width$}", number, width = length)
    }

    num_str
}