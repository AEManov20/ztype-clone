use bevy::{
    prelude::*,
    winit::WinitSettings,
};

mod bullet;
mod shared;

mod consts;
mod util;

mod enemy;
mod player;

use consts::WINDOW_RES;

fn setup_camera(windows: Res<Windows>, mut commands: Commands) {
    // this calls unwrap/expect
    let primary = windows.primary();

    let cam_bundle_default = Camera2dBundle::default();
    println!("{}", primary.width());

    let bundle = Camera2dBundle {
        transform: Transform {
            translation: Vec3 {
                x: primary.width() / 2.0,
                y: primary.height() / 2.0,
                ..cam_bundle_default.transform.translation
            },
            ..cam_bundle_default.transform
        },
        ..cam_bundle_default
    };

    commands.spawn(bundle);
}

fn print_mouse_pos_in_world(windows: Res<Windows>, q_camera: Query<(&Camera, &GlobalTransform)>) {
    let windows = windows.as_ref();
    if let Some(mouse_pos) = windows.primary().cursor_position() {
        println!(
            "{:#?}",
            util::window_to_world(windows, q_camera.single(), mouse_pos)
        );
    }
}

// this system is just a sanity check in case
// the primary window DOES get resized
// even in the case of the WM disallowing it
fn update_camera(windows: Res<Windows>, mut query: Query<&mut Transform, With<Camera>>) {
    if windows.is_changed() {
        let mut camera_transform = query.single_mut();
        let primary = windows.primary();
        camera_transform.translation.x = primary.width() / 2.0;
        camera_transform.translation.y = primary.height() / 2.0;
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                width: WINDOW_RES.x,
                height: WINDOW_RES.y,
                resizable: false,
                ..default()
            },
            ..default()
        }))

        .add_event::<shared::events::BulletShot>()
        .add_event::<shared::events::EnemyCollision>()
        .add_event::<shared::events::EnemyDestroyed>()
        .add_event::<shared::events::PlayerDestroyed>()

        .add_startup_system(setup_camera)
        .add_startup_system(player::spawn.after(setup_camera))
        .add_startup_system(enemy::spawn
            .after(setup_camera)
            .after(player::spawn))
                    
        .add_system(update_camera)
        .add_system(player::update_input)
        .add_system(bullet::clean_up)
        .add_system(enemy::update_velocities)
        .add_system(enemy::update_text_position)
        .add_system(shared::update_velocity_transforms)
        .add_system(shared::check_collisions)
        
        .run();
}
