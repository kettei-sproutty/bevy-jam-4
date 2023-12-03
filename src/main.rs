mod menu;
mod settings;
mod state;
mod utils;

use bevy::prelude::*;
use settings::*;
use state::*;

fn main() {
  App::new()
    .add_plugins(DefaultPlugins)
    .insert_resource(Difficulty::Medium)
    .add_state::<GameState>()
    .add_systems(Startup, setup)
    .add_plugins(menu::MenuPlugin)
    .run();
}

fn setup(mut commands: Commands) {
  commands.spawn(Camera2dBundle::default());
}
