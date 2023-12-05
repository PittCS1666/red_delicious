use std::time::Duration;
use bevy::prelude::*;
use crate::{enemy, net};
use crate::game::movement::*;
use crate::{Atlas, AppState};
use crate::buffers::*;
use crate::game::components::*;
use crate::game::enemy::LastAttacker;
use crate::game::PlayerId;
use crate::net::{is_client, is_host, TICKLEN_S, TickNum};
use crate::net::packets::{PlayerTickEvent, UserCmdEvent};
use crate::menus::layout::{toggle_leaderboard, update_leaderboard};
use bevy::audio::VolumeLevel;

pub const PLAYER_SPEED: f32 = 250.;
pub const PLAYER_DEFAULT_HP: u8 = 100;
pub const PLAYER_DEFAULT_DEF: f32 = 1.;
pub const PLAYER_SIZE: Vec2 = Vec2 { x: 32., y: 32. };
pub const MAX_PLAYERS: usize = 4;
pub const SWORD_DAMAGE: u8 = 40;
const DEFAULT_COOLDOWN: f32 = 0.8;
pub const ATTACK_BITFLAG: u8 = 1;
pub const SPAWN_BITFLAG: u8 = 2;

#[derive(Event)]
pub struct SetIdEvent(pub u8);

#[derive(Event)]
pub struct AttackEvent {
    pub seq_num: u16,
    pub id: u8,
}

#[derive(Event)]
pub struct SpawnEvent {
    pub id: u8
}

#[derive(Event)]
pub struct LocalPlayerDeathEvent;

#[derive(Event)]
pub struct LocalPlayerSpawnEvent;

/// Marks the player controlled by the local computer
#[derive(Component)]
pub struct LocalPlayer;

#[derive(Component)]
pub struct PlayerWeapon;

#[derive(Component)]
pub struct SwordAnimation{
    pub current: f32,
    pub cursor_vector: Vec2,
    pub max: f32,
}

#[derive(Component)]
pub struct Cooldown(pub Timer);

#[derive(Component)]
pub struct HealthBar;

#[derive(Component)]
pub struct Shield;

