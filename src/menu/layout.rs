use bevy::prelude::*;

/// <div style="background-color:rgb(72.9%, 75.6%, 72.1%); width: 10px; padding: 10px; border: 1px solid;"></div>
pub const ASH_GREY: Color = Color::srgb(0.729, 0.756, 0.721);

/// <div style="background-color:rgb(34.5%, 64.3%, 69.0%); width: 10px; padding: 10px; border: 1px solid;"></div>
pub const MOONSTONE: Color = Color::srgb(0.345, 0.643, 0.690);

/// <div style="background-color:rgb(4.7%, 48.6%, 34.9%); width: 10px; padding: 10px; border: 1px solid;"></div>
pub const DARK_SPRING_GREEN: Color = Color::srgb(0.047, 0.486, 0.349);

/// <div style="background-color:rgb(16.9%, 18.8%, 22.7%); width: 10px; padding: 10px; border: 1px solid;"></div>
pub const GUNMETAL: Color = Color::srgb(0.169, 0.188, 0.227);

/// <div style="background-color:rgb(83.9%, 28.6%, 20.0%); width: 10px; padding: 10px; border: 1px solid;"></div>
pub const CHILI_RED: Color = Color::srgb(0.839, 0.286, 0.2);

pub const MENU_BACKGROUND_COLOR: Color = DARK_SPRING_GREEN;
pub const BUTTON_NORMAL_BACKGROUND_COLOR: Color = ASH_GREY;

// pub fn main_menu(commands: &mut Commands, assets: &Res<EsAssets>) -> Entity {
//     commands
//         .spawn(NodeBundle {
//             background_color: DARK_SPRING_GREEN.into(),
//             style: main_menu_style(),
//             ..default()
//         })
//         .with_children(|parent| {
//             parent.spawn(ButtonBundle {
//                 background_color: ASH_GREY.into(),
//                 style: Style {
//                     min_height: Val::Vh(5.),
//                     min_width: Val::Vw(25.),
//                     ..default()
//                 },
//                 ..default()
//             });
//             parent.spawn(ButtonBundle {
//                 background_color: ASH_GREY.into(),
//                 style: Style {
//                     min_height: Val::Vh(5.),
//                     min_width: Val::Vw(25.),
//                     ..default()
//                 },
//                 ..default()
//             });
//             parent.spawn(ButtonBundle {
//                 background_color: ASH_GREY.into(),
//                 style: Style {
//                     min_height: Val::Vh(5.),
//                     min_width: Val::Vw(25.),
//                     ..default()
//                 },
//                 ..default()
//             });
//         })
//         .id()
// }
//
// fn main_menu_style() -> Style {
//     Style {
//         width: Val::Vw(100.),
//         height: Val::Vh(100.),
//         flex_direction: FlexDirection::Column,
//         justify_content: JustifyContent::Center,
//         align_items: AlignItems::Center,
//         column_gap: Val::Px(8.),
//         row_gap: Val::Px(8.),
//         ..default()
//     }
// }
