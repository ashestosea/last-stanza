use crate::GameState;
use bevy::{prelude::*, math::vec3, render::camera::{ScalingMode, Camera2d}};

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
	fn build(&self, app: &mut App) {
		app
			.init_resource::<ButtonColors>()
			.add_system_set(SystemSet::on_enter(GameState::Menu).with_system(setup_menu))
            .add_system_set(SystemSet::on_update(GameState::Menu).with_system(click_play_button))
            .add_system_set(SystemSet::on_exit(GameState::Menu).with_system(cleanup_menu));
	}
}

struct ButtonColors {
    normal: UiColor,
    hovered: UiColor,
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
	mut state: ResMut<State<GameState>>,
    button_colors: Res<ButtonColors>,
) {
	let mut cam = OrthographicCameraBundle::<Camera2d>::new_2d();
	cam.transform.translation = vec3(0., 40., 0.);
	cam.orthographic_projection.scaling_mode = ScalingMode::FixedHorizontal;
	cam.orthographic_projection.scale = 100.;
	commands.spawn_bundle(cam);
	                state.set(GameState::Playing).unwrap();

	
    commands
        .spawn_bundle(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(120.0), Val::Px(50.0)),
                // margin: UiRect::all(Val::Auto),
				// margin: 
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            color: button_colors.normal,
            ..Default::default()
        });
        // .with_children(|parent| {
        //     parent.spawn_bundle(TextBundle {
        //         text: Text {
        //             sections: vec![TextSection {
        //                 value: "Play".to_string(),
        //                 style: TextStyle {
        //                     // font: font_assets.fira_sans.clone(),
        //                     font_size: 40.0,
        //                     color: Color::rgb(0.9, 0.9, 0.9),
        //                 }
        //             }],
        //             alignment: Default::default(),
        //         },
        //         ..Default::default()
        //     });
        // });
}

fn click_play_button(
    button_colors: Res<ButtonColors>,
    mut state: ResMut<State<GameState>>,
    mut interaction_query: Query<
        (&Interaction, &mut UiColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    interaction_query.for_each_mut(|(interaction, mut color)| {
        match *interaction {
            Interaction::Clicked => {
                state.set(GameState::Playing).unwrap();
            }
            Interaction::Hovered => {
                *color = button_colors.hovered;
            }
            Interaction::None => {
                *color = button_colors.normal;
            }
        }
	});
}

fn cleanup_menu(mut commands: Commands, button: Query<Entity, With<Button>>) {
    commands.entity(button.single()).despawn_recursive();
}