#[derive(Component)]
pub struct PlayerShield {
    pub active: bool,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin{
    fn build(&self, app: &mut App){
        app.add_systems(FixedUpdate, update_buffer.before(net::client::fixed).before(enemy::fixed_move))
            .add_systems(Update, (
                handle_usercmd_events,
                ).run_if(in_state(AppState::Game)).run_if(is_host).before(net::host::fixed))
            .add_systems(Update, (
                attack_input,
                animate_sword,
                handle_move,
                handle_player_ticks.run_if(is_client),
                ).run_if(in_state(AppState::Game)))
            .add_systems(FixedUpdate, (
                attack_host.before(attack_simulate),
                attack_simulate,
                spawn_simulate,
            ).run_if(in_state(AppState::Game)).run_if(is_host).before(net::host::fixed))
            .add_systems(FixedUpdate, (
                attack_draw.after(attack_simulate),
                health_simulate.after(spawn_simulate),
                health_draw.after(health_simulate),
                ).before(net::client::fixed).before(net::host::fixed))
            .add_systems(Update, handle_id_events.run_if(is_client).run_if(in_state(AppState::Connecting)))
            .add_systems(OnEnter(AppState::Game), (spawn_players, reset_cooldowns))
            .add_systems(OnEnter(AppState::GameOver), remove_players.after(toggle_leaderboard).after(update_leaderboard))
            .add_event::<SetIdEvent>()
            .add_event::<AttackEvent>()
            .init_resource::<Events<SpawnEvent>>()
            .add_event::<PlayerTickEvent>()
            .add_event::<UserCmdEvent>()
            .add_event::<LocalPlayerDeathEvent>()
            .add_event::<LocalPlayerSpawnEvent>();
    }
}

pub fn reset_cooldowns(mut query: Query<&mut Cooldown, With<Player>>) {
    for mut c in &mut query {
        c.0.tick(Duration::from_secs_f32(100.));
    }
}


/*

client::fixed
  send ClientTick to host

client::update
  receive HostTick and send events to other systems to fill out info

host::fixed
  send HostTick culled for each guy

host::update
  receive ClientTick and send events to other systems to fill out info

DONE
attack_input (update, all)
  if left clicking and cooldown is up, set event.attack to true

attack_simulate (on attack event, host)
  go through all of the players, collecting their powerups, mut Stats, PosBuffer, DirBuffer, EventBuffer, mut HpBuffer, option lp
  if EventBuffer says it's not attacking, continue
  go through all enemies, collecting their PosBuffer, mut HpBuffer
  if attacker, tick = seq_num, else tick = seq_num - net::DELAY
  do collision checks for each player that is attacking on enemies (using PosBuffer and DirBuffer of player, PosBuffer of enemy)
  enemies that are hit take damage depending on player powerups, (using powerups of player, HpBuffer of enemy)
  HpBuffer is updated at true current tick
  go through all of the players
  if same player as this one, continue
  if player is shielding on this tick, continue
  do collision checks using PosBuffer and DirBuffer of attacker, PosBuffer of victim
  if hit, set HpBuffer of victim for true current tick
  if killed, set Stats of attacker and victim
  go through all of the chests
  if chest hp is 0, continue
  do collision checks using PosBuffer and DirBuffer of attacker, position of chest
  if hit, set hp of chest to zero and bust it open

attack_host (fixed, host)
  if attack is true, make an attack event for yourself

attack_draw (fixed, all)
  go through all of the players, collecting their PosBuffer, DirBuffer, EventBuffer, option lp
  if lp, delay = 0, else delay = net::DELAY
  if attacking this tick, draw their attack

powerup_simulate (fixed, host)
  go through all of the powerups, go through all the players, if they are in the same place remove powerup and add to player powerups

shield_input (update, all)
  if right clicking, set event.shield to true

to check if shield is active, check if this tick's shield event is true

shield_draw (fixed, all)
  for every player, if event.shield is true for this tick, show their shield

spawn_input (update if dead, all)
  if left clicking and valid position, set event.spawn to true and move player there

handle_host_tick (on host tick event, client)
  for each player, mark it dead and invisible
  for players in tick, if alive, mark not dead and not invisible, fill PosBuffer, HpBuffer, DirBuffer, EventBuffer, Stats, Powerups for each player
  for players, find localplayer, check if dead and if Res<MapState> == MAP_MINI, then send LocalPlayerDeathEvent
                                 check if not dead and if Res<MapState> == MAP_SPAWN, then send LocalPlayerSpawnEvent
  for each enemy, mark it dead and invisible
  for enemies in tick, if alive, mark not dead and not invisible, fill PosBuffer, HpBuffer, EventBuffer
  remove all powerups
  for powerups in tick, spawn a powerup on location
  remove all chests
  for chests in tick, spawn a chest on location, if hp 0 then empty, otherwise full
  redraw score counter, powerup ui, health bars

handle_client_tick (on client tick event, host)
  fill player PosBuffer, DirBuffer, EventBuffer
  if player sent an attack, make an attack event and send it

UserCmd
    pos
    dir
    events

PlayerTick
    pos
    hp
    dir
    events
    stats
    powerups

EnemyTick
    pos
    hp
    special
    events

HostTick
    PlayerTicks
    ** all below are culled by visibility **
    EnemyTicks
    Chests (hp, Vec2)
    Powerups (type, Vec2)


 */

pub fn spawn_players(
    mut commands: Commands,
    entity_atlas: Res<Atlas>,
    asset_server: Res<AssetServer>,
    res_id: Res<PlayerId>
) {
    for i in 0..MAX_PLAYERS {
        let pl;
        pl = commands.spawn((
            Player(i as u8),
            PosBuffer(CircularBuffer::new()),
            DirBuffer(CircularBuffer::new()),
            EventBuffer(CircularBuffer::new()),
            HpBuffer(CircularBuffer::new()),
            Stats {
                score: 0,
                enemies_killed: 0,
                players_killed: 0,
                camps_captured: 0,
                deaths: 0,
                kd_ratio: 0.
            },
            Health {
                current: 0,
                max: PLAYER_DEFAULT_HP,
                dead: true
            },
            SpriteSheetBundle {
                texture_atlas: entity_atlas.handle.clone(),
                sprite: TextureAtlasSprite { index: entity_atlas.coord_to_index(i as i32, 0), ..default()},
                visibility: Visibility::Hidden,
                transform: Transform::from_xyz(0., 0., 1.),
                ..default()
            },
            Collider(PLAYER_SIZE),
            Cooldown(Timer::from_seconds(DEFAULT_COOLDOWN, TimerMode::Once)),
            StoredPowerUps {
                power_ups: [0; NUM_POWERUPS],
            },
            PlayerShield {
                active: false,
            },
        )).id();

        if i as u8 == res_id.0 {
            commands.entity(pl).insert(LocalPlayer);
        }

        let health_bar = commands.spawn((
            SpriteBundle {
                texture: asset_server.load("healthbar.png"),
                transform: Transform {
                    translation: Vec3::new(0., 24., 2.),
                    ..Default::default()
                },
                ..Default::default()},
            HealthBar,
        )).id();

        commands.entity(pl).add_child(health_bar);
    }
}

pub fn remove_players(
    mut commands: Commands,
    players: Query<Entity, With<Player>>,
) {
    for e in players.iter() {
        commands.entity(e).despawn_recursive();
    }
}

/*
// Update the health bar child of player entity to reflect current hp
pub fn update_health_bars(
    mut health_bar_query: Query<&mut Transform, With<HealthBar>>,
    mut player_health_query: Query<(&mut Health, &Children), With<Player>>,
) {
    for (mut health, children) in player_health_query.iter_mut() {
        health.max = PLAYER_DEFAULT_HP;
        for child in children.iter() {
            let tf = health_bar_query.get_mut(*child);
            if let Ok(mut tf) = tf {
                tf.scale = Vec3::new((health.current as f32) / (health.max as f32), 1.0, 1.0);
            }
        }
    }
}

// Update the score displayed during the game
pub fn update_score(
    players: Query<(&Health, &Stats), With<LocalPlayer>>,
    mut score_displays: Query<&mut Text, With<ScoreDisplay>>,
    mut powerup_displays: Query<(&mut Text, &PowerupDisplayText), (With<PowerupDisplayText>, Without<ScoreDisplay>)>,
) {
    let score_display = score_displays.get_single_mut();
    let lp = players.get_single();
    if score_display.is_err() || lp.is_err() { return }
    let mut text = score_display.unwrap();
    let (hp, stats) = lp.unwrap();
    text.sections[0].value = format!("Score: {}", stats.score);
    for (mut powerup, index) in &mut powerup_displays {
        if index.0 == 2 {
            powerup.sections[0].value = format!("{}%", 100. * hp.current as f32 / PLAYER_DEFAULT_HP as f32);
        }
    }
}

pub fn handle_life_and_death(
    mut players: Query<(&mut Health, &mut Visibility, Option<&LocalPlayer>, &mut Stats)>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut death_writer: EventWriter<LocalPlayerDeathEvent>,
    mut spawn_writer: EventWriter<LocalPlayerSpawnEvent>,
) {
    for (mut health, mut vis, lp, mut stats) in players.iter_mut() {
        if health.current <= 0 && !health.dead {
            commands.spawn(AudioBundle {
                source: asset_server.load("dead-2.ogg"),
                ..default()
            });
            health.dead = true;
            *vis = Visibility::Hidden;
            if lp.is_some() {
                death_writer.send(LocalPlayerDeathEvent);
            }
            stats.deaths = stats.deaths.saturating_add(1);
            if stats.deaths != 0 {
                stats.kd_ratio = stats.players_killed as f32 / stats.deaths as f32;
            } 
            else {
                stats.kd_ratio = stats.players_killed as f32;
            }
        }
        else if health.current > 0 && health.dead {
            health.dead = false;
            if lp.is_some() {
                spawn_writer.send(LocalPlayerSpawnEvent);
            }
            *vis = Visibility::Visible;
        }
    }
}

// if the player collides with a powerup, add it to the player's powerup list and despawn the powerup entity
pub fn grab_powerup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut player_query: Query<(&Transform, &mut Health, &mut Cooldown, &mut StoredPowerUps), With<Player>>,
    powerup_query: Query<(Entity, &Transform, &PowerUp), With<PowerUp>>,
    mut powerup_displays: Query<(&mut Text, &PowerupDisplayText), With<PowerupDisplayText>>,
) {
    for (player_transform, mut player_health, mut cooldown, mut player_power_ups) in player_query.iter_mut() {
        for (powerup_entity, powerup_transform, power_up) in powerup_query.iter() {
            // check detection
            let player_pos = player_transform.translation.truncate();
            let powerup_pos = powerup_transform.translation.truncate();
            if player_pos.distance(powerup_pos) < 16. {
                // add powerup to player
                player_power_ups.power_ups[power_up.0 as usize] = player_power_ups.power_ups[power_up.0 as usize].saturating_add(1);
                for (mut powerup, index) in &mut powerup_displays {
                    if power_up.0 == PowerUpType::DamageDealtUp && index.0 == 0 {
                        powerup.sections[0].value = format!("{:.2}x",
                        (SWORD_DAMAGE as f32 + player_power_ups.power_ups[PowerUpType::DamageDealtUp as usize] as f32 * DAMAGE_DEALT_UP as f32)
                        / SWORD_DAMAGE as f32);
                    }
                    else if power_up.0 == PowerUpType::DamageReductionUp && index.0 == 1 {
                        // Defense multiplier determined by DAMAGE_REDUCTION_UP ^ n, where n is stacks of damage reduction
                        powerup.sections[0].value = format!("{:.2}x", 
                        (PLAYER_DEFAULT_DEF
                        / (PLAYER_DEFAULT_DEF * DAMAGE_REDUCTION_UP.powf(player_power_ups.power_ups[PowerUpType::DamageReductionUp as usize] as f32))));
                    }
                    else if power_up.0 == PowerUpType::Meat && index.0 == 2 {
                        player_health.current = player_health.current.saturating_add(MEAT_VALUE);
                    }
                    else if power_up.0 == PowerUpType::AttackSpeedUp && index.0 == 3 {
                        let updated_duration = cooldown.0.duration().mul_f32(1. / ATTACK_SPEED_UP);
                        cooldown.0.set_duration(updated_duration);
                        powerup.sections[0].value = format!("{:.2}x",
                        (DEFAULT_COOLDOWN
                        / (cooldown.0.duration().as_millis() as f32 / 1000.)));
                    }
                    else if power_up.0 == PowerUpType::MovementSpeedUp && index.0 == 4 {
                        powerup.sections[0].value = format!("{:.2}x",
                        (PLAYER_SPEED + (player_power_ups.power_ups[PowerUpType::MovementSpeedUp as usize] as f32 * MOVEMENT_SPEED_UP as f32))
                        / PLAYER_SPEED);
                    }
                }
                commands.spawn(AudioBundle {
                    source: asset_server.load("powerup.ogg"),
                    ..default()
                });
                // despawn powerup
                commands.entity(powerup_entity).despawn();
            }
        }
    }
}
 */

pub fn attack_input(
    time: Res<Time>,
    tick: Res<TickNum>,
    mouse_button_inputs: Res<Input<MouseButton>>,
    mut players: Query<(&mut Cooldown, &mut EventBuffer), With<LocalPlayer>>,
) {
    let player = players.get_single_mut();
    if player.is_err() { return }
    let (mut c, mut eb) = player.unwrap();
    c.0.tick(time.delta());
    if !(mouse_button_inputs.pressed(MouseButton::Left) && c.0.finished()) {
        return;
    }
    let events = eb.0.get(tick.0).clone();
    if events.is_none() {
        eb.0.set(tick.0, Some(ATTACK_BITFLAG));
    }
    else {
        let events = events.unwrap();
        eb.0.set(tick.0, Some(events | ATTACK_BITFLAG));
    }
    c.0.reset();
}

pub fn attack_host(
    players: Query<&EventBuffer, With<LocalPlayer>>,
    tick: Res<TickNum>,
    mut attack_writer: EventWriter<AttackEvent>
) {
    let player = players.get_single();
    if player.is_err() { return }
    let eb = player.unwrap();
    let events = eb.0.get(tick.0);
    if events.is_none() { return }
    if events.unwrap() & ATTACK_BITFLAG != 0 {
        attack_writer.send(AttackEvent {
            seq_num: tick.0,
            id: 0
        });
    }
}

pub fn attack_draw(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    tick: Res<TickNum>,
    players: Query<(Entity, &EventBuffer, &DirBuffer, Option<&LocalPlayer>)>,
) {
    for (e, eb, db, lp) in &players {
        let tick = if lp.is_some() { tick.0 } else { tick.0.saturating_sub(net::DELAY) };
        let events = eb.0.get(tick);
        if events.is_none() { continue }
        if events.unwrap() & ATTACK_BITFLAG != 0 {
            let dir = db.0.get(tick);
            if dir.is_none() { println!(" how bro"); continue }
            let dir = dir.unwrap();
            let cursor_vector = Vec2 { x: dir.cos(), y: dir.sin() };
            commands.spawn(AudioBundle {
                source: asset_server.load("player-swing.ogg"),
                settings: PlaybackSettings { volume: bevy::audio::Volume::Absolute(VolumeLevel::new(0.5)), ..default()}
            });
            commands.entity(e).with_children(|parent| {
                parent.spawn((
                    SpriteBundle {
                        texture: asset_server.load("sword01.png").into(),
                        visibility: Visibility::Hidden,
                        ..Default::default()
                    },
                    PlayerWeapon,
                    SwordAnimation {
                        current: 0.0,
                        cursor_vector,
                        max: TICKLEN_S,
                    })
                );
            });
        }
    }
}

pub fn attack_simulate(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    tick: Res<TickNum>,
    mut attack_reader: EventReader<AttackEvent>,
    players: Query<(&Player, &Transform, &DirBuffer, &StoredPowerUps)>,
    mut enemies: Query<(&Transform, &mut HpBuffer, &mut LastAttacker), With<Enemy>>,
    mut chest: Query<(&Transform, &mut Health), (With<ItemChest>, Without<Enemy>)>,
) {
    for ev in &mut attack_reader {
        for (pl, tf, db, spu) in &players {
            if pl.0 != ev.id { continue }
            let sword_angle = db.0.get(ev.seq_num);
            if sword_angle.is_none() { println!("why is this broken?"); continue }
            let sword_angle = sword_angle.unwrap();
            let player_pos = tf.translation.truncate();
            for (enemy_tf, mut enemy_hb, mut last_attacker) in enemies.iter_mut() {
                let enemy_pos = enemy_tf.translation.truncate();
                if player_pos.distance(enemy_pos) > 32.0 + 50.0 { continue; } // enemy too far

                let combat_angle = (enemy_pos - player_pos).y.atan2((enemy_pos - player_pos).x);
                let angle_diff = sword_angle - combat_angle;
                if angle_diff.abs() > std::f32::consts::PI * 0.375 { continue; } // enemy not in sector

                last_attacker.0 = Some(pl.0);
                let damage = SWORD_DAMAGE.saturating_add(spu.power_ups[PowerUpType::DamageDealtUp as usize].saturating_mul(DAMAGE_DEALT_UP));
                let hp = enemy_hb.0.get(tick.0).unwrap();
                enemy_hb.0.set(tick.0, Some(hp.saturating_sub(damage)));
                if pl.0 == 1 {
                    println!("LMFAO");
                }
                commands.spawn(AudioBundle {
                    source: asset_server.load("hitHurt.ogg"),
                    settings: PlaybackSettings { volume: bevy::audio::Volume::Absolute(VolumeLevel::new(0.5)), ..default()}
                });
            }
            for (chest_tf, mut chest_hp) in chest.iter_mut() {
                let chest_pos = chest_tf.translation.truncate();
                if player_pos.distance(chest_pos) > 32.0 + 50.0 { continue; } // chest too far

                let combat_angle = (chest_pos - player_pos).y.atan2((chest_pos - player_pos).x);
                let angle_diff = sword_angle - combat_angle;
                if angle_diff.abs() > std::f32::consts::PI * 0.375 { continue; } // chest not in sector

                if chest_hp.current != 0 {
                    chest_hp.current = 0;
                    commands.spawn(AudioBundle {
                        source: asset_server.load("chest.ogg"),
                        settings: PlaybackSettings { volume: bevy::audio::Volume::Absolute(VolumeLevel::new(0.5)), ..default()}
                    });
                }
            }
        }
    }
}

pub fn animate_sword(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut Visibility, &mut SwordAnimation), With<PlayerWeapon>>,
) {
    for (mut tf, mut vis, mut anim) in query.iter_mut() {
        let attack_radius = 50.0;
        let current_step = anim.current / anim.max;

        let cursor_angle = anim.cursor_vector.y.atan2(anim.cursor_vector.x);
        let sword_translation_angle;
        if anim.cursor_vector.x > 0.0 {
            sword_translation_angle = current_step * std::f32::consts::PI * 0.75 - std::f32::consts::PI * 0.375 - cursor_angle;
        } else {
            sword_translation_angle = current_step * std::f32::consts::PI * 0.75 - std::f32::consts::PI * 0.375 + cursor_angle;
        }
        let sword_rotation_vector = Vec3::new(sword_translation_angle.cos(), sword_translation_angle.sin(), 0.0);
        let sword_rotation_angle = sword_rotation_vector.y.atan2(sword_rotation_vector.x);

        tf.translation.x = sword_translation_angle.cos() * attack_radius;
        if anim.cursor_vector.x > 0.0 {
            tf.rotation = Quat::from_rotation_z(-1.0 * sword_rotation_angle);
            tf.translation.y = -1.0 * sword_translation_angle.sin() * attack_radius;
        } else {
            tf.rotation = Quat::from_rotation_z(sword_rotation_angle);
            tf.translation.y = sword_translation_angle.sin() * attack_radius;
            tf.scale.y = -1.0;
        }
        if anim.current == 0.0 {
            *vis = Visibility::Visible;
        }
        anim.current += time.delta_seconds();
        if anim.current >= anim.max {
            *vis = Visibility::Hidden;
        }
    }
}

