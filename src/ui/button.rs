use bevy::prelude::*;

pub fn create_button<'a>(children: &mut ChildBuilder, text: &str, action: impl Component) -> () {
  let button_style = Style {
    width: Val::Px(200.0),
    height: Val::Px(40.0),
    ..default()
  };

  let button_text_style = Style {
    align_self: AlignSelf::Center,
    margin: UiRect::all(Val::Px(5.0)),
    ..default()
  };

  let button = ButtonBundle {
    style: button_style,
    ..default()
  };

  let text_style = TextStyle {
    color: Color::BLACK,
    ..default()
  };

  children.spawn((button, action)).with_children(|parent| {
    parent.spawn(TextBundle::from_section(text, text_style).with_style(button_text_style));
  });
}
