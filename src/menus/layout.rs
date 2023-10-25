use crate::menus::components::*;
use bevy::prelude::Deref;
use bevy::prelude::DerefMut;
use bevy::prelude::Timer;
use bevy::prelude::*;
use crate::game::camera::SpatialCameraBundle;
use crate::game::components::*;

pub const SCREEN_WIDTH: f32 = 1280.0;
pub const SCREEN_HEIGHT: f32 = 720.0;
pub const PADDING: f32 = 20.0;

#[derive(Component, Deref, DerefMut)]
pub struct PopupTimer(Timer);

pub fn spawn_main_menu(
    mut commands: Commands, 
    asset_server: Res<AssetServer>
) {
    build_main_menu(&mut commands, &asset_server);
}

pub fn despawn_main_menu(
    mut commands: Commands, 
    main_menu_query: Query<Entity, With<MainMenu>>
) {
    if let Ok(main_menu_entity) = main_menu_query.get_single() {
        commands.entity(main_menu_entity).despawn_recursive();
    }
}

pub fn build_main_menu(
    commands: &mut Commands, 
    asset_server: &Res<AssetServer>
) -> Entity {
    let main_menu_entity = commands
        .spawn((
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..default()
                },
                background_color: Color::WHITE.into(),
                ..default()
            },
            MainMenu {},
        ))
        //play but
        .with_children(|parent| {
            //title
            parent
                .spawn((ButtonBundle {
                    style: Style {
                        width: Val::Px(600.0),
                        height: Val::Px(80.0),
                        margin: UiRect {
                            left: Val::Px(8.),
                            right: Val::Px(8.),
                            top: Val::Px(0.0),
                            bottom: Val::Px(60.0),
                        },
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: Color::WHITE.into(),
                    ..default()
                },))
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection::new(
                                "Red Delicious",
                                TextStyle {
                                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                    font_size: 64.0,
                                    color: Color::RED,
                                },
                            )],
                            alignment: TextAlignment::Center,
                            ..default()
                        },
                        ..default()
                    });
                });
            //host butt
            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(200.0),
                            height: Val::Px(80.0),
                            margin: UiRect {
                                left: Val::Px(8.),
                                right: Val::Px(8.),
                                top: Val::Px(0.0),
                                bottom: Val::Px(8.0),
                            },
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        background_color: Color::rgb(0.15, 0.15, 0.15).into(),
                        ..default()
                    },
                    HostButton {},
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection::new(
                                "Host",
                                TextStyle {
                                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                    font_size: 20.0,
                                    color: Color::WHITE,
                                },
                            )],
                            alignment: TextAlignment::Center,
                            ..default()
                        },
                        ..default()
                    });
                });
            //join butt
            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(200.0),
                            height: Val::Px(80.0),
                            margin: UiRect {
                                left: Val::Px(8.),
                                right: Val::Px(8.),
                                top: Val::Px(0.0),
                                bottom: Val::Px(8.0),
                            },
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        background_color: Color::rgb(0.15, 0.15, 0.15).into(),
                        ..default()
                    },
                    JoinButton {},
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection::new(
                                "Join a session",
                                TextStyle {
                                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                    font_size: 20.0,
                                    color: Color::WHITE,
                                },
                            )],
                            alignment: TextAlignment::Center,
                            ..default()
                        },
                        ..default()
                    });
                });
            //controls butt
            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(200.0),
                            height: Val::Px(80.0),
                            margin: UiRect {
                                left: Val::Px(8.),
                                right: Val::Px(8.),
                                top: Val::Px(0.0),
                                bottom: Val::Px(8.0),
                            },
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        background_color: Color::rgb(0.15, 0.15, 0.15).into(),
                        ..default()
                    },
                    ControlsButton {},
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection::new(
                                "Controls",
                                TextStyle {
                                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                    font_size: 20.0,
                                    color: Color::WHITE,
                                },
                            )],
                            alignment: TextAlignment::Center,
                            ..default()
                        },
                        ..default()
                    });
                });
            //credits but
            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(200.0),
                            height: Val::Px(80.0),
                            margin: UiRect {
                                left: Val::Px(8.),
                                right: Val::Px(8.),
                                top: Val::Px(0.0),
                                bottom: Val::Px(8.0),
                            },
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        background_color: Color::rgb(0.15, 0.15, 0.15).into(),
                        ..default()
                    },
                    CreditsButton {},
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection::new(
                                "Credits page",
                                TextStyle {
                                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                    font_size: 20.0,
                                    color: Color::WHITE,
                                },
                            )],
                            alignment: TextAlignment::Center,
                            ..default()
                        },
                        ..default()
                    });
                });
        })
        .id();

    main_menu_entity
}

