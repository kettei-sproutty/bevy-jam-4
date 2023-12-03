mod state;
mod ui;

use bevy::{app::AppExit, prelude::*};
use state::MenuState;
use ui::*;

use crate::{state::GameState, utils::despawn_screen};

pub struct MenuPlugin;

#[derive(Component)]
struct OnMainMenuScreen;

#[derive(Component)]
struct SelectedOption;

#[derive(Component)]
pub enum MenuButtonAction {
  Play,
  Settings,
  Credits,
  Quit,
}

impl Plugin for MenuPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_state::<MenuState>()
      .add_systems(OnEnter(GameState::MainMenu), menu_setup)
      .add_systems(
        OnExit(GameState::MainMenu),
        despawn_screen::<OnMainMenuScreen>,
      )
      .add_systems(OnEnter(MenuState::Main), main_menu_setup)
      .add_systems(
        Update,
        (menu_action, button_system).run_if(in_state(GameState::MainMenu)),
      );
  }
}

fn menu_setup(mut menu_state: ResMut<NextState<MenuState>>) {
  menu_state.set(MenuState::Main);
}

fn main_menu_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
  let container_style = Style {
    width: Val::Percent(100.0),
    height: Val::Percent(100.0),
    display: Display::Flex,
    flex_direction: FlexDirection::Column,
    justify_content: JustifyContent::Center,
    align_items: AlignItems::Center,
    column_gap: Val::Px(10.0),
    row_gap: Val::Px(10.0),
    ..default()
  };

  let container = NodeBundle {
    style: container_style.clone(),
    ..default()
  };

  commands
    .spawn((container, OnMainMenuScreen))
    .with_children(|parent| {
      parent.spawn(ImageBundle {
        style: Style {
          width: Val::Percent(100.0),
          height: Val::Percent(100.0),
          position_type: PositionType::Absolute,
          ..default()
        },
        image: asset_server.load("splash.png").into(),
        ..default()
      });
    })
    .with_children(|parent| {
      parent.spawn(
        TextBundle::from_section(
          "Friendly fire will not be tolerated",
          TextStyle {
            font_size: 60.0,
            color: Color::WHITE,
            ..default()
          },
        )
        .with_style(Style {
          margin: UiRect::all(Val::Px(50.0)),
          ..default()
        }),
      );
      create_button(parent, "Play", MenuButtonAction::Play);
      create_button(parent, "Settings", MenuButtonAction::Settings);
      create_button(parent, "Credits", MenuButtonAction::Credits);
      create_button(parent, "Quit", MenuButtonAction::Quit);
    });
}

fn menu_action(
  interaction_query: Query<(&Interaction, &MenuButtonAction), (Changed<Interaction>, With<Button>)>,
  mut app_exit_events: EventWriter<AppExit>,
  mut menu_state: ResMut<NextState<MenuState>>,
  mut game_state: ResMut<NextState<GameState>>,
) {
  for (interaction, menu_button_action) in &interaction_query {
    if *interaction == Interaction::Pressed {
      match menu_button_action {
        MenuButtonAction::Play => {
          game_state.set(GameState::InGame);
          menu_state.set(MenuState::Disabled);
        }
        MenuButtonAction::Settings => menu_state.set(MenuState::Settings),
        MenuButtonAction::Credits => menu_state.set(MenuState::Credits),
        MenuButtonAction::Quit => {
          app_exit_events.send(AppExit);
        }
      }
    }
  }
}

const NORMAL_BUTTON: Color = Color::WHITE;
const HOVERED_BUTTON: Color = Color::ANTIQUE_WHITE;
const HOVERED_PRESSED_BUTTON: Color = Color::GRAY;
const PRESSED_BUTTON: Color = Color::GRAY;

fn button_system(
  mut interaction_query: Query<
    (&Interaction, &mut BackgroundColor, Option<&SelectedOption>),
    (Changed<Interaction>, With<Button>),
  >,
) {
  for (interaction, mut color, selected) in &mut interaction_query {
    *color = match (*interaction, selected) {
      (Interaction::Pressed, _) | (Interaction::None, Some(_)) => PRESSED_BUTTON.into(),
      (Interaction::Hovered, Some(_)) => HOVERED_PRESSED_BUTTON.into(),
      (Interaction::Hovered, None) => HOVERED_BUTTON.into(),
      (Interaction::None, None) => NORMAL_BUTTON.into(),
    }
  }
}
