pub mod boat {
    use bevy::prelude::*;
    #[derive(Component)]
    pub struct Boat;

    pub fn setup_player(mut commands: Commands) {
        commands.spawn((
                Boat,
                Sprite {
                    color: Color::srgb(0.25, 0.6, 1.0),
                    custom_size: Some(Vec2::new(50.0, 50.0)),
                    ..default()
                },
                Transform::from_xyz(0.0, 0.0, 0.0),
        ));
    }

    const PLAYER_SPEED: f32 = 300.0;

    pub fn move_player(
        keys: Res<ButtonInput<KeyCode>>,
        time: Res<Time>,
        mut query: Query<&mut Transform, With<Boat>>,
    ) {
        let mut transform = query.single_mut();
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

        transform.translation.x += direction.x * PLAYER_SPEED * time.delta_secs();
        transform.translation.y += direction.y * PLAYER_SPEED * time.delta_secs();
    }
}
