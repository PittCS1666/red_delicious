use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use bevy::window::PrimaryWindow;
use crate::AppState;
use crate::movement;
use crate::game::camp::setup_camps;
use crate::game::components::{Camp, CampStatus, Grade, Health, Player};
use crate::game::{player, player::{LocalPlayer, LocalPlayerDeathEvent, LocalPlayerSpawnEvent, PLAYER_DEFAULT_HP, MAX_PLAYERS}, PlayerId};
use crate::game::buffers::EventBuffer;
use crate::game::player::SpawnEvent;
use crate::map;
use crate::net::{IsHost, TickNum};

pub const GAME_PROJ_SCALE: f32 = 0.5;

const MINIMAP_DIMENSIONS: UVec2 = UVec2::new(map::MAPSIZE as u32, map::MAPSIZE as u32);
const MINIMAP_PAD: UVec2 = UVec2::new(32, 32); // How many pixels between top right of window and top right of minimap (not border)
const MINIMAP_TRANSLATION: Vec3 = Vec3::new(
    ((super::WIN_W / 2.) as u32 - MINIMAP_PAD.x - (MINIMAP_DIMENSIONS.x / 2)) as f32 * GAME_PROJ_SCALE,
    ((super::WIN_H / 2.) as u32 - MINIMAP_PAD.y - (MINIMAP_DIMENSIONS.y / 2)) as f32 * GAME_PROJ_SCALE,
    5.
);

const CAMP_MARKER_COLORS: [Color; 5] = [
    Color::Rgba{red: 0.2, green: 0.76, blue: 0.13, alpha: 1.}, // HP up
    Color::Rgba{red: 0.76, green: 0.13, blue: 0.13, alpha: 1.}, // Damage up
    Color::Rgba{red: 0.13, green: 0.27, blue: 0.76, alpha: 1.}, // Defense up
    Color::Rgba{red: 0.76, green: 0.13, blue: 0.66, alpha: 1.}, // Attack speed up
    Color::Rgba{red: 0.76, green: 0.6, blue: 0.13, alpha: 1.}  // Movement speed up
];

const ENEMY_PLAYER_COLOR: Color = Color::Rgba {red: 1., green: 0.2, blue: 0.2, alpha: 1.};

#[derive(Component)]
pub struct GameCamera;

#[derive(Component)]
pub struct LocalPlayerMarker;

#[derive(Component)]
pub struct EnemyPlayerMarker(pub u8);

#[derive(Component)]
pub struct CampMarker(pub u8);

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
            .add_systems(Update, spawn_update.run_if(player::local_player_dead))
            .add_systems(Update, marker_follow_local_player.run_if(not(player::local_player_dead)))
            .add_systems(OnEnter(AppState::Game), spawn_minimap.after(setup_camps))
            .add_systems(Update, configure_map_on_event)
            .add_systems(Update, spawn_camp_markers.run_if(any_with_component::<Camp>()))
            .add_systems(Update, hide_cleared_camp_markers.run_if(any_with_component::<CampMarker>()))
            .add_systems(Update, spawn_enemy_player_markers.run_if(any_with_component::<LocalPlayer>()))
            .add_systems(Update, show_enemy_player_markers.run_if(player::local_player_dead))
            .add_systems(Update, hide_enemy_player_markers.run_if(not(player::local_player_dead)))
            .add_systems(Update, show_hide_local_player_marker.run_if(any_with_component::<LocalPlayerMarker>()));
    }
}

fn startup(
    commands: Commands,
) {
    spawn_camera(commands);
}

// Spawns the main game camera as a child of a SpatialBundle that will follow the player in Game state
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

// Spawns the minimap, its border, and the player marker and parents the SpatialBundle to them
pub fn spawn_minimap(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut assets: ResMut<Assets<Image>>,
    map: Res<map::WorldMap>,
    mut game_camera: Query<Entity, With<SpatialCameraBundle>>,
) {
    let minimap_border_entity = commands.spawn((
        SpriteBundle {
            texture: asset_server.load("minimap_border.png"),
            transform: Transform {
                translation: Vec3 {
                    x: 0.,
                    y: 0.,
                    z: MINIMAP_TRANSLATION.z
                },
                ..Default::default()
            },
            ..Default::default()
        },
        MinimapBorder,
    )).id();

    for parent in &mut game_camera {
        commands.entity(parent).add_child(minimap_border_entity);
    }

    let minimap: Image = draw_minimap(map);
    let minimap_handle = assets.add(minimap);

    let minimap_entity = commands.spawn((
        SpriteBundle {
            texture: minimap_handle,
            transform: Transform {
                translation: Vec3 {
                    x: 0.,
                    y: 0.,
                    z: 1.
                },
                ..Default::default()
            },
            ..Default::default()
        },
        Minimap,
    )).id();

    commands.entity(minimap_border_entity).add_child(minimap_entity);
}

