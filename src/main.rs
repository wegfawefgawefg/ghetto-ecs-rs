use rand::Rng;

use entity::EntityType;

mod entity;
mod entity_initializers;
mod entity_manager;

pub fn main() {
    // make an entity manager
    let mut entity_manager = entity_manager::EntityManager::new();

    // make a new entity
    let player_vid = entity_manager.new_entity(EntityType::Player);
    let player = entity_manager.get_mut_entity(player_vid.unwrap()).unwrap();

    // print the entity
    println!("{:?}", player);

    // set x or y
    player.3.x = 10;
    player.3.y = 10;

    // print the entity
    println!("{:?}", player);

    loop {
        // make random amounts of entities
        let num_entities = rand::thread_rng().gen_range(0..100);
        for _ in 0..num_entities {
            entity_manager.new_entity(EntityType::Player);
        }
        // disable random entities
        let num_entities = rand::thread_rng().gen_range(0..100);
        for _ in 0..num_entities {
            let id = rand::thread_rng().gen_range(0..entity_manager::MAX_NUM_ENTITIES);
            let vid = entity_manager::EntityVID {
                id: id,
                version: entity_manager.versions[id],
            };
            entity_manager.set_inactive(vid);
        }
    }
}