pub fn spawn_credits_page(
    mut commands: Commands, 
    asset_server: Res<AssetServer>,
    mut cameras: Query<&mut Transform, With<SpatialCameraBundle>>,
) {
    for mut tf in cameras.iter_mut() {
        let translation = Vec3::new(0.0, 0.0, 1.0);
        tf.translation = translation;
    }
    commands
        .spawn((
            SpriteBundle {
                texture: asset_server.load("brendan_credits_slide.png"),
                transform: Transform::from_xyz(0., 0., -0.9)
                .with_scale(Vec3::new(0.5, 0.5, 1.0)),
                ..default()
            }, 
            Popup,
        ))
        .insert(PopupTimer(Timer::from_seconds(0., TimerMode::Once)));
    commands
        .spawn((
            SpriteBundle {
                texture: asset_server.load("CreditAlexLampe.png"),
                transform: Transform::from_xyz(0., 0., -0.8)
                .with_scale(Vec3::new(0.5, 0.5, 1.0)),
                ..default()
            },
            Popup,
        ))
        .insert(PopupTimer(Timer::from_seconds(3., TimerMode::Once)));
    commands
        .spawn((
            SpriteBundle {
                texture: asset_server.load("CreditGarrettDiCenzo.png"),
                transform: Transform::from_xyz(0., 0., -0.7)
                .with_scale(Vec3::new(0.5, 0.5, 1.0)),
                ..default()
            },
            Popup,
        ))
        .insert(PopupTimer(Timer::from_seconds(6., TimerMode::Once)));
    commands
        .spawn((
            SpriteBundle {
                texture: asset_server.load("CreditIanWhitfield.png"),
                transform: Transform::from_xyz(0., 0., -0.6)
                .with_scale(Vec3::new(0.5, 0.5, 1.0)),
                ..default()
            },
            Popup,
        ))
        .insert(PopupTimer(Timer::from_seconds(9., TimerMode::Once)));
    commands
        .spawn((
            SpriteBundle {
                texture: asset_server.load("CreditJordanBrudenell.png"),
                transform: Transform::from_xyz(0., 0., -0.5)
                .with_scale(Vec3::new(0.5, 0.5, 1.0)),
                ..default()
            },
            Popup,
        ))
        .insert(PopupTimer(Timer::from_seconds(12., TimerMode::Once)));
    commands
        .spawn((
            SpriteBundle {
                texture: asset_server.load("CreditRuohengXu.png"),
                transform: Transform::from_xyz(0., 0., -0.4)
                .with_scale(Vec3::new(0.5, 0.5, 1.0)),
                ..default()
            },
            Popup,
        ))
        .insert(PopupTimer(Timer::from_seconds(15., TimerMode::Once)));
    commands
        .spawn((
            SpriteBundle {
                texture: asset_server.load("CreditSamDurigon.png"),
                transform: Transform::from_xyz(0., 0., -0.3)
                .with_scale(Vec3::new(0.5, 0.5, 1.0)),
                ..default()
            },
            Popup,
        ))
        .insert(PopupTimer(Timer::from_seconds(18., TimerMode::Once)));
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::FlexEnd,
                    align_items: AlignItems::Center,
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..default()
                },
                ..default()
            },
            Popup,
        ))
        .with_children(|parent| {
            parent.spawn(
        (ButtonBundle {
                    style: Style {
                        width: Val::Px(200.0),
                        height: Val::Px(80.0),
                        margin: UiRect {
                            left: Val::Px(8.),
                            right: Val::Px(8.),
                            top: Val::Px(0.0),
                            bottom: Val::Px(8.0),
                        },
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: Color::rgb(0.15, 0.15, 0.15).into(),
                    ..default()
                },
                BackToMainMenu {},
            ))
            .with_children(|parent| {
                parent.spawn(TextBundle {
                    text: Text {
                        sections: vec![TextSection::new(
                            "Back",
                            TextStyle {
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: 20.0,
                                color: Color::WHITE,
                            },
                        )],
                        alignment: TextAlignment::Center,
                        ..default()
                    },
                    ..default()
                });
            });
        });
}

