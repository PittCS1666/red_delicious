use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use bevy::window::PrimaryWindow;
use crate::game::player::{LocalPlayer, PLAYER_DEFAULT_HP};
use crate::{map, map::WorldMap};
use crate::movement;
use crate::AppState;
use crate::game::components::Health;
use crate::game::player;

pub const GAME_PROJ_SCALE: f32 = 0.5;

const MINIMAP_DIMENSIONS: UVec2 = UVec2::new(map::MAPSIZE as u32, map::MAPSIZE as u32);
const MINIMAP_PAD: UVec2 = UVec2::new(32, 32); // How many pixels between top right of window and top right of minimap (not border)
const MINIMAP_TRANSLATION: Vec3 = Vec3::new(
    ((super::WIN_W / 2.) as u32 - MINIMAP_PAD.x - (MINIMAP_DIMENSIONS.x / 2)) as f32 * GAME_PROJ_SCALE,
    ((super::WIN_H / 2.) as u32 - MINIMAP_PAD.y - (MINIMAP_DIMENSIONS.y / 2)) as f32 * GAME_PROJ_SCALE,
    5.
);

#[derive(Component)]
pub struct GameCamera;

#[derive(Component)]
pub struct Marker;

#[derive(Component)]
pub struct Minimap;

#[derive(Component)]
pub struct MinimapBorder;

#[derive(Component)]
pub struct RespawnMap;

#[derive(Component)]
pub struct SpatialCameraBundle;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, startup)
            .add_systems(Update, game_update.after(movement::handle_move).run_if(in_state(AppState::Game)))
            .add_systems(Update, respawn_update.run_if(in_state(AppState::Respawn)))
            .add_systems(OnExit(AppState::MainMenu), spawn_minimap)
            .add_systems(OnEnter(AppState::Respawn), configure_map)
            .add_systems(OnExit(AppState::Respawn), configure_map);
    }
}

fn startup(
    commands: Commands,
) {
    spawn_camera(commands);
}

fn spawn_camera(
    mut commands: Commands
) {
    commands.spawn((
        SpatialBundle {
            ..Default::default()
        },
        SpatialCameraBundle,
    )).with_children(|parent|{
            parent.spawn((
                Camera2dBundle {
                    projection: OrthographicProjection{near: -1000., scale: GAME_PROJ_SCALE, ..default()},
                    ..default()
                },
                GameCamera,
            ));
        },
    );
}

fn spawn_minimap(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut assets: ResMut<Assets<Image>>,
    map: Res<WorldMap>,
    mut cam_bundle: Query<Entity, With<SpatialCameraBundle>>
) {
    let border_ent = commands.spawn((
        SpriteBundle {
            texture: asset_server.load("minimap_border.png"),
            transform: Transform {
                translation: MINIMAP_TRANSLATION,
                scale: Vec3::new(GAME_PROJ_SCALE, GAME_PROJ_SCALE, 1.),
                ..Default::default()
            },
            ..Default::default()
        },
        MinimapBorder,
    )).id();

    let minimap: Image = draw_minimap(map);
    let minimap_handle = assets.add(minimap);

    let minimap_ent = commands.spawn((
        SpriteBundle {
            texture: minimap_handle,
            transform: Transform {
                translation: Vec3 {
                    x: MINIMAP_TRANSLATION.x,
                    y: MINIMAP_TRANSLATION.y,
                    z: MINIMAP_TRANSLATION.z + 1.
                },
                scale: Vec3::new(GAME_PROJ_SCALE, GAME_PROJ_SCALE, 1.),
                ..Default::default()
            },
            ..Default::default()
        },
        Minimap,
    )).id();

    let marker_ent = commands.spawn((
        SpriteBundle {
            texture: asset_server.load("player_marker.png"),
            transform: Transform {
                translation: Vec3 {
                    x: MINIMAP_TRANSLATION.x,
                    y: MINIMAP_TRANSLATION.y,
                    z: MINIMAP_TRANSLATION.z + 2.
                },
                scale: Vec3::new(GAME_PROJ_SCALE, GAME_PROJ_SCALE, 1.),
                ..Default::default()
            },
            ..Default::default()
        },
        Marker,
    )).id();

    for parent in &mut cam_bundle {
        commands.entity(parent).add_child(border_ent);
        commands.entity(parent).add_child(minimap_ent);
        commands.entity(parent).add_child(marker_ent);
    }
}

