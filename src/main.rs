#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::prelude::*;

mod assets;
mod game;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "bevy game".to_string(),
                    ..default()
                }),
                ..default()
            }),
            assets::plugin,
        ))
        .run();
}
