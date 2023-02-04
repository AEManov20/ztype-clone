use bevy::prelude::*;

use crate::{
    consts::{BULLET_SIZE, BULLET_SPEED_MULTIPLIER},
    shared::{Velocity, events::BulletShot},
};

// bullet representing velocity
#[derive(Component)]
pub struct Bullet;

#[derive(Bundle)]
pub struct BulletBundle {
    pub bullet: Bullet,
    pub velocity: Velocity,

    #[bundle]
    pub sprite_bundle: SpriteBundle,
}

// not a system
pub fn spawn(commands: &mut Commands, events: &mut EventWriter<BulletShot>, player_pos: Vec3, velocity: Velocity) {
    events.send(BulletShot);

    commands.spawn(BulletBundle {
        bullet: Bullet,
        velocity,
        sprite_bundle: SpriteBundle {
            sprite: Sprite {
                color: Color::YELLOW,
                custom_size: Some(BULLET_SIZE),
                ..Default::default()
            },
            transform: Transform {
                translation: player_pos,
                ..Default::default()
            },
            ..Default::default()
        },
    });
}

pub fn clean_up(mut commands: Commands, query: Query<(Entity, &Transform), With<Bullet>>, windows: Res<Windows>) {
    let primary = windows.primary();

    for (entity, transform) in query.iter() {
        if transform.translation.x > primary.width() || transform.translation.y > primary.height() {
            commands.entity(entity).despawn_recursive();
        }
    }
}