pub fn despawn_credits_page(
    mut commands: Commands, 
    credits_page_query: Query<Entity, With<Popup>>,
) {
    for entity in credits_page_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn show_popup(
    time: Res<Time>, 
    mut popup: Query<(&mut PopupTimer, 
    &mut Transform)>
) {
    for (mut timer, mut transform) in popup.iter_mut() {
        timer.tick(time.delta());
        if timer.just_finished() {
            transform.translation.z += 10.;
        }
    }
}

pub fn spawn_host_page(
    mut commands: Commands, 
    asset_server: Res<AssetServer>
) {
     build_host_page(&mut commands, &asset_server);
}

pub fn despawn_host_page(
    mut commands: Commands, 
    host_page_entity: Query<Entity, 
    With<HostPage>>
) {
    if let Ok(host_page_entity) = host_page_entity.get_single() {
        commands.entity(host_page_entity).despawn_recursive();
    }
}

pub fn build_host_page(
    commands: &mut Commands, 
    asset_server: &Res<AssetServer>
) -> Entity {
    let host_page_entity = commands
        .spawn((
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..default()
                },
                background_color: Color::WHITE.into(),
                ..default()
            },
            HostPage {},
        ))
        .with_children(|parent| {
            //title
            parent
                .spawn((ButtonBundle {
                    style: Style {
                        width: Val::Px(600.0),
                        height: Val::Px(80.0),
                        margin: UiRect {
                            left: Val::Px(8.),
                            right: Val::Px(8.),
                            top: Val::Px(0.0),
                            bottom: Val::Px(60.0),
                        },
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: Color::WHITE.into(),
                    ..default()
                },))
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection::new(
                                "Hosting",
                                TextStyle {
                                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                    font_size: 64.0,
                                    color: Color::RED,
                                },
                            )],
                            alignment: TextAlignment::Center,
                            ..default()
                        },
                        ..default()
                    });
                });

            parent.spawn((
                TextBundle {
                    style: Style {
                        width: Val::Px(400.0),
                        height: Val::Px(80.0),
                        margin: UiRect {
                            left: Val::Px(8.),
                            right: Val::Px(8.),
                            top: Val::Px(0.0),
                            bottom: Val::Px(8.0),
                        },
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    text: Text {
                        sections: vec![TextSection::new(
                            "Enter your port number here: ",
                            TextStyle {
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: 24.0,
                                color: Color::BLACK,
                            },
                        )],
                        alignment: TextAlignment::Center,
                        ..default()
                    },
                    ..default()
                },
                HostPortInput {
                    port: String::new(),
                },
            ));

            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(200.0),
                            height: Val::Px(80.0),
                            margin: UiRect {
                                left: Val::Px(8.),
                                right: Val::Px(8.),
                                top: Val::Px(0.0),
                                bottom: Val::Px(8.0),
                            },
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        background_color: Color::rgb(0.15, 0.15, 0.15).into(),
                        ..default()
                    },
                    HostPortSaveBut {},
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection::new(
                                "Host now",
                                TextStyle {
                                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                    font_size: 20.0,
                                    color: Color::WHITE,
                                },
                            )],
                            alignment: TextAlignment::Center,
                            ..default()
                        },
                        ..default()
                    });
                });

            //back to main menu
            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(200.0),
                            height: Val::Px(80.0),
                            margin: UiRect {
                                left: Val::Px(8.),
                                right: Val::Px(8.),
                                top: Val::Px(0.0),
                                bottom: Val::Px(8.0),
                            },
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        background_color: Color::rgb(0.15, 0.15, 0.15).into(),
                        ..default()
                    },
                    BackToMainMenu {},
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection::new(
                                "Back",
                                TextStyle {
                                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                    font_size: 20.0,
                                    color: Color::WHITE,
                                },
                            )],
                            alignment: TextAlignment::Center,
                            ..default()
                        },
                        ..default()
                    });
                });
        })
        .id();
    host_page_entity
}

