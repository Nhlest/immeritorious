use crate::game::ImmeritoriousState;
use bevy::prelude::*;

#[derive(Component)]
pub struct Active;

#[derive(Component)]
pub struct ButtonTag<const T: &'static str>;

#[derive(Component)]
pub struct TextField;

pub struct UiPlugin;

impl Plugin for UiPlugin {
  fn build(&self, app: &mut App) {
    app.add_systems(
      Update,
      (
        Self::text_input,
        Self::text_button_activate,
        Self::highlight_active_button,
      )
        .run_if(in_state(ImmeritoriousState::MainMenu)),
    );
  }
}

impl UiPlugin {
  fn text_button_activate(
    mut commands: Commands,
    buttons: Query<(Entity, &Interaction), (With<Button>, With<TextField>)>,
    active_buttons: Query<Entity, (With<Button>, With<Active>)>,
  ) {
    if let Some(new_active) = buttons
      .iter()
      .filter(|(_, i)| **i == Interaction::Pressed)
      .map(|(e, _)| e)
      .next()
    {
      active_buttons.iter().for_each(|e| {
        commands.entity(e).remove::<Active>();
      });
      commands.entity(new_active).insert(Active);
    }
  }
  fn highlight_active_button(
    mut buttons: Query<&mut BackgroundColor, (With<Button>, With<Active>)>,
    mut other_buttons: Query<(&mut BackgroundColor, &Interaction, Option<&TextField>), (With<Button>, Without<Active>)>,
  ) {
    if let Some(mut style) = buttons.iter_mut().next() {
      *style = Color::rgb(0.7, 0.1, 0.1).into();
    }
    other_buttons
      .iter_mut()
      .for_each(|(mut style, interaction, text_field)| {
        *style = match interaction {
          Interaction::Pressed => Color::rgb(0.3, 0.9, 0.0),
          Interaction::Hovered => Color::rgb(0.0, 0.3, 0.7),
          Interaction::None => {
            if text_field.is_none() {
              Color::rgb(0.3, 0.8, 0.3)
            } else {
              Color::rgb(0.8, 0.8, 0.8)
            }
          }
        }
        .into();
      });
  }
  fn text_input(
    keys: Res<Input<KeyCode>>,
    mut key_events: EventReader<ReceivedCharacter>,
    mut text_field: Query<&mut Text>,
    button: Query<&Children, (With<Button>, With<Active>)>,
  ) {
    if button.is_empty() {
      return;
    }
    for e in button.single().iter() {
      if let Ok(mut text) = text_field.get_mut(*e) {
        let section = &mut text.sections.first_mut().unwrap().value;
        if keys.just_pressed(KeyCode::Back) {
          section.pop();
        }
        for ReceivedCharacter { char, .. } in key_events.iter() {
          if char.is_alphanumeric() || char.is_ascii_punctuation() || char.is_whitespace() {
            section.push(*char);
          }
        }
        return;
      }
    }
  }
}

pub fn regular_button<const TAG: &'static str>(c: &mut ChildBuilder, asset_server: &AssetServer, text: &str) {
  c.spawn((
    ButtonBundle {
      style: Style {
        width: Val::Px(150.0),
        height: Val::Px(50.0),
        border: UiRect::all(Val::Px(2.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
      },
      background_color: Color::rgb(0.8, 0.8, 0.8).into(),
      border_color: Color::RED.into(),
      ..default()
    },
    ButtonTag::<TAG>,
  ))
  .with_children(|c| {
    c.spawn(TextBundle {
      style: Default::default(),
      text: Text::from_section(
        text,
        TextStyle {
          font: asset_server.load("quardratic.ttf"),
          font_size: 22.0,
          color: Color::BLACK,
        },
      ),
      text_layout_info: Default::default(),
      text_flags: Default::default(),
      ..default()
    });
  });
}

pub fn text_field_button<const TAG: &'static str>(c: &mut ChildBuilder, asset_server: &AssetServer, text: &str) {
  c.spawn((
    ButtonBundle {
      style: Style {
        width: Val::Px(150.0),
        height: Val::Px(50.0),
        border: UiRect::all(Val::Px(2.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
      },
      background_color: Color::rgb(0.8, 0.8, 0.8).into(),
      border_color: Color::RED.into(),
      ..default()
    },
    TextField,
  ))
  .with_children(|c| {
    c.spawn((
      TextBundle {
        style: Default::default(),
        text: Text::from_section(
          text,
          TextStyle {
            font: asset_server.load("quardratic.ttf"),
            font_size: 22.0,
            color: Color::BLACK,
          },
        ),
        text_layout_info: Default::default(),
        text_flags: Default::default(),
        ..default()
      },
      ButtonTag::<TAG>,
    ));
  });
}
