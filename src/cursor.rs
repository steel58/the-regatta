pub mod cursor {
    use bevy::prelude::*;
    use bevy::window::{CursorGrabMode, PrimaryWindow};
    #[derive(Component)]
    pub struct VirtualCursor {
        sensitivity: f32,
    }

    const CAMERA_SPEED: f32 = 300.0;
    const CAMERA_MOVE_BORDER: f32 = 20.0;

    pub fn spawn_cursor (
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<ColorMaterial>>,
        q_windows: Query<&Window, With<PrimaryWindow>>,
    ) {
        let triangle = Triangle2d::new(
            Vec2::new(0.0, 25.0),   // top
            Vec2::new(-18.0, -25.0), // bottom left
            Vec2::new(18.0, -25.0),  // bottom right
        );

        let window = q_windows.single();
        let height = window.resolution.height() / 2.0;
        let width = window.resolution.width() /  2.0;

        commands.spawn((
                VirtualCursor {
                    sensitivity: 0.5,
                },
                Mesh2d(meshes.add(triangle)),
                MeshMaterial2d(materials.add(Color::srgb(0.8, 0.14, 1.0))),
                Transform::from_xyz(0.0, 0.0, 0.0)
                .with_rotation(Quat::from_rotation_z(f32::to_radians(45.0))),
        ));
    }


    pub fn update_cursor (
        mut evr_motion: EventReader<bevy::input::mouse::MouseMotion>,
        mut q_cursors: Query<(&mut Transform, &VirtualCursor), With<VirtualCursor>>,
        mut q_cameras: Query<&mut Transform, (With<Camera>, Without<VirtualCursor>)>,
    ) {
        let (mut transform, cursor) = q_cursors.single_mut();

        evr_motion
            .read()
            .for_each(|event| {
                let x_displacement = event.delta.x * cursor.sensitivity;
                let y_displacement = event.delta.y * cursor.sensitivity;
                transform.translation.x = transform.translation.x + x_displacement;
                transform.translation.y = transform.translation.y - y_displacement;
            });
    }


    pub fn confine_cursor (
        mut q_cursors: Query<&mut Transform, With<VirtualCursor>>,
        mut q_cameras: Query<
            (&Camera, &mut Transform, &GlobalTransform),
            (With<Camera>, Without<VirtualCursor>)
        >,
        time: Res<Time>,
    ) {
        let (camera, mut camera_transform, global_camera_transform) = q_cameras.single_mut();
        let mut transform = q_cursors.single_mut();

        if let Some(viewport_rect) = camera.logical_viewport_rect() {
            let cam_min = camera.viewport_to_world_2d(
                global_camera_transform,
                viewport_rect.min,
            ).unwrap();

            let cam_max = camera.viewport_to_world_2d(
                global_camera_transform,
                viewport_rect.max,
            ).unwrap();

            let x_min = cam_min.x;
            let x_max = cam_max.x;

            let y_min = cam_max.y;
            let y_max = cam_min.y;

            let mut camera_direction = Vec2::ZERO;

            if transform.translation.y >= y_max - CAMERA_MOVE_BORDER {
                camera_direction.y += 1.0;
            }

            if transform.translation.y <= y_min + CAMERA_MOVE_BORDER {
                camera_direction.y -= 1.0;
            }

            if transform.translation.x <= x_min + CAMERA_MOVE_BORDER {
                camera_direction.x -= 1.0;
            }

            if transform.translation.x >= x_max - CAMERA_MOVE_BORDER {
                camera_direction.x += 1.0;
            }

            if camera_direction != Vec2::ZERO {
                camera_direction = camera_direction.normalize();
            }

            let camera_displacement_x = camera_direction.x * CAMERA_SPEED * time.delta_secs(); 
            let camera_displacement_y = camera_direction.y * CAMERA_SPEED * time.delta_secs();
            camera_transform.translation.x += camera_displacement_x;
            camera_transform.translation.y += camera_displacement_y;



            transform.translation.x = (transform.translation.x + camera_displacement_x).clamp(x_min, x_max);
            transform.translation.y = (transform.translation.y + camera_displacement_y).clamp(y_min, y_max);
        }
    }


    pub fn grab_mouse (
        mut windows: Query<&mut Window, With<PrimaryWindow>>
    ) {
        let mut window = windows.single_mut();
        window.cursor_options.grab_mode = CursorGrabMode::Locked;
        window.cursor_options.visible = false;
    }


    pub fn free_mouse (
        mut q_windows: Query<&mut Window, With<PrimaryWindow>>,
    ) {
        let mut primary_window = q_windows.single_mut();

        primary_window.cursor_options.grab_mode = CursorGrabMode::None;
        primary_window.cursor_options.visible = true;
    }

    pub fn move_camera (
        keys: Res<ButtonInput<KeyCode>>,
        time: Res<Time>,
        mut q_cameras: Query<&mut Transform, (With<Camera>, Without<VirtualCursor>)>,
        q_cursors: Query<&Transform, With<VirtualCursor>>,
    ) {
        let mut transform = q_cameras.single_mut();
        let mut direction = Vec2::ZERO;
        let cursor_transform = q_cursors.single();

        if keys.pressed(KeyCode::ArrowUp) {
            direction.y += 1.0;
        }

        if keys.pressed(KeyCode::ArrowDown) {
            direction.y -= 1.0;
        }

        if keys.pressed(KeyCode::ArrowLeft) {
            direction.x -= 1.0;
        }

        if keys.pressed(KeyCode::ArrowRight) {
            direction.x += 1.0;
        }

        if direction != Vec2::ZERO {
            direction = direction.normalize();
        }

        let camera_displacement_x = direction.x * CAMERA_SPEED * time.delta_secs(); 
        let camera_displacement_y = direction.y * CAMERA_SPEED * time.delta_secs();
        transform.translation.x += camera_displacement_x;
        transform.translation.y += camera_displacement_y;
    }
}