pub fn spawn_join_page(
    mut commands: Commands, 
    asset_server: Res<AssetServer>
) {
    build_join_page(&mut commands, &asset_server);
}

pub fn despawn_join_page(
    mut commands: Commands, 
    join_page_entity: Query<Entity, 
    With<JoinPage>>
) {
    if let Ok(join_page_entity) = join_page_entity.get_single() {
        commands.entity(join_page_entity).despawn_recursive();
    }
}

pub fn build_join_page(
    commands: &mut Commands, 
    asset_server: &Res<AssetServer>
) -> Entity {
    let join_page_entity = commands
        .spawn((
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..default()
                },
                background_color: Color::WHITE.into(),
                ..default()
            },
            JoinPage {},
        ))
        .with_children(|parent| {
            //title
            parent
                .spawn((ButtonBundle {
                    style: Style {
                        width: Val::Px(600.0),
                        height: Val::Px(80.0),
                        margin: UiRect {
                            left: Val::Px(8.),
                            right: Val::Px(8.),
                            top: Val::Px(0.0),
                            bottom: Val::Px(60.0),
                        },
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: Color::WHITE.into(),
                    ..default()
                },))
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection::new(
                                "Join an existing session",
                                TextStyle {
                                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                    font_size: 64.0,
                                    color: Color::RED,
                                },
                            )],
                            alignment: TextAlignment::Center,
                            ..default()
                        },
                        ..default()
                    });
                });

            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(500.0),
                            height: Val::Px(80.0),
                            margin: UiRect {
                                left: Val::Px(8.),
                                right: Val::Px(8.),
                                top: Val::Px(0.0),
                                bottom: Val::Px(8.0),
                            },
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        background_color: Color::rgb(0.15, 0.15, 0.15).into(),
                        ..default()
                    },
                    Switch {
                        host_port: false,
                        port: false,
                        ip: false,
                    },
                    JoinPortBut {},
                ))
                .with_children(|parent| {
                    parent.spawn((
                        TextBundle {
                            text: Text {
                                sections: vec![TextSection::new(
                                    "Your Port#: ",
                                    TextStyle {
                                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                        font_size: 24.0,
                                        color: Color::WHITE,
                                    },
                                )],
                                alignment: TextAlignment::Center,
                                ..default()
                            },
                            ..default()
                        },
                        JoinPortInput {
                            port: String::new(),
                        },
                    ));
                });

                parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(500.0),
                            height: Val::Px(80.0),
                            margin: UiRect {
                                left: Val::Px(8.),
                                right: Val::Px(8.),
                                top: Val::Px(0.0),
                                bottom: Val::Px(8.0),
                            },
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        background_color: Color::rgb(0.15, 0.15, 0.15).into(),
                        ..default()
                    },
                    Switch {
                        host_port: false,
                        port: false,
                        ip: false,
                    },
                    JoinHostPortBut {},
                ))
                .with_children(|parent| {
                    parent.spawn((
                        TextBundle {
                            text: Text {
                                sections: vec![TextSection::new(
                                    "Hosting Port#: ",
                                    TextStyle {
                                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                        font_size: 24.0,
                                        color: Color::WHITE,
                                    },
                                )],
                                alignment: TextAlignment::Center,
                                ..default()
                            },
                            ..default()
                        },
                        JoinHostPortInput {
                            port: String::new(),
                        },
                    ));
                });

            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(500.0),
                            height: Val::Px(80.0),
                            margin: UiRect {
                                left: Val::Px(8.),
                                right: Val::Px(8.),
                                top: Val::Px(0.0),
                                bottom: Val::Px(8.0),
                            },
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        background_color: Color::rgb(0.15, 0.15, 0.15).into(),
                        ..default()
                    },
                    Switch {
                        host_port: false,
                        port: false,
                        ip: false,
                    },
                    JoinIpBut {},
                ))
                .with_children(|parent| {
                    parent.spawn((
                        TextBundle {
                            text: Text {
                                sections: vec![TextSection::new(
                                    "IP#: ",
                                    TextStyle {
                                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                        font_size: 24.0,
                                        color: Color::WHITE,
                                    },
                                )],
                                alignment: TextAlignment::Center,
                                ..default()
                            },
                            ..default()
                        },
                        JoinIPInput { ip: String::new() },
                    ));
                });

            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(200.0),
                            height: Val::Px(80.0),
                            margin: UiRect {
                                left: Val::Px(8.),
                                right: Val::Px(8.),
                                top: Val::Px(0.0),
                                bottom: Val::Px(8.0),
                            },
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        background_color: Color::rgb(0.15, 0.15, 0.15).into(),
                        ..default()
                    },
                    JoinSaveBut {},
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection::new(
                                "Join now",
                                TextStyle {
                                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                    font_size: 20.0,
                                    color: Color::WHITE,
                                },
                            )],
                            alignment: TextAlignment::Center,
                            ..default()
                        },
                        ..default()
                    });
                });

            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(200.0),
                            height: Val::Px(80.0),
                            margin: UiRect {
                                left: Val::Px(8.),
                                right: Val::Px(8.),
                                top: Val::Px(0.0),
                                bottom: Val::Px(8.0),
                            },
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        background_color: Color::rgb(0.15, 0.15, 0.15).into(),
                        ..default()
                    },
                    BackToMainMenu {},
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection::new(
                                "Back",
                                TextStyle {
                                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                    font_size: 20.0,
                                    color: Color::WHITE,
                                },
                            )],
                            alignment: TextAlignment::Center,
                            ..default()
                        },
                        ..default()
                    });
                });
        })
        .id();
    join_page_entity
}

