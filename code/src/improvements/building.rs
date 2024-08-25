use super::resource::Resource;

pub struct Building<'a>{
    id: u32,
    name: String,
    production_resources: Vec<(i32, Resource<'a>)>,
    cost: Vec<(u32, Resource<'a>)>
}