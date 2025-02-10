use bevy::{input::mouse::AccumulatedMouseScroll, prelude::*, render::camera::ScalingMode};
use std::ops::Range;

#[derive(Debug, Resource)]
struct CameraSettings {
    /// The height of the viewport in world units when the orthographic camera's scale is 1
    pub orthographic_viewport_height: f32,
    /// Clamp the orthographic camera's scale to this range
    pub orthographic_zoom_range: Range<f32>,
    /// Multiply mouse wheel inputs by this factor when using the orthographic camera
    pub orthographic_zoom_speed: f32,
    /// The speed at which the camera moves when a button is pressed
    pub camera_move_speed: f32,
}

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera);
        app.insert_resource(CameraSettings {
            orthographic_viewport_height: 10.0,
            orthographic_zoom_range: 40.0..200.0,
            orthographic_zoom_speed: 0.01,
            camera_move_speed: 200.0,
        });
        app.add_systems(Update, zoom);
        app.add_systems(Update, move_camera);
    }
}

fn spawn_camera(mut commands: Commands, camera_settings: Res<CameraSettings>) {
    commands.spawn((
        Name::new("Camera"),
        Camera2d,
        Projection::from(OrthographicProjection {
            scaling_mode: ScalingMode::FixedVertical {
                viewport_height: camera_settings.orthographic_viewport_height,
            },
            scale: 100.0, // Start with a more distant scale (higher value for distance)
            ..OrthographicProjection::default_2d()
        }),
    ));
}

fn zoom(
    camera: Single<&mut Projection, With<Camera>>,
    camera_settings: Res<CameraSettings>,
    mouse_wheel_input: Res<AccumulatedMouseScroll>,
) {
    if let Projection::Orthographic(ref mut orthographic) = *camera.into_inner() {
        let delta_zoom = -mouse_wheel_input.delta.y * camera_settings.orthographic_zoom_speed;
        let multiplicative_zoom = 1. + delta_zoom;

        orthographic.scale = (orthographic.scale * multiplicative_zoom).clamp(
            camera_settings.orthographic_zoom_range.start,
            camera_settings.orthographic_zoom_range.end,
        );
    }
}

fn move_camera(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<Camera>>,
    camera_settings: Res<CameraSettings>,
    time: Res<Time>,
) {
    if let Ok(mut transform) = query.get_single_mut() {
        let mut direction = Vec3::ZERO;
        if keyboard_input.pressed(KeyCode::KeyW) || keyboard_input.pressed(KeyCode::ArrowUp) {
            direction.y -= 100.0;
        }
        if keyboard_input.pressed(KeyCode::KeyS) || keyboard_input.pressed(KeyCode::ArrowDown) {
            direction.y += 100.0;
        }
        if keyboard_input.pressed(KeyCode::KeyA) || keyboard_input.pressed(KeyCode::ArrowLeft) {
            direction.x += 100.0;
        }
        if keyboard_input.pressed(KeyCode::KeyD) || keyboard_input.pressed(KeyCode::ArrowRight) {
            direction.x -= 100.0;
        }
        if direction != Vec3::ZERO {
            direction = direction.normalize();
        }
        transform.translation += direction * camera_settings.camera_move_speed * time.delta_secs();
    }
}
