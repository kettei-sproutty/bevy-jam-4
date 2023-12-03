use crate::state::GameState;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
  fn build(&self, app: &mut App) {
    app
      .add_systems(OnEnter(GameState::InGame), setup_physics)
      .add_systems(Update, print_ball_altitude);
  }
}

fn setup_physics(mut commands: Commands) {
  /* Create the ground. */
  commands
    .spawn(Collider::cuboid(500.0, 50.0))
    .insert(TransformBundle::from(Transform::from_xyz(0.0, -100.0, 0.0)));

  /* Create the bouncing ball. */
  commands
    .spawn(RigidBody::Dynamic)
    .insert(Collider::ball(50.0))
    .insert(Restitution::coefficient(0.7))
    .insert(TransformBundle::from(Transform::from_xyz(0.0, 400.0, 0.0)));
}

fn print_ball_altitude(positions: Query<&Transform, With<RigidBody>>) {
  for transform in positions.iter() {
    println!("Ball altitude: {}", transform.translation.y);
  }
}
