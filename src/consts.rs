use bevy::prelude::KeyCode::*;
use bevy::prelude::*;
use std::ops::Range;

pub(crate) const WINDOW_RES: Vec2 = Vec2 { x: 480.0, y: 720.0 };

pub(crate) const ENEMY_SIZE: Vec2 = Vec2 { x: 20.0, y: 20.0 };

pub(crate) const ENEMY_SPAWN_SPREAD_RADIUS: f32 = 300.0;
pub(crate) const ENEMY_SPAWN_DEGREE_RANGE: Range<i32> = -45..45;
pub(crate) const ENEMY_SPAWN_RANDOM_DISPLACEMENT: Range<f32> = -50f32..50f32;
pub(crate) const ENEMY_SPAWN_PER_DEGREE: i32 = 20;
pub(crate) const ENEMY_VELOCITY_VECTOR_DISPLACEMENT: Range<f32> = -30f32..30f32;

pub(crate) const ENEMY_SPEED_MULTIPLIER: f32 = 1.0;
pub(crate) const ENEMY_TEXT_FONT_SIZE: f32 = 25.0;
pub(crate) const ENEMY_TEXT_PADDING: UiRect = UiRect {
    left: Val::Px(5.0), right: Val::Px(5.0),
    top: Val::Px(2.5),   bottom: Val::Px(2.5)
};

pub(crate) const PLAYER_SIZE: Vec2 = Vec2 { x: 20.0, y: 20.0 };

pub(crate) const BULLET_SIZE: Vec2 = Vec2 { x: 5.0, y: 5.0 };
pub(crate) const BULLET_SPEED_MULTIPLIER: f32 = 5.0;

pub(crate) const FONT_PATH: &str = "fonts/Montserrat-Regular.ttf";

pub(crate) const ALL_KEYS: [bevy::prelude::KeyCode; 26] = [
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z,
];
