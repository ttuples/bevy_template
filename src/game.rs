#![allow(unused)]
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

pub(crate) fn plugin(app: &mut App) {
    app.init_resource::<GameData>();
}

#[derive(Resource, Clone, PartialEq, Debug, Default)]
struct GameData {}

