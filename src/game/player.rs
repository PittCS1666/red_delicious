use bevy::prelude::*;
use bevy::transform::commands;
use bevy::window::PrimaryWindow;
use crate::{enemy, net};
use crate::game::movement::*;
use crate::{Atlas, AppState};
use serde::{Deserialize, Serialize};
use crate::buffers::*;
use crate::game::components::*;
use crate::game::components;
use crate::game::enemy::LastAttacker;
use crate::net::IsHost;

pub const PLAYER_SPEED: f32 = 250.;
const PLAYER_DEFAULT_HP: u8 = 100;
pub const PLAYER_SIZE: Vec2 = Vec2 { x: 32., y: 32. };
pub const MAX_PLAYERS: usize = 4;
pub const SWORD_DAMAGE: u8 = 40;

//TODO public struct resource holding player count

/// sent by network module to disperse information from the host
#[derive(Event, Debug)]
pub struct PlayerTickEvent {
    pub seq_num: u16,
    pub id: u8,
    pub tick: PlayerTick
}

/// the information that the host needs to produce on each tick
#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub struct PlayerTick {
    pub pos: Vec2,
    pub hp: u8,
}

#[derive(Event, Debug)]
pub struct UserCmdEvent {
    pub seq_num: u16,
    pub id: u8,
    pub tick: UserCmd
}

/// the information that the client needs to produce on each tick
// TODO this should just have inputs rather than a pos
#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub struct UserCmd {
    pub pos: Vec2,
    pub dir: f32,
}

#[derive(Component)]
pub struct LocalPlayer;  // marks the player controlled by the local computer

#[derive(Component)]
pub struct PlayerWeapon;

#[derive(Component)]
pub struct HealthBar;

#[derive(Component)]
struct DespawnPlayerWeaponTimer(Timer);

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin{
    fn build(&self, app: &mut App){
        app.add_systems(FixedUpdate, fixed.before(enemy::fixed_move))
            .add_systems(Update,
                (spawn_weapon_on_click,
                despawn_after_timer,
                update_health_bar,
                scoreboard_system,
                handle_dead_player,
                grab_powerup,
                move_player.run_if(in_state(AppState::Game)),
                packet, usercmd))
            .add_systems(OnEnter(AppState::Game), spawn_players)
            .add_systems(OnExit(AppState::Game), despawn_players)
            .add_event::<PlayerTickEvent>()
            .add_event::<UserCmdEvent>();
    }
}


pub fn spawn_players(
    mut commands: Commands,
    entity_atlas: Res<Atlas>,
    asset_server: Res<AssetServer>,
    is_host: Res<IsHost>
) {
    for i in 0..MAX_PLAYERS {
        let pb = PosBuffer(CircularBuffer::new_from(Vec2::new(i as f32 * 100., i as f32 * 100.)));
        let pl = commands.spawn((
            Player(i as u8),
            pb,
            Health {
                current: PLAYER_DEFAULT_HP,
                max: PLAYER_DEFAULT_HP,
            },
            Score {
                current_score: 0,
            },
            SpriteSheetBundle {
                texture_atlas: entity_atlas.handle.clone(),
                sprite: TextureAtlasSprite { index: entity_atlas.coord_to_index(i as i32, 0), ..default()},
                transform: Transform::from_xyz(0., 0., 1.),
                ..default()
            },
            Collider(PLAYER_SIZE),
            StoredPowerUps {
                power_ups: [0; NUM_POWERUPS],
            },
        )).id();

        let health_bar = commands.spawn((
            SpriteBundle {
            texture: asset_server.load("healthbar.png").into(),
            transform: Transform {
                translation: Vec3::new(0., 24., 1.),
                ..Default::default()
            },
            ..Default::default()},
            HealthBar,
        )).id();

        commands.entity(pl).push_children(&[health_bar]);

        if i == 0 && is_host.0 {
            commands.entity(pl).insert(LocalPlayer);
        }
        if i == 1 && !is_host.0 {
            commands.entity(pl).insert(LocalPlayer);
        }
    }
}

// Despawn all players when exiting the game
pub fn despawn_players(mut commands: Commands, players: Query<Entity, With<Player>>) {
    for e in players.iter() {
        commands.entity(e).despawn();
    }
}

