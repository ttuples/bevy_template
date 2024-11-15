#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(bevy_template::AppPlugin)
        .run();
}