/*
pub fn spawn_shield_on_right_click(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mouse_button_inputs: Res<Input<MouseButton>>,
    mut query: Query<(Entity, &Transform, &mut PlayerShield), With<LocalPlayer>>,
) {
    if mouse_button_inputs.just_pressed(MouseButton::Right) {
        let shield_texture_handle = asset_server.load("shield01.png"); //where to replace the shield image

        for (player_entity, _player_transform, mut shield) in query.iter_mut() {
            shield.active = true;
            commands.entity(player_entity).with_children(|parent| {
                parent.spawn(SpriteBundle {
                    texture: shield_texture_handle.clone(),
                    transform: Transform {
                        translation: Vec3::new(0.0, 0.0, 0.5),
                        ..Default::default()
                    },
                    ..Default::default()
                }).insert(Shield);
            });
            commands.spawn(AudioBundle {
                source: asset_server.load("shield.ogg"),
                ..default()
            });
        }
    }
}

pub fn despawn_shield_on_right_click_release(
    mut commands: Commands,
    mouse_button_inputs: Res<Input<MouseButton>>,
    mut query: Query<(&Children, &mut PlayerShield), With<LocalPlayer>>,
    shield_query: Query<Entity, With<Shield>>,
) {
    let player = query.get_single_mut();
    if player.is_err() { return; }
    let (player_children, mut player_shield) = player.unwrap();
    if !mouse_button_inputs.pressed(MouseButton::Right) {
        player_shield.active = false;
        for &child in player_children.iter() {
            if shield_query.get(child).is_ok() {
                commands.entity(child).despawn();
            }
        }
    }
}
 */
