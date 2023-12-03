use bevy::prelude::*;

#[allow(dead_code)]
#[derive(Debug, Clone, Eq, PartialEq, Hash, Default, States)]
pub enum GameState {
  #[default]
  MainMenu,
  InGame,
  Paused,
}