fn draw_minimap(
    map: Res<WorldMap>,
) -> Image 
{
    let mut minimap_data: Vec<u8> = Vec::new();

    // Create data vec with 4 bytes per pixel from map data
    for row in 0..map::MAPSIZE {
        for col in 0..map::MAPSIZE {
            let tile = map.biome_map[row][col];
            let mut rgba: Vec<u8>;

            match tile {
                map::Biome::Free => {
                    rgba = vec![255,255,255,255];
                }
                map::Biome::Wall => {
                    //rgba = vec![55,59,94,255]; // FULL COLOR
                    rgba = vec![87,53,0,255]; // SEPIA
                }
                map::Biome::Ground => {
                    //rgba = vec![62,166,10,255]; // FULL COLOR
                    rgba = vec![182,136,66,255]; // SEPIA
                }
                map::Biome::Camp => {
                    //rgba = vec![71,109,40,255]; // FULL COLOR
                    rgba = vec![131,91,20,255]; // SEPIA
                }
            }
            minimap_data.append(&mut rgba);
        }
    }

    let minimap = Image::new(
        Extent3d{
            width: MINIMAP_DIMENSIONS.x,
            height: MINIMAP_DIMENSIONS.y,
            depth_or_array_layers: 1
        },
        TextureDimension::D2,
        minimap_data,
        TextureFormat::Rgba8UnormSrgb
    );

    return minimap;
}

fn configure_map(
    mut minimap: Query<&mut Transform, (With<Minimap>, Without<MinimapBorder>, Without<Marker>, Without<SpatialCameraBundle>, Without<LocalPlayer>)>,
    mut border: Query<&mut Transform, (With<MinimapBorder>, Without<Minimap>, Without<Marker>, Without<SpatialCameraBundle>, Without<LocalPlayer>)>,
    mut marker: Query<&mut Transform, (With<Marker>, Without<Minimap>, Without<MinimapBorder>, Without<SpatialCameraBundle>, Without<LocalPlayer>)>,
    camera: Query<&Transform, (With<SpatialCameraBundle>, Without<Minimap>, Without<MinimapBorder>, Without<Marker>, Without<LocalPlayer>)>,
    app_state: Res<State<AppState>>
) {
    // Set params based on current state
    let mut new_translation: Vec2 = Vec2::new(0., 0.);
    let mut new_scale: f32 = 1.;

    match app_state.get() {
        AppState::Game => {
            new_translation = Vec2::new(MINIMAP_TRANSLATION.x, MINIMAP_TRANSLATION.y);
            new_scale = GAME_PROJ_SCALE;
        }
        _ => { }
    }

    // Move minimap and border back to corner, show marker
    for mut minimap_tf in &mut minimap {
        minimap_tf.translation.x = new_translation.x;
        minimap_tf.translation.y = new_translation.y;
        minimap_tf.scale.x = new_scale;
        minimap_tf.scale.y = new_scale;
    }

    for mut border_tf in &mut border {
        border_tf.translation.x = new_translation.x;
        border_tf.translation.y = new_translation.y;
        border_tf.scale.x = new_scale;
        border_tf.scale.y = new_scale;
    }

    for mut marker_tf in &mut marker {
        for camera_tf in &camera {
            marker_tf.translation.x = camera_tf.translation.x / 16.;
            marker_tf.translation.y = camera_tf.translation.y / 16.;
            marker_tf.scale.x = new_scale;
            marker_tf.scale.y = new_scale;
        }
    }
}