pub fn spawn_controls_page(
    mut commands: Commands, 
    asset_server: Res<AssetServer>
) {
     build_controls_page(&mut commands, &asset_server);
}

pub fn despawn_controls_page(
    mut commands: Commands,
    controls_page_entity: Query<Entity, With<ControlsPage>>,
) {
    if let Ok(controls_page_entity) = controls_page_entity.get_single() {
        commands.entity(controls_page_entity).despawn_recursive();
    }
}

pub fn build_controls_page(
    commands: &mut Commands, 
    asset_server: &Res<AssetServer>
) -> Entity {
    let controls_page_entity = commands
        .spawn((
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..default()
                },
                background_color: Color::WHITE.into(),
                ..default()
            },
            ControlsPage {},
        ))
        .with_children(|parent| {
            // Controls Title
            parent.spawn(ButtonBundle {
                style: Style {
                    width: Val::Px(600.0),
                    height: Val::Px(80.0),
                    margin: UiRect {
                        left: Val::Px(8.),
                        right: Val::Px(8.),
                        top: Val::Px(0.0),
                        bottom: Val::Px(60.0),
                    },
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: Color::WHITE.into(),
                ..default()
            })
            .with_children(|parent| {
                parent.spawn(TextBundle {
                    text: Text {
                        sections: vec![TextSection::new(
                            "Controls",
                            TextStyle {
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: 60.0,
                                color: Color::RED,
                            },
                        )],
                        alignment: TextAlignment::Center,
                        ..default()
                    },
                    ..default()
                });
            });
            // Controls Text
            parent.spawn(TextBundle {
                text: Text {
                    sections: vec![TextSection::new(
                        "Movement - WASD",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 24.0,
                            color: Color::BLACK,
                        },
                    )],
                    alignment: TextAlignment::Center,
                    ..default()
                },
                ..default()
            });
            parent.spawn(TextBundle {
                text: Text {
                    sections: vec![TextSection::new(
                        "Attack - Left Click",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 24.0,
                            color: Color::BLACK,
                        },
                    )],
                    alignment: TextAlignment::Center,
                    ..default()
                },
                ..default()
            });
            parent.spawn(TextBundle {
                text: Text {
                    sections: vec![TextSection::new(
                        "Interact - E",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 24.0,
                            color: Color::BLACK,
                        },
                    )],
                    alignment: TextAlignment::Center,
                    ..default()
                },
                ..default()
            });
            parent.spawn(TextBundle {
                text: Text {
                    sections: vec![TextSection::new(
                        "Quit Game - Esc",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 24.0,
                            color: Color::BLACK,
                        },
                    )],
                    alignment: TextAlignment::Center,
                    ..default()
                },
                ..default()
            });
            // Back Button
            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(200.0),
                            height: Val::Px(80.0),
                            margin: UiRect {
                                left: Val::Px(8.),
                                right: Val::Px(8.),
                                top: Val::Px(60.0),
                                bottom: Val::Px(8.0),
                            },
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        background_color: Color::rgb(0.15, 0.15, 0.15).into(),
                        ..default()
                    },
                    BackToMainMenu {},
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection::new(
                                "Back",
                                TextStyle {
                                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                    font_size: 20.0,
                                    color: Color::WHITE,
                                },
                            )],
                            alignment: TextAlignment::Center,
                            ..default()
                        },
                        ..default()
                    });
                });
        })  
        .id();
    controls_page_entity
}

