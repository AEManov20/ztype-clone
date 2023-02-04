use bevy::{prelude::*, render::camera::RenderTarget};

pub(crate) fn window_to_world(wnds: &Windows, q_camera: (&Camera, &GlobalTransform), desired_pos: Vec2) -> Vec2 {
    // get the camera info and transform
    // assuming there is exactly one main camera entity, so query::single() is OK
    let (camera, camera_transform) = q_camera;
    
    // get the window that the camera is displaying to (or the primary window)
    let wnd = if let RenderTarget::Window(id) = camera.target {
        wnds.get(id).unwrap()
    } else {
        wnds.get_primary().unwrap()
    };

    // get the size of the window
    let window_size = Vec2::new(wnd.width() as f32, wnd.height() as f32);

    // convert screen position [0..resolution] to ndc [-1..1] (gpu coordinates)
    let ndc = (desired_pos / window_size) * 2.0 - Vec2::ONE;

    // matrix for undoing the projection and camera transform
    let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix().inverse();

    // use it to convert ndc to world-space coordinates
    let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));

    // reduce it to a 2D value
    let world_pos: Vec2 = world_pos.truncate();

    world_pos
}