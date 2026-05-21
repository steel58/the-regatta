pub mod boat {
    use bevy::prelude::*;
    #[derive(Component)]
    pub struct Boat;
    #[derive(Component)]
    pub struct Sail;
    
    const BOAT_SPEED: f32 = 300.0;
    const SAIL_SPEED: f32 = 0.5;
    // Distance to pivot from sail centre
    const SAIL_PIVOT: f32 = 20.0;


    pub fn setup_player(
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<ColorMaterial>>,
    ) {
        commands.spawn((
                Boat,
                Sprite {
                    color: Color::srgb(0.38, 0.26, 0.15),
                    custom_size: Some(Vec2::new(50.0, 80.9017)),
                    ..default()
                },
                Transform::from_xyz(0.0, 0.0, 0.0),
        ));

        let mast = Circle::new(3.0);
        commands.spawn((
                Boat,
                Mesh2d(meshes.add(mast)),
                MeshMaterial2d(materials.add(Color::srgb(0.75, 0.75, 0.75))),
                Transform::from_xyz(0.0, SAIL_PIVOT, 2.1),
        ));

        commands.spawn((
                Boat,
                Sail,
                Sprite {
                    color: Color::srgb(1.0, 1.0, 1.0),
                    custom_size: Some(Vec2::new(8.0, 60.0)),
                    ..default()
                },
                Transform::from_xyz(0.0, 0.0, 1.0),
        ));
    }


    pub fn move_boat(
        keys: Res<ButtonInput<KeyCode>>,
        time: Res<Time>,
        mut query: Query<&mut Transform, With<Boat>>,
    ) {
        query.iter_mut()
            .for_each(|mut transform| {
                move_boat_section(&keys, &time, &mut transform.translation);
            });
    }

    fn move_boat_section(
        keys: &Res<ButtonInput<KeyCode>>,
        time: &Res<Time>,
        transform: &mut Vec3,
    ) {
        let mut direction = Vec2::ZERO;

        if keys.pressed(KeyCode::KeyW) {
            direction.y += 1.0;
        }

        if keys.pressed(KeyCode::KeyS) {
            direction.y -= 1.0;
        }

        if keys.pressed(KeyCode::KeyA) {
            direction.x -= 1.0;
        }

        if keys.pressed(KeyCode::KeyD) {
            direction.x += 1.0;
        }

        // Normalize so diagonal movement isn't faster
        if direction != Vec2::ZERO {
            direction = direction.normalize();
        }

        transform.x += direction.x * BOAT_SPEED * time.delta_secs();
        transform.y += direction.y * BOAT_SPEED * time.delta_secs();
    }


    pub fn rotate_sail(
        keys: Res<ButtonInput<KeyCode>>,
        time: Res<Time>,
        mut query: Query<&mut Transform, With<Sail>>,
    ) {
        let mut transform = query.single_mut();
        let mut direction = 0.0;
        if keys.pressed(KeyCode::KeyJ) {
            direction += 1.0;
        }

        if keys.pressed(KeyCode::KeyK) {
            direction -= 1.0;
        }
        
        let rotation = SAIL_SPEED * direction * time.delta_secs();
        let translation = reposition_sail(
            transform.rotation, 
            rotation,
        );
        transform.rotate_z(rotation);
        transform.translation.x += translation.x;
        transform.translation.y += translation.y;
    }

    fn reposition_sail(
        prev_quat: Quat,
        rotation: f32,
    ) -> Vec2 {
        let prev_angle = prev_quat.to_euler(EulerRot::XYZ).2;
        // 0 Radians is pointing to positive y, so cos and sin are flipped
        let pivot_start_x = SAIL_PIVOT * f32::sin(prev_angle);
        let pivot_start_y = SAIL_PIVOT * f32::cos(prev_angle);
        let pivot_end_x = SAIL_PIVOT * f32::sin(prev_angle + rotation);
        let pivot_end_y = SAIL_PIVOT * f32::cos(prev_angle + rotation);
        let displacement_x = pivot_end_x - pivot_start_x;
        let displacement_y = pivot_end_y - pivot_start_y;
        Vec2::new(displacement_x, -displacement_y)
    }
}
