pub struct WorldCamera{
    x_offset: u64,
    y_offset: u64,
}

impl WorldCamera{

    pub fn new() -> WorldCamera{
        WorldCamera{x_offset:0,y_offset:0}
    }

    pub fn move_camera(&mut self, move_x:u64, move_y:u64){
        
    } 

    pub fn offsets(&self) -> (u64,u64){
        (self.x_offset,self.y_offset)
    }

}