pub mod wind {
    use crate::utils::utils::UserSettings;
    use bevy::prelude::*;
    #[derive(Default, GizmoConfigGroup, Reflect)]
    pub struct WindArrows {
        arrow_space: f32,
        arrow_length: f32,
    }

    #[derive(Component)]
    pub struct WindTrace {
        start_x: f32,
        start_y: f32,
        current_x: f32,
        current_y: f32,
    }

    #[derive(Resource)]
    pub struct WindSettings {
        pub xx_factor: f32,
        pub xy_factor: f32,
        pub xx_function: fn(f32) -> f32,
        pub xy_function: fn(f32) -> f32,
        pub x_constant: f32,
        pub yx_factor: f32,
        pub yy_factor: f32,
        pub yx_function: fn(f32) -> f32,
        pub yy_function: fn(f32) -> f32,
        pub y_constant: f32,
    }

    impl Default for WindSettings {
        fn default() -> Self {
            Self {
                xx_factor: 3.0,
                xy_factor: 1.0,
                yy_factor: 5.0,
                yx_factor: 2.0,
                x_constant: 6.3,
                y_constant: 8.6,
                xx_function: f32::sin,
                xy_function: f32::cos,
                yx_function: f32::sin,
                yy_function: f32::cos,
            }
        }
    }

    pub fn draw_wind_field(
        mut wind_arrows: Gizmos<WindArrows>,
        q_cameras: Query<(&Camera, &GlobalTransform), With<Camera>>,
        user_settings: Res<UserSettings>,
        wind_settings: Res<WindSettings>,
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
                    let wind_vector = calculate_wind(x, y, &wind_settings);
                    let end = Vec2::new(
                        x + wind_vector.x,
                        y + wind_vector.y,
                    );
                    wind_arrows.arrow_2d(start, end, Color::srgb(1.0, 1.0, 1.0));
                    y += user_settings.wind_spacing;
                }
                x += user_settings.wind_spacing;
            }
        }
    }

    pub fn calculate_wind(
        x: f32,
        y: f32,
        wind_settings: &Res<WindSettings>,
    ) -> Vec2 {
        let wind_x =
            wind_settings.xx_factor * (wind_settings.xx_function)(x) 
            + wind_settings.xy_factor * (wind_settings.xy_function)(y) 
            + wind_settings.x_constant;
        let wind_y =
            wind_settings.yx_factor * (wind_settings.yx_function)(x) 
            + wind_settings.yy_factor * (wind_settings.yy_function)(y) 
            + wind_settings.y_constant;

        Vec2::new(wind_x, wind_y)
    }
}
