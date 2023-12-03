mod game;
mod menu;
mod ui;

mod settings;
mod state;
mod utils;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use settings::*;
use state::*;

fn main() {
  App::new()
    .add_plugins(DefaultPlugins)
    .insert_resource(Difficulty::Medium)
    .add_state::<GameState>()
    .add_systems(Startup, setup)
    .add_plugins(menu::MenuPlugin)
    .add_plugins(game::GamePlugin)
    .add_plugins((
      RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0),
      RapierDebugRenderPlugin::default(),
    ))
    .run();
}

fn setup(mut commands: Commands) {
  commands.spawn(Camera2dBundle::default());
}
