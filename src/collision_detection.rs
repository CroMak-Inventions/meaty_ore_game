use bevy::{platform::collections::HashMap, prelude::*};

use crate::{
    asteroids::{Asteroid, AsteroidCollisionAnimationEvent},
    health::Health,
    movement::{Acceleration, Velocity},
    schedule::InGameSet,
    sound_fx::AsteroidCollisionSoundEvent,
    spaceship::{Spaceship, SpaceshipMissile},
};


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

#[derive(Component, Debug)]
pub struct CollisionDamage {
    pub amount: f32,
}

impl CollisionDamage {
    pub fn new(amount: f32) -> Self {
        Self { amount }
    }
}

#[derive(Event, Debug)]
pub struct CollisionEvent {
    pub entity: Entity,
    pub collided_entity: Entity,
}

impl CollisionEvent {
    pub fn new(entity: Entity, collided_entity: Entity) -> Self {
        Self {entity, collided_entity}
    }
}

pub struct CollisionDetectionPlugin;

impl Plugin for CollisionDetectionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update,
            collision_detection.in_set(InGameSet::CollisionDetection),
        )
        .add_systems(
            Update,
            (
                (
                    dispatch_collistion_events::<Asteroid>,
                    dispatch_collistion_events::<Spaceship>,
                    dispatch_collistion_events::<SpaceshipMissile>,
                ),
                handle_collision_event,
            )
            .chain()
            .in_set(InGameSet::EntityUpdates),
        )
        .add_event::<CollisionEvent>();
    }
}

fn collision_detection(
    mut query: Query<(Entity, &Transform, &mut Collider)>
) {
    let mut colliding_entities: HashMap<Entity, Vec<Entity>> = HashMap::new();

    // First, detect collisions.
    for (entity_a, transform_a, collider_a) in query.iter() {
        for (entity_b, transform_b, collider_b) in query.iter() {
            if entity_a != entity_b {
                let distance = transform_a.translation
                                    .distance(transform_b.translation);
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


fn dispatch_collistion_events<T: Component>(
    mut collision_event_writer: EventWriter<CollisionEvent>,
    query: Query<(Entity, &Collider), With<T>>
) {
    for (entity, collider) in query.iter() {
        for &collided_entity in collider.colliding_entities.iter() {
            // Entity collided with another Entity of the same type
            if query.get(collided_entity).is_ok() {
                continue;
            }

            collision_event_writer.write(CollisionEvent::new(
                entity,
                collided_entity,
            ));
        }
    }
}

pub fn handle_collision_event(
    mut collision_event_reader: EventReader<CollisionEvent>,
    mut sound_event_writer: EventWriter<AsteroidCollisionSoundEvent>,
    mut animation_event_writer: EventWriter<AsteroidCollisionAnimationEvent>,
    mut health_query: Query<&mut Health>,
    asteroid_query: Query<(&Velocity, &Acceleration)>,
    missile_query: Query<&Transform, Or<(With<Spaceship>, With<SpaceshipMissile>)>>,
    collision_damage_query: Query<&CollisionDamage>
) {
    for &CollisionEvent {
        entity,
        collided_entity
    } in collision_event_reader.read() {
        // let's figure out the changes in health
        let Ok(mut health) = health_query.get_mut(entity) else {
            continue;
        };

        let Ok(collision_damage) = collision_damage_query.get(collided_entity) else {
            continue;
        };

        health.value -= collision_damage.amount;

        // now we send out a sound
        sound_event_writer.write(AsteroidCollisionSoundEvent);

        // now we send out a collistion animation.  We only do this for missile
        // collisions, which is why we query for the missile.
        let Ok(xform) = missile_query.get(entity) else {
            continue;
        };

        let Ok((
            velocity,
            acceleration,
        )) = asteroid_query.get(collided_entity) else {
            continue;
        };

        animation_event_writer.write(
            AsteroidCollisionAnimationEvent::new(
                xform,
                velocity,
                acceleration,
            )
        );
    }
}