// shield

pub fn spawn_simulate(
    tick: Res<TickNum>,
    mut spawn_reader: EventReader<SpawnEvent>,
    mut players: Query<(&Player, &mut HpBuffer, Option<&LocalPlayer>)>
) {
    for ev in &mut spawn_reader {
        for (pl, mut hb, lp) in &mut players {
            if pl.0 != ev.id { continue }
            hb.0.set(tick.0, Some(PLAYER_DEFAULT_HP));
            if lp.is_some() {
                println!("setting {} to {} by spawn", tick.0, PLAYER_DEFAULT_HP);
            }
        }
    }
    spawn_reader.clear();
}

pub fn health_simulate(
    tick: Res<TickNum>,
    mut players: Query<(&HpBuffer, &mut Health, &mut Visibility, &mut Stats, Option<&LocalPlayer>)>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut death_writer: EventWriter<LocalPlayerDeathEvent>,
    mut spawn_writer: EventWriter<LocalPlayerSpawnEvent>,
) {
    for (hb, mut hp, mut vis, mut stats, lp) in &mut players {
        let next_hp = hb.0.get(tick.0);
        if next_hp.is_none() { continue }
        hp.current = next_hp.unwrap();
        hp.max = PLAYER_DEFAULT_HP;
        if hp.current > 0 && hp.dead {
            hp.dead = false;
            if lp.is_some() {
                spawn_writer.send(LocalPlayerSpawnEvent);
            }
            *vis = Visibility::Visible;
        }
        else if hp.current == 0 && !hp.dead {
            commands.spawn(AudioBundle {
                source: asset_server.load("dead-2.ogg"),
                settings: PlaybackSettings { volume: bevy::audio::Volume::Absolute(VolumeLevel::new(0.5)), ..default()}
            });
            hp.dead = true;
            *vis = Visibility::Hidden;
            if lp.is_some() {
                death_writer.send(LocalPlayerDeathEvent);
            }
            stats.deaths = stats.deaths.saturating_add(1);
            if stats.deaths != 0 {
                stats.kd_ratio = stats.players_killed as f32 / stats.deaths as f32;
            }
            else {
                stats.kd_ratio = stats.players_killed as f32;
            }
        }
    }
}

