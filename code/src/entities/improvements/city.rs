use crate::entities::entity_base::BaseEntity;

use super::building::Building;

pub struct City<'a>{
    buildings: Vec<Building<'a>>,
    entity: BaseEntity,
}