/*
    Entity Manager
    The entity manager is used to interact with entities.
    It is used to add entities to the game, to invalidate or search across them, and get values from them.
    Nobody should store refs to entities, they have to fetch them from the entity manager by vid instead.
*/

use crate::{
    entity::{Entity, EntityType},
    entity_initializers::*,
};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct EntityVID {
    pub id: usize,
    pub version: u32,
}
pub const MAX_NUM_ENTITIES: usize = 128;
pub struct EntityManager {
    pub entities: Vec<Entity>,
    pub active: [bool; MAX_NUM_ENTITIES],
    pub versions: [u32; MAX_NUM_ENTITIES],
    pub available_ids: Vec<usize>,
}

impl EntityManager {
    pub fn new() -> Self {
        let mut entities = Vec::with_capacity(MAX_NUM_ENTITIES);
        let mut available_ids = Vec::with_capacity(MAX_NUM_ENTITIES);
        for i in 0..MAX_NUM_ENTITIES {
            entities.push(Entity::new());
            available_ids.push(i as usize);
        }
        Self {
            entities: entities,
            active: [false; MAX_NUM_ENTITIES],
            versions: [0; MAX_NUM_ENTITIES],
            available_ids: available_ids,
        }
    }

    pub fn set_inactive(&mut self, vid: EntityVID) {
        if vid.version == self.versions[vid.id] {
            self.active[vid.id] = false;
            self.available_ids.push(vid.id);
            // println!("Entity Manager: Entity {} set inactive", vid.id);
        }
    }

    pub fn new_entity(&mut self, entity_type: EntityType) -> Option<EntityVID> {
        let id = match self.available_ids.pop() {
            Some(id) => {
                let vid = EntityVID {
                    id: id,
                    version: self.versions[id],
                };
                self.active[id] = true;
                self.initialize_entity(vid, entity_type);
                return Some(vid);
            }
            None => {
                println!("Entity Manager: No more entities available");
                return None;
            }
        };
    }

    pub fn get_entity(&mut self, vid: EntityVID) -> Option<&Entity> {
        if vid.version == self.versions[vid.version as usize] && self.active[vid.id] {
            let entity = &self.entities[vid.id as usize];
            return Some(entity);
        }
        None
    }

    pub fn get_mut_entity(&mut self, vid: EntityVID) -> Option<&mut Entity> {
        if vid.version == self.versions[vid.version as usize] && self.active[vid.id] {
            let entity = &mut self.entities[vid.id as usize];
            return Some(entity);
        }
        None
    }

    pub fn get_entities(&mut self) -> &mut Vec<Entity> {
        &mut self.entities
    }

    pub fn initialize_entity(&mut self, vid: EntityVID, entity_type: EntityType) {
        let entity = self.get_mut_entity(vid).unwrap();
        entity.0 = entity_type;
        match entity_type {
            EntityType::Player => {
                initialize_player(entity);
            }
            EntityType::Box => {
                initialize_box(entity);
            }
            EntityType::Gold => {
                initialize_gold(entity);
            }
            EntityType::Skeleton => {
                initialize_skeleton(entity);
            }
            _ => {}
        }
    }

    // pub fn step(&mut self) {
    //     for entity in self.entities.iter_mut() {
    //         match entity.type_ {
    //             EntityType::Player => {
    //                 entity.step();
    //             }
    //             EntityType::Box => {
    //                 entity.step();
    //             }
    //             EntityType::Gold => {
    //                 entity.step();
    //             }
    //             EntityType::Skeleton => {
    //                 entity.step();
    //             }
    //             _ => {}
    //         }
    //     }
    // }
}
