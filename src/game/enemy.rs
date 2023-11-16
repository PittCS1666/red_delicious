use bevy::math::Vec3Swizzles;
use bevy::prelude::*;
use crate::{AppState, net};
use crate::Atlas;
use movement::correct_wall_collisions;
use crate::game::buffers::{CircularBuffer, PosBuffer};
use crate::game::components::*;
use crate::net::{is_client, is_host, TickNum};
use crate::game::components::PowerUpType;
use crate::game::map::WorldMap;
use crate::game::movement;
use crate::game::player::PlayerShield;
use super::player::PLAYER_DEFAULT_DEF;

pub const ENEMY_SIZE: Vec2 = Vec2 { x: 32., y: 32. };
pub const ENEMY_SPEED: f32 = 150. / net::TICKRATE as f32;
pub const ENEMY_MAX_HP: u8 = 100;
pub const AGGRO_RANGE: f32 = 200.0;

const CIRCLE_RADIUS: f32 = 64.;
const CIRCLE_DAMAGE: u8 = 15;

//TODO public struct resource holding enemy count



#[derive(Component)]
pub struct EnemyWeapon;

#[derive(Component)]
pub struct LastAttacker(pub Option<u8>);

#[derive(Component)]
pub struct Aggro(pub Option<u8>);

#[derive(Component)]
struct DespawnEnemyWeaponTimer(Timer);

#[derive(Component)]
pub struct SpawnEnemyWeaponTimer(Timer);

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin{
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, (
                fixed_aggro,
                fixed_move.after(fixed_aggro),
                fixed_resolve.run_if(in_state(AppState::Game)).after(fixed_move).after(net::lerp::resolve_collisions),
                ).run_if(is_host)
            )
            .add_systems(Update, (
                handle_packet.run_if(is_client),
                update_enemies.after(handle_packet),
                handle_attack.before(update_enemies),
            ))
            .add_systems(OnExit(AppState::Game), remove_enemies);
    }
}

pub fn spawn_enemy(
    commands: &mut Commands, 
    entity_atlas: &Res<Atlas>, 
    id: u8, 
    campid: u8, 
    pos: Vec2, 
    sprite: i32, 
    power_up_type: PowerUpType
) {
    let pb = PosBuffer(CircularBuffer::new_from(pos));
    let mut pu: [u8; NUM_POWERUPS];
    pu = [0; NUM_POWERUPS];
    pu[power_up_type as usize] = 1;
    commands.spawn((
        Enemy(id),
        pb,
        Health {
            current: ENEMY_MAX_HP,
            max: ENEMY_MAX_HP,
            dead: false,
        },
        EnemyCamp(campid),
        SpriteSheetBundle {
            texture_atlas: entity_atlas.handle.clone(),
            sprite: TextureAtlasSprite { index: entity_atlas.coord_to_index(0, sprite), ..default()},
            //TODO: change this to translate based on parent xyz
            transform: Transform::from_xyz(0., 0., 2.),
            ..default()
        },
        Collider(ENEMY_SIZE),
        LastAttacker(None),
        StoredPowerUps
        {
            power_ups: pu,
        },
        Aggro(None),
        SpawnEnemyWeaponTimer(Timer::from_seconds(4.0, TimerMode::Repeating)),//add a timer to spawn the enemy attack very 4 seconds
    ));
}

pub fn remove_enemies(mut commands: Commands, enemies: Query<Entity, With<Enemy>>) {
    for e in enemies.iter() {
        commands.entity(e).despawn_recursive();
    }
}