// Update the health bar of the player displayed during the game
pub fn update_health_bar(
    mut health_bar_query: Query<&mut Transform, With<HealthBar>>,
    mut player_health_query: Query<(&mut Health, &Children, &StoredPowerUps), With<Player>>,
) {
    for (mut health, children, player_power_ups) in player_health_query.iter_mut() {
        health.max = PLAYER_DEFAULT_HP + player_power_ups.power_ups[PowerUpType::MaxHPUp as usize] * MAX_HP_UP;
        for child in children.iter() {
            let tf = health_bar_query.get_mut(*child);
            if let Ok(mut tf) = tf {
                tf.scale = Vec3::new((health.current as f32) / (health.max as f32), 1.0, 1.0);
            }
        }
    }
}

// Update the score displayed during the game
pub fn scoreboard_system(
    player_score_query: Query<&Score, With<LocalPlayer>>,
    mut score_query: Query<&mut Text, With<ScoreDisplay>>,
) {
    for mut text in score_query.iter_mut() {
        let player = player_score_query.single();
        text.sections[0].value = format!("Score: {}", player.current_score);
    }
}

// If player hp <= 0, reset player position and subtract 1 from player score if possible
pub fn handle_dead_player(
    mut player_query: Query<(&mut Transform, &mut Health, &StoredPowerUps), (With<Player>, Without<Enemy>)>,
    mut score_query: Query<&mut Score, (With<Player>, Without<Enemy>)>,
) {
    for (mut tf, mut health, player_power_ups) in player_query.iter_mut() {
        if health.current <= 0 {
            for mut player in score_query.iter_mut() {
                if (player.current_score.checked_sub(1)).is_some() {
                    player.current_score -= 1;
                } else {
                    player.current_score = 0;
                }
            }
            let translation = Vec3::new(0.0, 0.0, 1.0);
            tf.translation = translation;
            health.current = PLAYER_DEFAULT_HP + player_power_ups.power_ups[PowerUpType::MaxHPUp as usize] * MAX_HP_UP;
        }
    }
}

// if the player collides with a powerup, add it to the player's powerup list
pub fn grab_powerup(
    mut commands: Commands,
    mut player_query: Query<(&Transform, &mut Health, &mut StoredPowerUps), With<Player>>,
    powerup_query: Query<(Entity, &Transform, &PowerUp), With<PowerUp>>,
) {
    for (player_transform, mut player_health, mut player_power_ups) in player_query.iter_mut() {
        for (powerup_entity, powerup_transform, power_up) in powerup_query.iter() {
            // check detection
            let player_pos = player_transform.translation.truncate();
            let powerup_pos = powerup_transform.translation.truncate();
            if player_pos.distance(powerup_pos) < 16. {
                print!("grabbed powerup\n");
                // add powerup to player
                // player_power_ups.power_ups[power_up.0 as usize] += 1; // THIS DOES NOT WORK! I have no idea why
                match power_up.0
                {
                    components::PowerUpType::DamageDealtUp => {
                        player_power_ups.power_ups[PowerUpType::DamageDealtUp as usize] += 1;
                    },
                    components::PowerUpType::DamageReductionUp => {
                        player_power_ups.power_ups[PowerUpType::DamageReductionUp as usize] += 1;
                    },
                    components::PowerUpType::MaxHPUp => {
                        player_power_ups.power_ups[PowerUpType::MaxHPUp as usize] += 1;
                        player_health.current += MAX_HP_UP;
                    },
                    components::PowerUpType::AttackSpeedUp => {
                        player_power_ups.power_ups[PowerUpType::AttackSpeedUp as usize] += 1;
                        // TODO: add attack speed change somewhere
                    },
                    components::PowerUpType::MovementSpeedUp => {
                        player_power_ups.power_ups[PowerUpType::MovementSpeedUp as usize] += 1;
                    },
                }
                print!("{:?}\n", player_power_ups.power_ups);
                // despawn powerup
                commands.entity(powerup_entity).despawn();
            }
        }
    }
}

