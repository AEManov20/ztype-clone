use std::ops::Range;

use bevy::{prelude::*, sprite::collide_aabb::collide};
use rand::Rng;

use crate::{bullet::Bullet, player::Player, enemy::Enemy};

use self::events::{PlayerDestroyed, EnemyCollision, EnemyDestroyed, BulletShot};

// storage component
#[derive(Component)]
pub struct Word(Option<String>);

impl Word {
    pub fn new(word: Option<String>) -> Self {
        Word(word)
    }

    pub fn get_word(&self) -> Option<String> {
        self.0.clone()
    }
}

#[derive(Component)]
pub struct Velocity {
    pub vector: Vec2,
    pub multiplier: f32,
}

// a system to update all entities that have Velocity and Transform
pub fn update_velocity_transforms(mut query: Query<(&Velocity, &mut Transform)>) {
    for (vel, mut transform) in query.iter_mut() {
        transform.translation.x += vel.vector.x * vel.multiplier;
        transform.translation.y += vel.vector.y * vel.multiplier;
    }
}

pub fn random_vec2(r: Range<f32>) -> Vec2 {
    Vec2 {
        x: rand::thread_rng().gen_range(r.clone()),
        y: rand::thread_rng().gen_range(r)
    }
}

pub fn check_collisions(
    mut commands: Commands,
    bullet_query: Query<(Entity, &Sprite, &Transform), With<Bullet>>,
    player_query: Query<(Entity, &Sprite, &Transform), With<Player>>,
    enemy_query: Query<(Entity, &Sprite, &Transform), With<Enemy>>,
    mut player_destroyed_events: EventWriter<PlayerDestroyed>,
    mut enemy_collision_events: EventWriter<EnemyCollision>,
    mut enemy_destroyed_events: EventWriter<EnemyDestroyed>
) {
    // check if enemy(ies) has collided with player(s)
    for (enemy_entity, enemy_sprite, enemy_transform) in enemy_query.iter() {
        for (player_entity, player_sprite, player_transform) in player_query.iter() {
            // TODO: this isn't gonna be using custom_size in the future, since
            // TODO: that's a thing for non-textured sprites
            if let Some(_) = collide(
                enemy_transform.translation,
                enemy_sprite.custom_size.unwrap(),
                player_transform.translation,
                player_sprite.custom_size.unwrap()
            ) {
                player_destroyed_events.send(PlayerDestroyed(
                    player_entity,
                    player_transform.translation
                ));
                enemy_destroyed_events.send(EnemyDestroyed(
                    enemy_entity,
                    player_transform.translation
                ));
            }
        }
    }

    // check if bullet(s) has collided with enemy(ies)
    for (bullet_entity, bullet_sprite, bullet_transform) in bullet_query.iter() {
        for (enemy_entity, enemy_sprite, enemy_transform) in enemy_query.iter() {
            if let Some(_) = collide(
                enemy_transform.translation,
                enemy_sprite.custom_size.unwrap(),
                bullet_transform.translation,
                bullet_sprite.custom_size.unwrap()
            ) {
                commands.entity(bullet_entity).despawn_recursive();
                enemy_collision_events.send(EnemyCollision(
                    enemy_entity,
                    enemy_transform.translation
                ));
            }
        }
    }
}

pub mod events {
    use bevy::prelude::*;

    // indicating that something has collided with a player
    // where player collision happened
    pub struct PlayerDestroyed(pub Entity, pub Vec3); // destroyed means, it destroys the entity too

    // indicating that something has collided with an enemy
    // where enemy collision happened
    // (most likely occurs when a bullet collidees with an enemy)
    pub struct EnemyCollision(pub Entity, pub Vec3);

    // indicating that an enemy has been destroyed
    // (most likely occurs when a bullet collidees with an enemy)
    // (or when a player collides with an enemy)
    pub struct EnemyDestroyed(pub Entity, pub Vec3); // destroyed means, it destroys the entity too

    // indicating that a bullet has been shot (created)
    pub struct BulletShot;
}