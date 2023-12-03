use bevy::prelude::*;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum MenuState {
  Main,
  Settings,
  Credits,
  #[default]
  Disabled,
}