// Spawn a sword on the player's position when left mouse button is clicked
pub fn spawn_weapon_on_click(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mouse_button_inputs: Res<Input<MouseButton>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    player_query: Query<(Entity, &Transform, &Player, &StoredPowerUps), With<LocalPlayer>>,
    mut enemy_query: Query<(&Transform, &Collider, &mut Health, &mut LastAttacker), With<Enemy>>,
) {

    if !mouse_button_inputs.just_pressed(MouseButton::Left) {
        return;
    }
    let window = window_query.get_single().unwrap();
    for (player_entity, player_transform, player_id, player_power_ups) in player_query.iter() {
        let window_size = Vec2::new(window.width(), window.height());
        let cursor_position = window.cursor_position().unwrap();
        let cursor_position_in_world = Vec2::new(cursor_position.x, window_size.y - cursor_position.y) - window_size * 0.5;
    
        let direction_vector = cursor_position_in_world.normalize();
        let weapon_direction = direction_vector.y.atan2(direction_vector.x);

        let circle_radius = 50.0;// position spawning the sword, make it variable later
        let offset_x = circle_radius * weapon_direction.cos();
        let offset_y = circle_radius * weapon_direction.sin();
        let offset = Vec2::new(offset_x, offset_y);
            
        commands.entity(player_entity).with_children(|parent| {
            parent.spawn(SpriteBundle {
                texture: asset_server.load("sword01.png").into(),
                transform: Transform {
                    translation: Vec3::new(offset.x, offset.y, 5.0),
                    rotation: Quat::from_rotation_z(weapon_direction),
                    ..Default::default()
                },
                ..Default::default()
            }).insert(PlayerWeapon).insert(DespawnPlayerWeaponTimer(Timer::from_seconds(1.0, TimerMode::Once)));
        });

        let (start, end) = attack_line_trace(player_transform, offset);
        for (enemy_transform, collider, mut health, mut last_attacker) in enemy_query.iter_mut() {
            if line_intersects_aabb(start, end, enemy_transform.translation.truncate(), collider.0) {
                last_attacker.0 = Some(player_id.0);
                match health.current.checked_sub(SWORD_DAMAGE + player_power_ups.power_ups[PowerUpType::DamageDealtUp as usize] * DAMAGE_DEALT_UP) {
                    Some(v) => {
                        health.current = v;
                    }
                    None => {
                        health.current = 0;
                    }
                }
            }
        }
    }
}

fn despawn_after_timer(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut DespawnPlayerWeaponTimer)>,
) {
    for (entity, mut despawn_timer) in query.iter_mut() {
        despawn_timer.0.tick(time.delta());
        if despawn_timer.0.finished() {
            commands.entity(entity).despawn();
        }
    }
}

fn attack_line_trace(player_transform: &Transform, weapon_offset: Vec2) -> (Vec2, Vec2) {
    let start = player_transform.translation.truncate();
    let end = start + weapon_offset;
    (start, end)
}

fn line_intersects_aabb(start: Vec2, end: Vec2, box_center: Vec2, box_size: Vec2) -> bool {
    let dir = (end - start).normalize();

    let t1 = (box_center.x - box_size.x / 2.0 - start.x) / dir.x;
    let t2 = (box_center.x + box_size.x / 2.0 - start.x) / dir.x;
    let t3 = (box_center.y - box_size.y / 2.0 - start.y) / dir.y;
    let t4 = (box_center.y + box_size.y / 2.0 - start.y) / dir.y;

    let tmin = t1.min(t2).max(t3.min(t4));
    let tmax = t1.max(t2).min(t3.max(t4));

    if tmax < 0.0 || tmin > tmax {
        return false;
    }

    let t = if tmin < 0.0 { tmax } else { tmin };
    return t > 0.0 && t * t < (end - start).length_squared();
}


pub fn fixed(
        tick: Res<net::TickNum>,
        mut players: Query<(&mut PosBuffer, &Transform), With<LocalPlayer>>,
    ) {
    for ( mut player_pos_buffer, current_pos) in &mut players {
        // pull current position into PositionBuffer
        player_pos_buffer.0.set(tick.0, Vec2::new(current_pos.translation.x, current_pos.translation.y));
    }
}

pub fn packet(
    mut player_reader: EventReader<PlayerTickEvent>,
    mut player_query: Query<(&Player, &mut PosBuffer)>
) {
    //TODO if you receive info that your predicted local position is wrong, it needs to be corrected
    for ev in player_reader.iter() {
        // TODO this is slow but i have no idea how to make the borrow checker okay
        //   with the idea of an array of player PosBuffer references
        for (pl, mut pb) in &mut player_query {
            if pl.0 == ev.id {
                pb.0.set(ev.seq_num, ev.tick.pos);
            }
        }
    }
}

pub fn usercmd(
    mut usercmd_reader: EventReader<UserCmdEvent>,
    mut player_query: Query<(&Player, &mut PosBuffer)>
) {
    // TODO in the future usercmds are just inputs, so here is where movement would be calculated
    //   ideally using the same function that clients use for local prediction
    for ev in usercmd_reader.iter() {
        // TODO this is slow but i have no idea how to make the borrow checker okay
        //   with the idea of an array of player PosBuffer references
        for (pl, mut pb) in &mut player_query {
            if pl.0 == ev.id {
                pb.0.set(ev.seq_num, ev.tick.pos);
            }
        }
    }
}
