use crate::{
    consts::{
        ENEMY_SIZE, ENEMY_SPAWN_DEGREE_RANGE, ENEMY_SPAWN_PER_DEGREE,
        ENEMY_SPAWN_RANDOM_DISPLACEMENT, ENEMY_SPAWN_SPREAD_RADIUS, ENEMY_TEXT_FONT_SIZE,
        FONT_PATH, ENEMY_VELOCITY_VECTOR_DISPLACEMENT, ENEMY_TEXT_PADDING,
    },
    shared::{Word, Velocity, random_vec2}, player::Player,
};
use bevy::prelude::*;
use rand::{thread_rng, Rng};

// marker component
#[derive(Component)]
pub struct Enemy {
    pub ui_text_entity: Entity,
}

#[derive(Component)]
pub struct EnemyText;

#[derive(Bundle)]
pub struct EnemyBundle {
    enemy: Enemy,
    word: crate::shared::Word,

    #[bundle]
    sprite_bundle: SpriteBundle,
}

impl EnemyBundle {
    pub fn new(word: &str, pos: Vec3, size: Option<Vec2>, ui_text_entity: Entity) -> Self {
        let word_str = word.to_owned();

        Self {
            enemy: Enemy { ui_text_entity },
            sprite_bundle: SpriteBundle {
                transform: Transform {
                    translation: pos,
                    ..default()
                },
                sprite: Sprite {
                    color: Color::RED,
                    custom_size: size,
                    ..default()
                },
                ..default()
            },
            word: Word::new(Some(word_str.clone())),
        }
    }
}

pub fn update_velocities(
    mut enemy_query: Query<(&mut Velocity, &Transform), With<Enemy>>,
    player_query: Query<(&Transform, Changed<Transform>), With<Player>>
) {
    for (player_transform, changed) in player_query.iter() {
        if !changed { return; }
        for (mut velocity, enemy_transform) in enemy_query.iter_mut() {
            let displacement = random_vec2(ENEMY_VELOCITY_VECTOR_DISPLACEMENT);

            velocity.vector = Vec2 {
                x: player_transform.translation.x - enemy_transform.translation.x,
                y: player_transform.translation.y - enemy_transform.translation.y,
            } + displacement;

            velocity.multiplier = 0.0005;
        }
    }
}

pub fn update_text_position(
    windows: Res<Windows>,
    mut text_style_query: Query<&mut Style, With<EnemyText>>,
    changed_query: Query<(&Enemy, &Transform, Changed<Transform>)>
) {
    let primary = windows.primary();
    for (enemy, transform, is_changed) in changed_query.iter() {
        if !is_changed { continue; }

        if let Ok(mut text_style) = text_style_query.get_mut(enemy.ui_text_entity) {
            text_style.position = UiRect {
                left: Val::Px(transform.translation.x.clamp(0.0, primary.width() - ENEMY_TEXT_FONT_SIZE)),
                bottom: Val::Px(transform.translation.y.clamp(0.0, primary.height() - ENEMY_TEXT_FONT_SIZE)),
                ..default()
            };
        }
    }
}

pub fn update_text_color() {
    todo!();
}

// TODO: refactor this into a generic function
pub fn clean_up(mut commands: Commands, query: Query<(Entity, &Transform), With<Enemy>>, windows: Res<Windows>) {
    let primary = windows.primary();

    for (entity, transform) in query.iter() {
        if transform.translation.x > primary.width() || transform.translation.y > primary.height() {
            commands.entity(entity).despawn_recursive();
        }
    }
}

// enemy spawner
pub fn spawn(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    windows: Res<Windows>
) {
    let primary_win = windows.primary();
    let to_rads = |degrees: f32| degrees * std::f32::consts::PI / 180.0;
    let word = "nice";

    for deg in ENEMY_SPAWN_DEGREE_RANGE {
        if deg % ENEMY_SPAWN_PER_DEGREE != 0 {
            continue;
        }

        // displacement vector with which the enemies get displaced by a little bit (depending on the constant)
        let displacement = random_vec2(ENEMY_SPAWN_RANDOM_DISPLACEMENT);

        // position of the to be created enemy
        // used both for the Style of the text
        // and the Sprite's transform
        let pos = Vec2 {
            x: to_rads(deg as f32).sin() * ENEMY_SPAWN_SPREAD_RADIUS
                + primary_win.width() / 2.0
                + displacement.x,
            y: to_rads(deg as f32).cos() * ENEMY_SPAWN_SPREAD_RADIUS
                + primary_win.height() - ENEMY_SPAWN_SPREAD_RADIUS / 2.0
                + displacement.y,
        };

        // ui entity
        let ui_entity = commands
            .spawn(NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    position: UiRect {
                        left: Val::Px(pos.x),
                        bottom: Val::Px(pos.y),
                        ..default()
                    },
                    padding: ENEMY_TEXT_PADDING,
                    ..default()
                },
                background_color: BackgroundColor(Color::rgba(0.0, 0.0, 0.0, 0.65)),
                ..default()
            })
            .with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    word,
                    TextStyle {
                        font: asset_server.load(FONT_PATH),
                        font_size: ENEMY_TEXT_FONT_SIZE,
                        color: Color::WHITE,
                    },
                ));
            })
            .id();
        commands.entity(ui_entity).insert(EnemyText);

        // sprite entity
        let sprite_entity = commands
            .spawn(EnemyBundle::new(
                word,
                Vec3 {
                    x: pos.x,
                    y: pos.y,
                    z: 0.0,
                },
                Some(ENEMY_SIZE),
                ui_entity,
            ))
            .id();
        commands
            .entity(sprite_entity)
            .insert(Velocity {
                multiplier: 0.,
                vector: Vec2 {
                    x: 1.,
                    y: 1.
                }
            });
    }
}