pub fn spawn_in_game_menu(
    mut commands: Commands, 
    asset_server: Res<AssetServer>
) {
    build_in_game_menu(&mut commands, &asset_server);
}

pub fn despawn_in_game_menu(
    mut commands: Commands, 
    in_game_menu_entity: Query<Entity, With<JoinPage>>
) {
    if let Ok(in_game_menu_entity) = in_game_menu_entity.get_single() {
        commands.entity(in_game_menu_entity).despawn_recursive();
    }
}

#[derive(Component)]
pub struct GameTimer {
    remaining_time: f32, // time in seconds
}


pub fn build_in_game_menu(
    commands: &mut Commands, 
    asset_server: &Res<AssetServer>
) -> Entity {
    let in_game_menu_entity = commands
        .spawn((
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            JoinPage {},
        ))
        .with_children(|parent| {
            // Score Display
            parent.spawn((TextBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    left: Val::Px(PADDING),
                    top: Val::Px(SCREEN_HEIGHT - PADDING - 64.0),
                    ..Default::default()
                },
                text: Text {
                    sections: vec![TextSection::new(
                        "Score: 0", //TODO dynamic binding of score attribute of the player
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 64.0,
                            color: Color::RED,
                        },
                    )],
                    alignment: TextAlignment::Left,
                    ..Default::default()
                },
                ..Default::default()},
                ScoreDisplay,)
            );

            // Timer Display
            let timer_entity = parent.spawn(TextBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    left: Val::Px(SCREEN_WIDTH / 2.0 - 100.0),
                    top: Val::Px(PADDING),
                    ..Default::default()
                },
                text: Text {
                    sections: vec![TextSection::new(
                        "5:00",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 64.0,
                            color: Color::WHITE,
                        },
                    )],
                    alignment: TextAlignment::Center,
                    ..Default::default()
                },
                ..Default::default()
            }).insert(GameTimer {
                remaining_time: 5.0 * 60.0,
            });
        })
        .id();

    in_game_menu_entity
}

