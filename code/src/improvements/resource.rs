pub struct Resource<'a>{
    pub id: u32,
    pub name:&'a str,
}

impl<'a> Resource<'a>{

    pub fn new(id: u32, name:&'a str) -> Resource{
        Resource{
            id:id,
            name:name,
        }
    }

}

pub struct ResourceCounter<'a>{
    pub r#type: Resource<'a>,
    player_id: u32, //Maybe not needed since you do not need to store what player has what resources. But could be nice for the server ig
    quantity: i32,
    capacity: i32,
    production_rate: i32,
    consumption_rate: i32,
}

impl<'a> ResourceCounter<'a>{

    pub fn new(resource:Resource<'a>, player: u32, inital_quantity:i32, initial_capacity:i32) -> ResourceCounter{
        ResourceCounter{
            r#type:resource,
            player_id: player,
            quantity: inital_quantity,
            capacity: initial_capacity,
            production_rate: 0,
            consumption_rate: 0,
        }
    }

    pub fn update(&mut self){
        self.quantity = (self.quantity + self.production_rate - self.consumption_rate).min(self.capacity);
    }

    pub fn update_quantity(&mut self, update_val: i32){
        self.quantity += update_val;
    }

    pub fn change_quantity(&mut self, update_val: i32){
        self.quantity = update_val;
    }

    pub fn update_capacity(&mut self, update_val: i32){
        self.capacity += update_val;
    }

    pub fn change_capacity(&mut self, update_val: i32){
        self.capacity = update_val;
    }

    pub fn update_production_rate(&mut self, update_val: i32){
        self.production_rate += update_val;
    }

    pub fn change_production_rate(&mut self, update_val: i32){
        self.production_rate = update_val;
    }

    pub fn update_consumption_rate(&mut self, update_val: i32){
        self.consumption_rate += update_val;
    }

    pub fn change_consumption_rate(&mut self, update_val: i32){
        self.consumption_rate = update_val;
    }

    pub fn get_quantity(&self) -> i32{
        self.quantity
    }

    pub fn get_capacity(&self) -> i32{
        self.capacity
    }

    pub fn get_production_rate(&self) -> i32{
        self.production_rate
    }

    pub fn get_consumption_rate(&self) -> i32{
        self.consumption_rate
    }

    pub fn get_player_id(&self) -> u32{
        self.player_id
    }



}
