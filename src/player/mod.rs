use crate::bullet;
use crate::consts::{PLAYER_SIZE, ALL_KEYS};
use crate::enemy::Enemy;
use crate::shared::Velocity;
use crate::shared::events::BulletShot;
use bevy::prelude::*;
use bevy::sprite::SpriteBundle;

// marker component
#[derive(Component)]
pub struct Player;

#[derive(Bundle)]
pub struct PlayerBundle {
    pub player: Player,

    #[bundle]
    pub sprite_bundle: SpriteBundle,
    // collider: Collider,
    // pub selected_word: Word
}

impl PlayerBundle {
    pub fn new(pos: Vec3, size: Option<Vec2>) -> Self {
        Self {
            player: Player,
            sprite_bundle: SpriteBundle {
                transform: Transform {
                    translation: pos,
                    ..default()
                },
                sprite: Sprite {
                    color: Color::WHITE,
                    custom_size: size,
                    ..default()
                },
                ..default()
            },
            // selected_word: Word::new(Some(word))
        }
    }
}

// player spawner
pub fn spawn(mut commands: Commands, windows: Res<Windows>) {
    let primary = windows.primary();
    commands.spawn(PlayerBundle::new(
        Vec3 {
            x: primary.width() / 2.0,
            y: PLAYER_SIZE.y * 2.0,
            z: 0.0,
        },
        Some(PLAYER_SIZE),
    ));
}

pub fn update_input(
    mut commands: Commands,
    keys: Res<Input<KeyCode>>,
    player_query: Query<&Transform, With<Player>>,
    enemy_query: Query<&Transform, With<Enemy>>,
    mut bullet_shot_writer: EventWriter<BulletShot>
) {
    for player_transform in player_query.iter() {
        for enemy_transform in enemy_query.iter() {
            let difference = enemy_transform.translation - player_transform.translation;

            if keys.any_just_pressed(ALL_KEYS) {
                bullet::spawn(
                    &mut commands,
                    &mut bullet_shot_writer,
                    player_transform.translation,
                    Velocity {
                        multiplier: 0.01,
                        vector: Vec2 { x: difference.x, y: difference.y },
                    },
                );

                bullet_shot_writer.send(BulletShot);
            }
        }
    }
}