pub fn health_draw(
    players: Query<(&Health, &Children)>,
    mut health_bars: Query<&mut Transform, With<HealthBar>>,
) {
    for (hp, children) in &players {
        for child in children.iter() {
            let tf = health_bars.get_mut(*child);
            if let Ok(mut tf) = tf {
                tf.scale = Vec3::new((hp.current as f32) / (hp.max as f32), 1.0, 1.0);
            }
        }
    }
}

// EVENT HANDLERS

pub fn handle_player_ticks(
    tick: Res<TickNum>,
    mut player_reader: EventReader<PlayerTickEvent>,
    mut player_query: Query<(&Player, &mut PosBuffer, &mut HpBuffer, &mut DirBuffer, &mut EventBuffer)>,
) {
    for ev in player_reader.iter() {
        for (pl, mut pb, mut hb, mut db, mut eb) in &mut player_query {
            if pl.0 == ev.tick.id {
                pb.0.set(ev.seq_num, Some(ev.tick.pos));

                hb.0.set(tick.0, Some(ev.tick.hp));
                db.0.set(ev.seq_num, Some(ev.tick.dir));
                eb.0.set(ev.seq_num, Some(ev.tick.events));
            }
        }
    }
}

/// This is for assigning IDs to players during the connection phase
pub fn handle_id_events(
    mut id_reader: EventReader<SetIdEvent>,
    mut res_id: ResMut<PlayerId>,
    mut app_state_next_state: ResMut<NextState<AppState>>,
) {
    for ev in &mut id_reader {
        res_id.0 = ev.0;
        app_state_next_state.set(AppState::Game);
    }
}

