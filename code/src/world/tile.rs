pub struct Tile{
    information: u8,
}

impl Tile{
    pub fn new(biome:u16, resource: u16) -> Tile{
        let tile = biome << 5 | resource << 1;
        Tile { information: tile as u8 }
    }

    pub fn get_biome(&self) -> u8{
        return self.information >> 5 & 7
    }

    pub fn get_improved(&self) -> u8{
        return self.information >> 4 & 1
    }

    pub fn get_resource(&self) -> u8{
        return self.information >> 1 & 7
    }

    pub fn get_occupied(&self) -> u8{
        return self.information & 1
    }

    pub fn set_biome(&mut self, new_biome: u8){
        assert!(new_biome < 8);
        self.information = (self.information & 31) | (new_biome << 5);
    }

    pub fn set_improved(&mut self, improved: u8){
        self.information = (self.information & 239) | ((improved & 1) << 4);
    }

    pub fn set_resource(&mut self, resource: u8){
        assert!(resource < 8);
        self.information = (self.information & 241) | (resource << 1);
    }

    pub fn set_occupied(&mut self, occupied: u8){
        self.information = (self.information & 254) | (occupied & 1);
    }

}