fn spawn_camp_markers(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut minimap: Query<Entity, With<Minimap>>,
    camp_markers: Query<Entity, With<CampMarker>>,
    camps: Query<(&Camp, &Transform, &Grade), With<Camp>>
) {
    // Return immediately if the camp markers already exist
    // TODO: Call this function once on a CampSpawnEvent to make this loop redundant
    for _marker in &camp_markers {
        return;
    }

    for parent in &mut minimap {
        for (camp_num, camp_pos, camp_grade) in camps.iter() {
            let camp_marker_ent = commands.spawn((
                SpriteBundle {
                    texture: asset_server.load("camp_marker.png"),
                    transform: Transform {
                        translation: Vec3 {
                            x: ((camp_pos.translation.x / map::TILESIZE as f32) as i32) as f32,
                            y: ((camp_pos.translation.y / map::TILESIZE as f32) as i32) as f32,
                            z: 2.
                        },
                        ..Default::default()
                    },
                    sprite: Sprite {
                        color: CAMP_MARKER_COLORS[(camp_grade.0 - 1) as usize],
                        ..Default::default()
                    },
                    ..Default::default()
                },
                CampMarker(camp_num.0),
            )).id();

            commands.entity(parent).add_child(camp_marker_ent);
        }
    }
}

fn spawn_enemy_player_markers(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut minimap: Query<Entity, With<Minimap>>,
    enemy_player_markers: Query<Entity, With<EnemyPlayerMarker>>,
    local_player_id: Res<PlayerId>
) {
    // Return immediately if the enemy player markers already exist
    for _marker in &enemy_player_markers {
        return;
    }

    for parent in &mut minimap {
        for player_id in 0..MAX_PLAYERS {
            if player_id as u8 != local_player_id.0 {
                let enemy_player_marker_ent = commands.spawn((
                    SpriteBundle {
                        texture: asset_server.load("player_marker.png"),
                        transform: Transform {
                            translation: Vec3 {
                                x: 0.,
                                y: 0.,
                                z: 2.
                            },
                            ..Default::default()
                        },
                        sprite: Sprite {
                            color: ENEMY_PLAYER_COLOR,
                            ..Default::default()
                        },
                        visibility: Visibility::Hidden,
                        ..Default::default()
                    },
                    EnemyPlayerMarker(player_id as u8),
                )).id();
        
                commands.entity(parent).add_child(enemy_player_marker_ent);
            }
        }
    }
}

// TODO: Tie this function to a camp cleared/camp spawned event instead of running on Update
fn hide_cleared_camp_markers(
    mut camp_markers: Query<(&CampMarker, &mut Visibility), With<CampMarker>>,
    camps: Query<(&Camp, &CampStatus), With<Camp>>,
    input: Res<Input<KeyCode>>,
    app_state_current_state: Res<State<AppState>>,
) {
    for (marker_num, mut marker_visibility) in &mut camp_markers {
        for (camp_num, camp_status) in &camps {
            if camp_num.0 == marker_num.0 {
                if !camp_status.0 || input.pressed(KeyCode::Tab) ||
                    *app_state_current_state.get() == AppState::GameOver {
                    *marker_visibility = Visibility::Hidden;
                }
                else {
                    *marker_visibility = Visibility::Visible;
                }
            }
        }
    }
}

fn show_enemy_player_markers(
    mut enemy_player_markers: Query<(&EnemyPlayerMarker, &mut Visibility, &mut Transform), With<EnemyPlayerMarker>>,
    players: Query<(&Player, &Transform, &Health), (With<Player>, Without<LocalPlayer>, Without<EnemyPlayerMarker>)>,
    input: Res<Input<KeyCode>>,
    app_state_current_state: Res<State<AppState>>,
) {
    for (marker_id, mut marker_visibility, mut marker_transform) in &mut enemy_player_markers {
        for (player_id, player_transform, player_health) in &players {
            if input.pressed(KeyCode::Tab) ||
                    *app_state_current_state.get() == AppState::GameOver {
                    *marker_visibility = Visibility::Hidden;
                }
                else {
                    if marker_id.0 == player_id.0 && !player_health.dead {
                        *marker_visibility = Visibility::Visible;
                        marker_transform.translation.x = make_position_not_float(player_transform.translation.x / map::TILESIZE as f32);
                        marker_transform.translation.y = make_position_not_float(player_transform.translation.y / map::TILESIZE as f32);
                    }
                }
        }
    }
}

