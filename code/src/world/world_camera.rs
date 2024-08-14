pub struct WorldCamera{
    x_offset: isize,
    y_offset: isize,
}

impl WorldCamera{

    pub fn new() -> WorldCamera{
        WorldCamera{x_offset:0,y_offset:0}
    }

    pub fn move_camera(&mut self, move_x:isize, move_y:isize){
        self.x_offset += move_x;
        self.y_offset += move_y;
    } 

    pub fn offsets(&self) -> (isize,isize){
        (self.x_offset,self.y_offset)
    }

}