pub fn handle_usercmd_events(
    tick: Res<TickNum>,
    mut usercmd_reader: EventReader<UserCmdEvent>,
    mut player_query: Query<(&Player, &mut PosBuffer, &mut DirBuffer, &mut EventBuffer)>,
    mut attack_writer: EventWriter<AttackEvent>,
    mut spawn_writer: EventWriter<SpawnEvent>,
) {
    for ev in usercmd_reader.iter() {
        for (pl, mut pb, mut db, mut eb) in &mut player_query {
            if pl.0 == ev.id {
                pb.0.set_with_time(ev.seq_num, Some(ev.tick.pos), ev.seq_num);
                db.0.set(ev.seq_num, Some(ev.tick.dir));
                //eb.0.set(tick.0, Some(ev.tick.events));
                eb.0.set(ev.seq_num, Some(ev.tick.events));
                if ev.tick.events & ATTACK_BITFLAG != 0 {
                    attack_writer.send(AttackEvent { seq_num: ev.seq_num, id: ev.id });
                }
                if ev.tick.events & SPAWN_BITFLAG != 0 {
                    spawn_writer.send(SpawnEvent { id: ev.id });
                }
            }
        }
    }
}

// RUN CONDITIONS

pub fn local_player_dead(health: Query<&Health, With<LocalPlayer>>) -> bool {
    let health = health.get_single();
    if health.is_err() { return false; }
    let health = health.unwrap();
    return health.dead;
}