pub fn update_time_remaining_system(
    time: Res<Time>, 
    mut game_timer_query: Query<(&mut GameTimer, &mut Text)>,
    mut join_page_query: Query<&mut Style, With<JoinPage>>,
    mut game_over_query: Query<&mut Style, (With<GameOver>, Without<JoinPage>)>,
) {
    for (mut timer, mut text) in &mut game_timer_query {
        if timer.remaining_time > 0.0 {
            timer.remaining_time -= time.delta_seconds();

            let minutes = (timer.remaining_time / 60.0) as i32;
            let seconds = (timer.remaining_time % 60.0) as i32;

            text.sections[0].value = format!("{:02}:{:02}", minutes, seconds);
        } else {
            // TODO Handle game over logic
            for mut style in join_page_query.iter_mut() {
                style.display = Display::None;
            }
            for mut style in game_over_query.iter_mut() {
                style.display = Display::Flex;
            }

        }
    }
}

pub fn build_game_over_screen(
    commands: &mut Commands, 
    asset_server: &Res<AssetServer>
) -> Entity {
    let game_over_entity = commands
        .spawn((
            NodeBundle {
                style: Style {
                    display: Display::None,
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..default()
                },
                background_color: Color::Rgba { red: (1.0), green: (1.0), blue: (1.0), alpha: (0.5) }.into(),
                ..default()
            },
            GameOver {},
        ))
        .with_children(|parent| {
            //title
            parent
                .spawn((ButtonBundle {
                    style: Style {
                        width: Val::Px(600.0),
                        height: Val::Px(80.0),
                        margin: UiRect {
                            left: Val::Px(8.),
                            right: Val::Px(8.),
                            top: Val::Px(0.0),
                            bottom: Val::Px(60.0),
                        },
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: Color::WHITE.into(),
                    ..default()
                },))
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection::new(
                                "Game Over",
                                TextStyle {
                                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                    font_size: 64.0,
                                    color: Color::RED,
                                },
                            )],
                            alignment: TextAlignment::Center,
                            ..default()
                        },
                        ..default()
                    });
                });
            //credits butt
            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(200.0),
                            height: Val::Px(80.0),
                            margin: UiRect {
                                left: Val::Px(8.),
                                right: Val::Px(8.),
                                top: Val::Px(0.0),
                                bottom: Val::Px(8.0),
                            },
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        background_color: Color::rgb(0.15, 0.15, 0.15).into(),
                        ..default()
                    },
                    CreditsButton {},
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection::new(
                                "Credits",
                                TextStyle {
                                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                    font_size: 20.0,
                                    color: Color::WHITE,
                                },
                            )],
                            alignment: TextAlignment::Center,
                            ..default()
                        },
                        ..default()
                    });
                });
            // main menu butt
            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(200.0),
                            height: Val::Px(80.0),
                            margin: UiRect {
                                left: Val::Px(8.),
                                right: Val::Px(8.),
                                top: Val::Px(0.0),
                                bottom: Val::Px(8.0),
                            },
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        background_color: Color::rgb(0.15, 0.15, 0.15).into(),
                        ..default()
                    },
                    BackToMainMenu {},
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection::new(
                                "Main Menu",
                                TextStyle {
                                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                    font_size: 20.0,
                                    color: Color::WHITE,
                                },
                            )],
                            alignment: TextAlignment::Center,
                            ..default()
                        },
                        ..default()
                    });
                });
        })
        .id();

    game_over_entity
}

pub fn spawn_game_over_screen(
    mut commands: Commands, 
    asset_server: Res<AssetServer>
) {
    build_game_over_screen(&mut commands, &asset_server);
}

pub fn despawn_game_over_screen(
    mut commands: Commands, 
    game_over_entity: Query<Entity, 
    With<GameOver>>
) {
    if let Ok(game_over_entity) = game_over_entity.get_single() {
        commands.entity(game_over_entity).despawn_recursive();
    }
}