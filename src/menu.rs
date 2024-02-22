use crate::loading::FontAssets;
use crate::GameState;
use bevy::prelude::*;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ButtonColors>()
            .add_systems(OnEnter(GameState::Menu), setup_menu)
            .add_systems(Update, click_play_button.run_if(in_state(GameState::Menu)))
            .add_systems(OnExit(GameState::Menu), cleanup_menu);
    }
}

#[derive(Resource)]
struct ButtonColors {
    normal: BackgroundColor,
    hovered: BackgroundColor,
}

impl Default for ButtonColors {
    fn default() -> Self {
        ButtonColors {
            normal: Color::rgb(0.15, 0.15, 0.15).into(),
            hovered: Color::rgb(0.25, 0.25, 0.25).into(),
        }
    }
}

fn setup_menu(
    mut commands: Commands,
    button_colors: Res<ButtonColors>,
    font_assets: Res<FontAssets>,
) {
    commands
        .spawn(ButtonBundle {
            style: Style {
                width: Val::Px(120.0),
                height: Val::Px(50.0),
                margin: UiRect::all(Val::Auto),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            background_color: button_colors.normal,
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text {
                    sections: vec![TextSection {
                        value: "Play".to_string(),
                        style: TextStyle {
                            font: font_assets.fantasque_sans.clone(),
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    }],
                    justify: JustifyText::Center,
                    ..Default::default()
                },
                ..Default::default()
            });
        });
}

fn click_play_button(
    button_colors: Res<ButtonColors>,
    mut state: ResMut<NextState<GameState>>,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    interaction_query
        .iter_mut()
        .for_each(|(interaction, mut color)| match *interaction {
            Interaction::Pressed => {
                state.set(GameState::Playing);
            }
            Interaction::Hovered => {
                *color = button_colors.hovered;
            }
            Interaction::None => {
                *color = button_colors.normal;
            }
        });
}

fn cleanup_menu(mut commands: Commands, button: Query<Entity, With<Button>>) {
    commands.entity(button.single()).despawn_recursive();
}