pub fn handle_attack(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    time: Res<Time>,
    mut query_enemies: Query<(Entity, &Transform, &mut SpawnEnemyWeaponTimer), With<Enemy>>,
    mut player_query: Query<(&Transform, &mut Health, &StoredPowerUps, &PlayerShield), With<Player>>
) {
    for (enemy_entity, enemy_transform, mut spawn_timer) in query_enemies.iter_mut() {
        spawn_timer.0.tick(time.delta());
        if spawn_timer.0.finished() {
            let attack = commands.spawn((SpriteBundle {
                texture: asset_server.load("EnemyAttack01.png").into(),
                transform: Transform {
                    translation: Vec3::new(0.0, 0.0, 5.0),
                    ..Default::default()
                },
                ..Default::default() },
                EnemyWeapon,
                Fade {current: 1.0, max: 1.0})).id();
            let enemy_entity = commands.get_entity(enemy_entity);
            if enemy_entity.is_none() {continue}
            let mut enemy_entity = enemy_entity.unwrap();
            // TODO if this panics ever again, check if enemy hp is 0 or less and continue; if so
            enemy_entity.add_child(attack);
            for (player_transform, mut player_hp, player_power_ups, shield) in player_query.iter_mut() {
                if player_transform.translation.distance(enemy_transform.translation) < CIRCLE_RADIUS {
                    // must check if damage reduction is greater than damage dealt, otherwise subtraction overflow or player will gain health
                    if shield.active { continue }
                    // Multiply enemy's damage value by player's default defense and DAMAGE_REDUCTION_UP ^ stacks of damage reduction
                    let dmg: u8 = (CIRCLE_DAMAGE as f32 * PLAYER_DEFAULT_DEF * DAMAGE_REDUCTION_UP.powf(player_power_ups.power_ups[PowerUpType::DamageReductionUp as usize] as f32)) as u8;
                    if dmg > 0
                    {
                        match player_hp.current.checked_sub(dmg) {
                            Some(v) => {
                                player_hp.current = v;
                            }
                            None => {
                                // player would die from hit
                                player_hp.current = 0;
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn update_enemies(
    mut commands: Commands,
    mut enemies: Query<(Entity, &Health, &LastAttacker, &StoredPowerUps, &mut TextureAtlasSprite, &Transform, &EnemyCamp), With<Enemy>>,
    mut scores: Query<(&mut Score, &Player)>,
    asset_server: Res<AssetServer>,
    mut camp_query: Query<(&Camp, &mut CampEnemies, &CampStatus), With<Camp>>,
) {
    for (e, hp, la, spu, mut sp, tf, ec_num) in enemies.iter_mut() {
        if hp.current <= 0 {
            // drop powerups by cycling through the stored powerups of the enemy
            // and spawning the appropriate one
            let power_up_icons = vec!["flamestrike.png", "rune-of-protection.png", "meat.png", "lightning.png", "berserker-rage.png"];
            for (index, &element) in spu.power_ups.iter().enumerate() {
                if element == 1
                {
                    commands.spawn((SpriteBundle {
                        texture: asset_server.load(power_up_icons[index]).into(),
                        transform: Transform {
                            translation: Vec3::new(tf.translation.x, tf.translation.y, 1.0),
                            ..Default::default()
                        },
                        ..Default::default()},
                                    PowerUp(unsafe { std::mem::transmute(index as u8) } ),
                    ));
                }
            }
            // decrement the enemy counter of the camp that this enemy is apart of
            for (camp_num, mut enemies_in_camp, camp_status) in camp_query.iter_mut() {
                if camp_num.0 == ec_num.0 {
                    enemies_in_camp.current_enemies -= 1;
                }

                // check if the camp is cleared and assign 5 points for clearing the camp
                if enemies_in_camp.current_enemies == 0 && camp_status.status == true{
                    for (mut score, pl) in scores.iter_mut() {
                        if pl.0 == la.0.expect("camp has no attacker") {
                            score.0 += 5;
                            println!("5 points awarded for clearing camp {}", camp_num.0)
                        }
                    }
                }
            }

            // despawn the enemy and increment the score of the player who killed it
            commands.entity(e).despawn_recursive();
            for (mut score, pl) in scores.iter_mut() {
                if pl.0 == la.0.expect("died with no attacker?") {
                    score.0 += 1;
                }
            }
            continue;
        }
        let damage = hp.current as f32 / hp.max as f32;
        sp.color = Color::Rgba {red: 1.0, green: damage, blue: damage, alpha: 1.0};
    }
}

pub fn fixed_aggro(
    tick: Res<net::TickNum>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut enemies: Query<(Entity, &PosBuffer, &mut Aggro), With<Enemy>>,
    players: Query<(&Player, &PosBuffer, &Health), Without<Enemy>>
) {
    for (enemy_entity, epb, mut aggro) in &mut enemies {
        let prev = epb.0.get(tick.0.wrapping_sub(1));
        let mut closest_player = None;
        let mut best_distance = f32::MAX;
        for (pl, ppb, hp) in &players {
            if hp.dead { continue }
            let dist = ppb.0.get(tick.0).distance(*prev);
            if dist < best_distance {
                best_distance = dist;
                closest_player = Some(pl);
            }
        }
        if best_distance > AGGRO_RANGE || closest_player.is_none() {
            if aggro.0.is_some() {
                //TODO show lost contact
            }
            aggro.0 = None;
        }
        else {
            if aggro.0.is_none() {
                let exlaim = commands.spawn((
                    SpriteBundle {
                        texture: asset_server.load("aggro.png").into(),
                        transform: Transform {
                            translation: Vec3::new(0.0, 32., 5.0),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    Fade {
                        current: 2.0,
                        max: 2.0
                    }
                )).id();
                commands.entity(enemy_entity).push_children(&[exlaim]);
            }
            let _ = aggro.0.insert(closest_player.unwrap().0);
        }
    }
}

pub fn fixed_move(
    tick: Res<net::TickNum>,
    mut enemies: Query<(&mut PosBuffer, &Aggro), (With<Enemy>, Without<Player>)>,
    players: Query<(&Player, &PosBuffer), (With<Player>, Without<Enemy>)>
) {
    for (mut epb, aggro) in &mut enemies {
        let prev = epb.0.get(tick.0.wrapping_sub(1));
        let mut next = prev.clone();

        'mov: {
            if aggro.0.is_none() { break 'mov }
            let aggro = aggro.0.unwrap();
            let mut ppbo = None;
            for (pl, ppb) in &players {
                if pl.0 == aggro {
                    ppbo = Some(ppb);
                }
            }
            if ppbo.is_none() { break 'mov }
            let player_pos = ppbo.unwrap().0.get(tick.0.wrapping_sub(1));

            let displacement = *player_pos - *prev;
            if !(displacement.length() < CIRCLE_RADIUS) {
                let movement = (*player_pos - *prev).normalize() * ENEMY_SPEED;
                next += movement;
            }
        }
        epb.0.set(tick.0, next);
    }
}

/// Resolve enemy wall collisions
pub fn fixed_resolve(
    mut enemies: Query<(&mut PosBuffer, &Collider), With<Enemy>>,
    map: Res<WorldMap>,
    tick: Res<TickNum>,
) {
    for (enemy_pos_buffer, collider) in &mut enemies {
        let pos_buffer = enemy_pos_buffer.into_inner();
        let pos = pos_buffer.0.get(tick.0);
        let mut pos3 = Vec3::new(pos.x, pos.y, 0.0);
        pos3 = correct_wall_collisions(&pos3, &collider.0, &map.biome_map);
        pos_buffer.0.set(tick.0, pos3.xy());
    }
}

pub fn handle_packet(
    mut enemy_reader: EventReader<net::packets::EnemyTickEvent>,
    mut enemy_query: Query<(&Enemy, &mut PosBuffer)>
) {
    //TODO if you receive info that your predicted local position is wrong, it needs to be corrected
    for ev in enemy_reader.iter() {
        // TODO this is slow but i have no idea how to make the borrow checker okay
        //   with the idea of an array of player PosBuffer references
        for (pl, mut pb) in &mut enemy_query {
            if pl.0 == ev.tick.id {
                pb.0.set(ev.seq_num, ev.tick.pos);
            }
        }
    }
}
