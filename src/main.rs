mod boat;
mod cursor;
mod wind;
mod utils;
use crate::cursor::cursor::{
    confine_cursor,
    spawn_cursor,
    update_cursor,
    grab_mouse,
    move_camera,
    free_mouse,
};
use crate::boat::boat::{
    move_boat,
    setup_player,
    rotate_sail,
};
use crate::wind::wind::{
    WindSettings,
    WindArrows,
    draw_wind_field,
};
use crate::utils::utils::{
    spawn_camera,
    start_state,
    GameState,
    UserSettings,
    pause_game,
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
        .insert_resource(ClearColor(Color::srgb(0.0, 0.39, 0.61)))
        .init_state::<GameState>()
        .init_resource::<UserSettings>()
        .init_resource::<WindSettings>()
        .init_gizmo_group::<WindArrows>()
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
            move_boat
                .run_if(in_state(GameState::InGame)),
            rotate_sail
                .run_if(in_state(GameState::InGame)),
            update_cursor
                .run_if(in_state(GameState::InGame)),
            move_camera
                .run_if(in_state(GameState::InGame)),
           draw_wind_field 
                .run_if(in_state(GameState::InGame)),
            confine_cursor,
            pause_game,
        ))
        .run();
}