fn respawn_update(
    mouse_button_inputs: Res<Input<MouseButton>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut app_state_next_state: ResMut<NextState<AppState>>,
    mut player: Query<(&mut Transform, &mut Health, &mut Visibility), With<LocalPlayer>>,
    map: Res<WorldMap>
) {
    // Get mouse position upon click
    if mouse_button_inputs.just_pressed(MouseButton::Left) {
        let window = window_query.get_single().unwrap();
        let cursor_position = window.cursor_position().unwrap();
        println!("{}", cursor_position);

        // Validate mouse position
        let mut cursor_to_map: UVec2 = UVec2::new(0, 0);

        // Ensure cursor within respawn map bounds
        if (cursor_position.x < ((super::WIN_W / 2.) - MINIMAP_DIMENSIONS.x as f32)) ||
            (cursor_position.x > ((super::WIN_W / 2.) + MINIMAP_DIMENSIONS.x as f32)) ||
            (cursor_position.y < ((super::WIN_H / 2.) - MINIMAP_DIMENSIONS.y as f32)) ||
            (cursor_position.y > ((super::WIN_H / 2.) + MINIMAP_DIMENSIONS.y as f32))
        {
            println!("invalid");
        } else {
            // Within bounds, convert to map tile coordinate
            cursor_to_map.x = ((cursor_position.x as u32 - ((super::WIN_W / 2.) as u32 - MINIMAP_DIMENSIONS.x)) / 2).clamp(0, (map::MAPSIZE - 1) as u32);
            cursor_to_map.y = ((cursor_position.y as u32 - ((super::WIN_H / 2.) as u32 - MINIMAP_DIMENSIONS.y)) / 2).clamp(0, (map::MAPSIZE - 1) as u32);

            println!("{}", cursor_to_map);

            // Check if coordinate is wall
            let tile = map.biome_map[cursor_to_map.y as usize][cursor_to_map.x as usize];

            match tile {
                map::Biome::Wall => {
                    println!("in wall");
                }
                _ => {
                    // Valid spawn tile
                    println!("valid");
                    app_state_next_state.set(AppState::Game);

                    let (mut tf, mut hp, mut vis) = player.single_mut();

                    hp.current = PLAYER_DEFAULT_HP;
                    hp.dead = false;
                    *vis = Visibility::Visible;
                    tf.translation.x = (cursor_to_map.x as f32 - 128.) * 16.;
                    tf.translation.y = -(cursor_to_map.y as f32 - 128.) * 16.;
                }
            }
        }
    }
}

fn game_update(
    player: Query<&Transform, (With<LocalPlayer>, Without<Marker>, Without<SpatialCameraBundle>)>,
    mut marker: Query<&mut Transform, (With<Marker>, Without<SpatialCameraBundle>, Without<LocalPlayer>)>,
    mut camera: Query<&mut Transform, (With<SpatialCameraBundle>, Without<Marker>, Without<LocalPlayer>)>
) {
    for player_tf in &player {
        for mut marker_tf in &mut marker {
            if player_tf.translation.x > -(((map::MAPSIZE / 2) * map::TILESIZE) as f32) && player_tf.translation.x < ((map::MAPSIZE / 2) * map::TILESIZE) as f32 {
                marker_tf.translation.x = MINIMAP_TRANSLATION.x + player_tf.translation.x / 32.
            }
            if player_tf.translation.y > -(((map::MAPSIZE / 2) * map::TILESIZE) as f32) && player_tf.translation.y < ((map::MAPSIZE / 2) * map::TILESIZE) as f32 {
                marker_tf.translation.y = MINIMAP_TRANSLATION.y + player_tf.translation.y / 32.
            }
        }
    
        for mut camera_tf in &mut camera {
            camera_tf.translation.x = player_tf.translation.x;
            camera_tf.translation.y = player_tf.translation.y;

            let clamp_pos_x: f32 = ((((map::MAPSIZE * map::TILESIZE) as isize)/2) - (((super::WIN_W * GAME_PROJ_SCALE) / 2.) as isize)) as f32;
            let clamp_pos_y: f32 = ((((map::MAPSIZE * map::TILESIZE) as isize)/2) - (((super::WIN_H * GAME_PROJ_SCALE) / 2.) as isize)) as f32;

            // Clamp camera view to map borders
            // Center camera in axis if map dimensions < window size
            if map::MAPSIZE * map::TILESIZE < super::WIN_W as usize {
                camera_tf.translation.x = 0.
            }
            else {
                if camera_tf.translation.x > clamp_pos_x {
                    camera_tf.translation.x = clamp_pos_x
                }
                if camera_tf.translation.x < -clamp_pos_x {
                    camera_tf.translation.x = -clamp_pos_x;
                }
            }

            if map::MAPSIZE * map::TILESIZE < super::WIN_H as usize {
                camera_tf.translation.y = 0.
            }
            else {
                if camera_tf.translation.y > clamp_pos_y {
                    camera_tf.translation.y = clamp_pos_y
                }
                if camera_tf.translation.y < -clamp_pos_y {
                    camera_tf.translation.y = -clamp_pos_y;
                }
            }
        }
    }
}