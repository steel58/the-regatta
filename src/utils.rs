pub mod utils {
    use bevy::prelude::*;

    pub fn spawn_camera(
        mut commands: Commands
    ) {
        commands.spawn(Camera2d);
    }
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Default, States)]
    pub enum GameState {
        #[default]
        InGame,
        Paused,
    }

    impl GameState {
        fn toggle_pause(&self) -> Self {

            match *self {
                GameState::InGame => GameState::Paused,
                GameState::Paused => GameState::InGame,
                _ => *self,
            }
        }
    }

    pub fn pause_game(
        mut next_state: ResMut<NextState<GameState>>,
        current_state: Res<State<GameState>>,
        input: Res<ButtonInput<KeyCode>>,
    ) {
        if input.just_pressed(KeyCode::Escape) {
            next_state.set(current_state.toggle_pause());
        }
    }

    pub fn start_state(mut next_state: ResMut<NextState<GameState>>) {
        next_state.set(GameState::InGame);
    }

    #[derive(Resource)]
    pub struct UserSettings {
        pub wind_spacing: f32,
        pub wind_length: f32,
    }

    impl Default for UserSettings {
        fn default() -> Self {
            UserSettings {
                wind_spacing: 80.0,
                wind_length: 20.0,
            }
        }
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
}
