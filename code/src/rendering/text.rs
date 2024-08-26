pub struct RenderedText<'a>{
    pub text: &'a str,
    pub vertex_start: u32,
    pub index_start: u16,
}

impl <'a> RenderedText<'a>{

    pub fn new(text: &'a str) -> RenderedText{
        RenderedText{
            text:text,
            vertex_start: 0,
            index_start: 0,
        }
    }

}