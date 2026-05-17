mod boat;
mod cursor;
use crate::cursor::cursor::{
    confine_cursor,
    spawn_cursor,
    update_cursor,
    grab_mouse,
    move_camera,
    free_mouse,
};
use crate::boat::boat::{
    move_player,
    setup_player,
};
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "The Regatta".into(),
                mode: bevy::window::WindowMode::Fullscreen(MonitorSelection::Primary),
                ..default()
            }),
            ..default()
        }))
    .init_state::<GameState>()
        .add_systems(Startup, (
            setup_player,
            spawn_camera,
            spawn_cursor,
            start_state,
            grab_mouse,
        ))
        .add_systems(OnEnter(GameState::Paused), free_mouse)
        .add_systems(OnEnter(GameState::InGame), grab_mouse)
        .add_systems(Update, (
            move_player
                .run_if(in_state(GameState::InGame)),
            update_cursor
                .run_if(in_state(GameState::InGame)),
            move_camera
                .run_if(in_state(GameState::InGame)),
            confine_cursor,
            pause_game,
        ))
        .run();
}

fn spawn_camera (
    mut commands: Commands
) {
    commands.spawn(Camera2d);
}


#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Default, States)]
enum GameState {
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

fn pause_game(
    mut next_state: ResMut<NextState<GameState>>,
    current_state: Res<State<GameState>>,
    input: Res<ButtonInput<KeyCode>>,
) {
    if input.just_pressed(KeyCode::Escape) {
        next_state.set(current_state.toggle_pause());
    }
}

fn start_state(mut next_state: ResMut<NextState<GameState>>) {
    next_state.set(GameState::InGame);
}
