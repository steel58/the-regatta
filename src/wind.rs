pub mod wind {
    use crate::utils::utils::UserSettings;
    use bevy::prelude::*;
    #[derive(Default, GizmoConfigGroup, Reflect)]
    pub struct WindArrows {
        arrow_space: f32,
        arrow_length: f32,
    }
    
    pub fn draw_field(
        mut wind_arrows: Gizmos<WindArrows>,
        q_cameras: Query<(&Camera, &GlobalTransform), With<Camera>>,
        user_settings: Res<UserSettings>,
    ) {
        let (camera, camera_transform) = q_cameras.single();

        if let Some(viewport_rect) = camera.logical_viewport_rect() {
            let cam_min = camera.viewport_to_world_2d(
                camera_transform,
                viewport_rect.min,
            ).unwrap();
            let cam_max = camera.viewport_to_world_2d(
                camera_transform,
                viewport_rect.max,
            ).unwrap();

            let wind_spacing_int = user_settings.wind_spacing as i32;
            let mut x_min = cam_min.x as i32;
            while x_min % wind_spacing_int != 0 {
                x_min -= 1;
            }
            let mut x_max = cam_max.x as i32;
            while x_max % wind_spacing_int != 0 {
                x_max += 1;
            }

            let mut y_min = cam_max.y as i32;
            while y_min % wind_spacing_int != 0 {
                y_min -= 1;
            }
            let mut y_max = cam_min.y as i32;
            while y_max % wind_spacing_int != 0 {
                y_max += 1;
            }

            let mut x = x_min as f32;

            while x <  x_max as f32 {
                let mut y = y_min as f32;
                while y < y_max as f32 {
                    let start = Vec2::new(x, y);
                    let end = Vec2::new(
                        x + user_settings.wind_length,
                        y + user_settings.wind_length,
                    );
                    wind_arrows.arrow_2d(start, end, Color::srgb(1.0, 1.0, 1.0));
                    y += user_settings.wind_spacing;
                }
                x += user_settings.wind_spacing;
            }
        }
    }
}
