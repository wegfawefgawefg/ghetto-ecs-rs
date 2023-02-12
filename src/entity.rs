use std::collections::HashSet;

use glam::{IVec2, Vec2};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum EntityType {
    None,
    Player,
    Box,
    Gold,
    Skeleton,
}

#[derive(Debug)]
pub struct Entity {
    pub type_: EntityType,
    pub has_physics: bool,
    pub can_collide: bool,

    pub pos: IVec2,
    pub vel: Vec2,
    pub acc: Vec2,

    pub size: IVec2,

    pub status_effects: HashSet<StatusEffect>,
}

impl Entity {
    pub fn new() -> Self {
        Self {
            type_: EntityType::None,
            has_physics: true,
            can_collide: true,

            pos: IVec2::new(0, 0),
            vel: Vec2::new(0.0, 0.0),
            acc: Vec2::new(0.0, 0.0),
            size: IVec2::new(1, 1),
        }
    }

    pub fn disable_physics(&mut self) {
        self.has_physics = false;
    }

    pub fn enable_physics(&mut self) {
        self.has_physics = true;
    }

    pub fn step(&mut self) {}

    pub fn step_physics(&mut self) {
        self.pos += self.vel.as_ivec2();
        self.vel += self.acc;
        self.acc = Vec2::new(0.0, 0.0);
    }
}
