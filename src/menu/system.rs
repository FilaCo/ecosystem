use bevy::color::palettes::css::WHITE;
use bevy::dev_tools::ui_debug_overlay::UiDebugOptions;
use bevy::prelude::*;

use crate::menu::layout::*;
use crate::prelude::*;

pub fn toggle_overlay(input: Res<ButtonInput<KeyCode>>, mut options: ResMut<UiDebugOptions>) {
    if input.just_pressed(KeyCode::Space) {
        options.toggle();
    }
}

pub fn setup_menu(mut menu_state: ResMut<NextState<EsMenuState>>) {
    menu_state.set(EsMenuState::Main);
}

pub fn setup_main_menu(mut commands: Commands, assets: Res<EsAssets>) {
    let menu_entity = commands
        .spawn(NodeBundle {
            background_color: DARK_SPRING_GREEN.into(),
            style: Style {
                width: Val::Vw(100.),
                height: Val::Vh(100.),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            ..default()
        })
        .insert(OnMainMenuScreen)
        .id();

    let play_button = commands
        .spawn(ButtonBundle {
            background_color: ASH_GREY.into(),
            style: Style {
                min_height: Val::Px(40.),
                min_width: Val::Px(150.),
                height: Val::Percent(15.),
                width: Val::Percent(40.),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            ..default()
        })
        .id();

    let play_button_text = commands
        .spawn(TextBundle::from_section(
            "PLAY",
            TextStyle {
                color: WHITE.into(),
                font: assets.font.clone(),
                font_size: 40.,
            },
        ))
        .id();

    commands
        .entity(play_button)
        .push_children(&[play_button_text]);

    commands.entity(menu_entity).push_children(&[play_button]);
}

pub fn setup_menu_camera(mut commands: Commands) {
    commands
        .spawn(EsMenuCamera)
        .insert(Camera2dBundle::default());
}

pub fn handle_button_interactions(
    mut q: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<Button>)>,
    mut menu_state: ResMut<NextState<EsMenuState>>,
    mut app_state: ResMut<NextState<EsAppState>>,
) {
    q.iter_mut()
        .for_each(|(interaction, mut background_color)| match interaction {
            Interaction::Pressed => {
                menu_state.set(EsMenuState::Disabled);
                app_state.set(EsAppState::SimulationRunning);
            }
            Interaction::Hovered => {}
            Interaction::None => {}
        });
}
