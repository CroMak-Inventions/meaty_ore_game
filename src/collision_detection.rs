use bevy::{platform::collections::HashMap, prelude::*};


#[derive(Component, Debug)]
pub struct Collider {
    pub radius: f32,
    pub colliding_entities: Vec<Entity>,
}

impl Collider {
    pub fn new(radius: f32) -> Self {
        Self {
            radius,
            colliding_entities: vec![],
        }
    }
}

pub struct CollisionDetectionPlugin;

impl Plugin for CollisionDetectionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, collision_detection);
    }
}

fn collision_detection(
    mut query: Query<(Entity, &GlobalTransform, &mut Collider)>
) {
    let mut colliding_entities: HashMap<Entity, Vec<Entity>> = HashMap::new();

    // First, detect collisions.
    for (entity_a, transform_a, collider_a) in query.iter() {
        // todo: we can design this a bit more efficiently by employing a
        //       "shellsort" inspired loop.  the .remaining() function will
        //       let us do that.
        //let query_iter = query.iter().remaining()

        for (entity_b, transform_b, collider_b) in query.iter() {
            if entity_a != entity_b {
                let distance = transform_a
                                    .translation()
                                    .distance(transform_b.translation());
                if distance < collider_a.radius + collider_b.radius {
                    colliding_entities
                        .entry(entity_a)
                        .or_insert_with(Vec::new)
                        .push(entity_b);
                }
            }
        }

    }

    // Second, Update Colliders
    for (entity, _, mut collider) in query.iter_mut() {
        collider.colliding_entities.clear();

        if let Some(collisions) = colliding_entities.get(&entity) {
            collider
                .colliding_entities
                .extend(collisions.iter().copied());
        }
    }
}
