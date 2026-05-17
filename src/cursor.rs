pub mod cursor {
    use bevy::prelude::*;
    use bevy::window::{CursorGrabMode, PrimaryWindow};
    #[derive(Component)]
    pub struct VirtualCursor {
        sensitivity: f32,
        x_max: f32,
        y_max: f32,
        x_min: f32,
        y_min: f32,
    }

    const CAMERA_SPEED: f32 = 300.0;

    pub fn spawn_cursor(
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
                    x_max: width,
                    y_max: height,
                    x_min: -width,
                    y_min: -height,
                },
                Mesh2d(meshes.add(triangle)),
                MeshMaterial2d(materials.add(Color::srgb(0.8, 0.14, 1.0))),
                Transform::from_xyz(0.0, 0.0, 0.0)
                .with_rotation(Quat::from_rotation_z(f32::to_radians(45.0))),
        ));
    }


    pub fn update_cursor(
        mut evr_motion: EventReader<bevy::input::mouse::MouseMotion>,
        mut q_cursors: Query<(&mut Transform, &VirtualCursor), With<VirtualCursor>>,
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
        mut q_cursors: Query<(&mut Transform, &VirtualCursor), With<VirtualCursor>>,
        q_cameras: Query<(&Camera, &GlobalTransform), With<Camera>>,
    ) {
        let (camera, camera_transform) = q_cameras.single();
        let (mut transform, _cursor) = q_cursors.single_mut();

        if let Some(viewport_rect) = camera.logical_viewport_rect() {
            let min = camera.viewport_to_world_2d(camera_transform, viewport_rect.min).unwrap();
            let max = camera.viewport_to_world_2d(camera_transform, viewport_rect.max).unwrap();

            transform.translation.x = transform.translation.x.clamp(min.x, max.x);
            transform.translation.y = transform.translation.y.clamp(max.y, min.y);
        }
    }


    pub fn grab_mouse(
        mut windows: Query<&mut Window, With<PrimaryWindow>>
    ) {
        let mut window = windows.single_mut();
        window.cursor_options.grab_mode = CursorGrabMode::Locked;
        window.cursor_options.visible = false;
    }


    pub fn free_mouse(
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
        mut q_cursors: Query<(&mut Transform, &mut VirtualCursor), With<VirtualCursor>>,
    ) {
        let mut transform = q_cameras.single_mut();
        let mut direction = Vec2::ZERO;
        let (mut cursor_transform, mut cursor) = q_cursors.single_mut();

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
        cursor.x_max += camera_displacement_x;
        cursor.y_max += camera_displacement_y;
        cursor.x_min += camera_displacement_x;
        cursor.y_min += camera_displacement_y;
        transform.translation.x += camera_displacement_x;
        transform.translation.y += camera_displacement_y;
        // cursor_transform.translation.x = cursor_transform.translation.x
        //     .clamp(cursor.x_min, cursor.x_max);
        // cursor_transform.translation.y = cursor_transform.translation.y
        //     .clamp(cursor.y_min, cursor.y_max);
    }

    pub fn debug_cursor(
        q_cursors: Query<(&Transform, &VirtualCursor)>,
        q_windows: Query<&Window, With<PrimaryWindow>>,
    ) {
        let (transform, cursor) = q_cursors.single();
        let window = q_windows.single();
        println!(
            "pos: ({}, {}) | bounds x: ({}, {}) y: ({}, {})",
            transform.translation.x,
            transform.translation.y,
            cursor.x_min, cursor.x_max,
            cursor.y_min, cursor.y_max,
        );
        println!(
            "window: (height: {}, width: {})",
            window.height(),
            window.width(),
        );
        println!(
            "window: (height: {}, width: {})",
            window.resolution.height(),
            window.resolution.width(),
        );
    }
}
