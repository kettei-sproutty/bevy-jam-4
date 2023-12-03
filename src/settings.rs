use bevy::prelude::*;

#[allow(dead_code)]
#[derive(Resource, Debug, Component, PartialEq, Eq, Clone, Copy, Default)]
pub enum Difficulty {
  Easy,
  #[default]
  Medium,
  Hard,
}
