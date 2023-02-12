use crate::entity::{Entity, EntityType};

pub fn initialize_player(entity: &mut Entity) {
    entity.type_ = EntityType::Player;
    entity.has_physics = true;
    entity.can_collide = true;
}

pub fn initialize_box(entity: &mut Entity) {
    entity.type_ = EntityType::Box;
    entity.has_physics = true;
    entity.can_collide = true;
}

pub fn initialize_gold(entity: &mut Entity) {
    entity.type_ = EntityType::Gold;
    entity.has_physics = true;
    entity.can_collide = true;
}

pub fn initialize_skeleton(entity: &mut Entity) {
    entity.type_ = EntityType::Skeleton;
    entity.has_physics = true;
    entity.can_collide = true;
}