fn hide_enemy_player_markers(
    mut enemy_player_markers: Query<&mut Visibility, With<EnemyPlayerMarker>>,
) {
    for mut marker_visibility in &mut enemy_player_markers {
        *marker_visibility = Visibility::Hidden;
    }
}

// Creates and returns the Image of the minimap from the map data
fn draw_minimap(
    map: Res<map::WorldMap>,
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
                map::Biome::Path => {
                    //rgba = vec![240,169,83,255]; // FULL COLOR
                    rgba = vec![241,213,166,255]; // SEPIA
                
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

// Adjusts minimap/border/marker position and size based on being in Game or Respawn state
fn configure_map_on_event(
    mut minimap_border: Query<&mut Transform, With<MinimapBorder>>,
    mut death_reader: EventReader<LocalPlayerDeathEvent>,
    mut spawn_reader: EventReader<LocalPlayerSpawnEvent>
) {
    let mut spawn_mode: Option<bool> = None;
    for _ in death_reader.iter() {
        spawn_mode = Some(true);
    }
    if spawn_mode.is_none() {
        for _ in spawn_reader.iter() {
            spawn_mode = Some(false);
        }
    }
    if spawn_mode.is_none() {
        return;
    }
    // minimap mode
    let mut new_translation: Vec2 = Vec2::new(MINIMAP_TRANSLATION.x, MINIMAP_TRANSLATION.y);
    let mut new_scale: f32 = GAME_PROJ_SCALE;

    if spawn_mode.unwrap() {
        new_translation = Vec2::new(0., 0.);
        new_scale = 1.;
    }

    // Set border translation/scale with aforementioned parameters
    for mut border_transform in &mut minimap_border {
        border_transform.translation.x = new_translation.x;
        border_transform.translation.y = new_translation.y;
        border_transform.scale.x = new_scale;
        border_transform.scale.y = new_scale;
    }
}

// Make marker reflect player position while player is alive
fn marker_follow_local_player(
    local_player: Query<&Transform, (With<LocalPlayer>, Without<LocalPlayerMarker>, Without<SpatialCameraBundle>)>,
    mut local_player_marker: Query<&mut Transform, (With<LocalPlayerMarker>, Without<SpatialCameraBundle>, Without<LocalPlayer>)>,
) {
    for local_player_transform in &local_player {
        // Set marker position on minimap to reflect the player's current position in the game world
        for mut marker_tf in &mut local_player_marker {
            if local_player_transform.translation.x > -(((map::MAPSIZE / 2) * map::TILESIZE) as f32) && local_player_transform.translation.x < ((map::MAPSIZE / 2) * map::TILESIZE) as f32 {
                marker_tf.translation.x = make_position_not_float(local_player_transform.translation.x / map::TILESIZE as f32);
            }
            if local_player_transform.translation.y > -(((map::MAPSIZE / 2) * map::TILESIZE) as f32) && local_player_transform.translation.y < ((map::MAPSIZE / 2) * map::TILESIZE) as f32 {
                marker_tf.translation.y = make_position_not_float(local_player_transform.translation.y / map::TILESIZE as f32);
            }
        }
    }
}

fn show_hide_local_player_marker(
    mut local_player_marker: Query<&mut Visibility, With<LocalPlayerMarker>>,
    input: Res<Input<KeyCode>>,
    app_state_current_state: Res<State<AppState>>,
) {
    for mut marker_visibility in &mut local_player_marker{
        if input.pressed(KeyCode::Tab) || *app_state_current_state.get() == AppState::GameOver {
            *marker_visibility = Visibility::Hidden;
        } else {
            *marker_visibility = Visibility::Visible;
        }
    }
}

fn make_position_not_float(position: f32) -> f32 {
    return position as i32 as f32;
}

fn spawn_update(
    mouse_button_inputs: Res<Input<MouseButton>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut local_player: Query<(&mut Transform, &mut Health, &mut EventBuffer, &mut Visibility), With<LocalPlayer>>,
    map: Res<map::WorldMap>,
    is_host: Res<IsHost>,
    minimap: Query<Entity, With<Minimap>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    local_player_marker: Query<Entity, With<LocalPlayerMarker>>,
    tick: Res<TickNum>,
    mut lp_spawn_writer: EventWriter<LocalPlayerSpawnEvent>,
    mut spawn_writer: EventWriter<SpawnEvent>,
) {
    if mouse_button_inputs.just_pressed(MouseButton::Left) {
        let window = window_query.get_single().unwrap();
        let cursor_position = window.cursor_position().unwrap();

        let mut cursor_to_map: UVec2 = UVec2::new(0, 0);
        if (cursor_position.x > ((super::WIN_W / 2.) - MINIMAP_DIMENSIONS.x as f32)) &&
            (cursor_position.x < ((super::WIN_W / 2.) + MINIMAP_DIMENSIONS.x as f32)) &&
            (cursor_position.y > ((super::WIN_H / 2.) - MINIMAP_DIMENSIONS.y as f32)) &&
            (cursor_position.y < ((super::WIN_H / 2.) + MINIMAP_DIMENSIONS.y as f32))
        {
            cursor_to_map.x = ((cursor_position.x as u32 - ((super::WIN_W / 2.) as u32 - MINIMAP_DIMENSIONS.x)) / 2).clamp(0, (map::MAPSIZE - 1) as u32);
            cursor_to_map.y = ((cursor_position.y as u32 - ((super::WIN_H / 2.) as u32 - MINIMAP_DIMENSIONS.y)) / 2).clamp(0, (map::MAPSIZE - 1) as u32);
            let tile = map.biome_map[cursor_to_map.y as usize][cursor_to_map.x as usize];
            if tile != map::Biome::Wall {
                let (mut lp_tf, mut lp_hp, mut lp_eb, mut lp_vis) = local_player.single_mut();

                let events = lp_eb.0.get(tick.0).clone();
                if events.is_none() {
                    lp_eb.0.set(tick.0, Some(player::SPAWN_BITFLAG));
                }
                else {
                    let events = events.unwrap();
                    lp_eb.0.set(tick.0, Some(events | player::SPAWN_BITFLAG));
                }
                if is_host.0 {
                    lp_spawn_writer.send(LocalPlayerSpawnEvent);
                    spawn_writer.send(SpawnEvent { id: 0 });
                }
                lp_tf.translation.x = (cursor_to_map.x as f32 - 128.) * 16.;
                lp_tf.translation.y = -(cursor_to_map.y as f32 - 128.) * 16.;

                // Spawn local player marker if necessary
                if local_player_marker.get_single().is_ok() { return }
                for minimap_entity in minimap.iter() {
                    let local_player_marker_entity = commands.spawn((
                        SpriteBundle {
                            texture: asset_server.load("player_marker.png"),
                            transform: Transform {
                                translation: Vec3 {
                                    x: 0.,
                                    y: 0.,
                                    z: 3.
                                },
                                ..Default::default()
                            },
                            ..Default::default()
                        },
                        LocalPlayerMarker,
                    )).id();

                    commands.entity(minimap_entity).add_child(local_player_marker_entity);
                }
            }
        }
    }
}

// Runs in Game state, makes SpatialCameraBundle follow player
fn game_update(
    local_player: Query<&Transform, (With<LocalPlayer>, Without<LocalPlayerMarker>, Without<SpatialCameraBundle>)>,
    mut game_camera: Query<&mut Transform, (With<SpatialCameraBundle>, Without<LocalPlayerMarker>, Without<LocalPlayer>)>
) {
    for local_player_transform in &local_player {
        // Make SpatialCameraBundle follow player
        for mut camera_transform in &mut game_camera {
            camera_transform.translation.x = local_player_transform.translation.x;
            camera_transform.translation.y = local_player_transform.translation.y;

            let clamp_pos_x: f32 = ((((map::MAPSIZE * map::TILESIZE) as isize)/2) - (((super::WIN_W * GAME_PROJ_SCALE) / 2.) as isize)) as f32;
            let clamp_pos_y: f32 = ((((map::MAPSIZE * map::TILESIZE) as isize)/2) - (((super::WIN_H * GAME_PROJ_SCALE) / 2.) as isize)) as f32;

            // Clamp camera view to map borders
            // Center camera in axis if map dimensions < window size
            if map::MAPSIZE * map::TILESIZE < super::WIN_W as usize {
                camera_transform.translation.x = 0.
            }
            else {
                if camera_transform.translation.x > clamp_pos_x {
                    camera_transform.translation.x = clamp_pos_x
                }
                if camera_transform.translation.x < -clamp_pos_x {
                    camera_transform.translation.x = -clamp_pos_x;
                }
            }

            if map::MAPSIZE * map::TILESIZE < super::WIN_H as usize {
                camera_transform.translation.y = 0.
            }
            else {
                if camera_transform.translation.y > clamp_pos_y {
                    camera_transform.translation.y = clamp_pos_y
                }
                if camera_transform.translation.y < -clamp_pos_y {
                    camera_transform.translation.y = -clamp_pos_y;
                }
            }
        }
    }
}