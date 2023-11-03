use bevy::math::Vec3Swizzles;
use bevy::prelude::*;
use bevy::reflect::Enum;
use bevy::sprite::collide_aabb::collide;
use crate::{AppState, net};
use crate::Atlas;
use serde::{Deserialize, Serialize};
use crate::game::buffers::{CircularBuffer, PosBuffer};
use crate::game::components::*;
use crate::game::components::PowerUpType;
use crate::game::map::{Biome, get_pos_in_tile, get_tile_at_pos, TILESIZE, WorldMap};
use crate::net::{is_host, TickNum};


pub const MAX_ENEMIES: usize = 32;
pub const ENEMY_SIZE: Vec2 = Vec2 { x: 32., y: 32. };
pub const ENEMY_SPEED: f32 = 150. / net::TICKRATE as f32;
pub const ENEMY_MAX_HP: u8 = 100;
pub const FOLLOW_DISTANCE: f32 = 200.0;

const CIRCLE_RADIUS: f32 = 64.;
const CIRCLE_DAMAGE: u8 = 15;

//TODO public struct resource holding enemy count


/// sent by network module to disperse information from the host
#[derive(Event, Debug)]
pub struct EnemyTickEvent {
    pub seq_num: u16,
    pub id: u8,
    pub tick: EnemyTick
}

/// the information that the host needs to produce on each tick
#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub struct EnemyTick {
    pub pos: Vec2,
    pub hp: u8
}

#[derive(Component)]
pub struct EnemyWeapon;

#[derive(Component)]
pub struct LastAttacker(pub Option<u8>);

#[derive(Component)]
struct DespawnEnemyWeaponTimer(Timer);

#[derive(Component)]
pub struct SpawnEnemyWeaponTimer(Timer);

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin{
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, (
                fixed_move.run_if(is_host),
                fixed_resolve.after(fixed_move).run_if(is_host),
            ))
            .add_systems(Update, (
                packet,
                spawn_weapon.after(handle_dead_enemy),
                despawn_after_timer,
                handle_dead_enemy,
                show_damage,
            ))
            .add_systems(OnExit(AppState::Game), despawn_enemies)
            .add_event::<EnemyTickEvent>();
    }
}

pub fn spawn_enemy(
    commands: &mut Commands, 
    entity_atlas: &Res<Atlas>, 
    id: u8, pos: Vec2, sprite: i32, power_up_type: PowerUpType
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
        },
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
        SpawnEnemyWeaponTimer(Timer::from_seconds(4.0, TimerMode::Repeating)),//add a timer to spawn the enemy attack very 4 seconds
    ));
}

pub fn despawn_enemies(mut commands: Commands, enemies: Query<Entity, With<Enemy>>) {
    for e in enemies.iter() {
        commands.entity(e).despawn_recursive();
    }
}

pub fn spawn_weapon(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    time: Res<Time>,
    mut query_enemies: Query<(Entity, &Transform, &mut SpawnEnemyWeaponTimer), With<Enemy>>,
    mut player_query: Query<(&Transform, &mut Health, &StoredPowerUps), With<Player>>
) {
    for (enemy_entity, enemy_transform, mut spawn_timer) in query_enemies.iter_mut() {
        spawn_timer.0.tick(time.delta());
        if spawn_timer.0.finished() {
            commands.entity(enemy_entity).with_children(|parent| {
                parent.spawn(SpriteBundle {
                    texture: asset_server.load("EnemyAttack01.png").into(),
                    transform: Transform {
                        translation: Vec3::new(0.0, 0.0, 5.0),
                        ..Default::default()
                    },
                    ..Default::default()
                }).insert(EnemyWeapon).insert(DespawnEnemyWeaponTimer(Timer::from_seconds(1.0, TimerMode::Once)));
            });
            for (player_transform, mut player_hp, player_power_ups) in player_query.iter_mut() {
                if player_transform.translation.distance(enemy_transform.translation) < CIRCLE_RADIUS {
                    // must check if damage reduction is greater than damage dealt, otherwise ubtraction overflow or player will gain health
                    if CIRCLE_DAMAGE > player_power_ups.power_ups[PowerUpType::DamageReductionUp as usize] * DAMAGE_REDUCTION_UP
                    {
                        match player_hp.current.checked_sub(CIRCLE_DAMAGE - player_power_ups.power_ups[PowerUpType::DamageReductionUp as usize] * DAMAGE_REDUCTION_UP) {
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

fn despawn_after_timer(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut DespawnEnemyWeaponTimer)>,
) {
    for (entity, mut despawn_timer) in query.iter_mut() {
        despawn_timer.0.tick(time.delta());
        if despawn_timer.0.finished() {
            commands.entity(entity).despawn_recursive();
        }
    }
}

// handle dead enemies by dropping all of their powerups, despawning them, and 
// incrementing the score of the player who killed them
pub fn handle_dead_enemy(
    mut commands: Commands,
    enemy_query: Query<(Entity, &Health, &LastAttacker, &StoredPowerUps, &GlobalTransform), With<Enemy>>,
    mut player_score_query: Query<(&mut Score, &Player)>,
    asset_server: Res<AssetServer>,
) {
    // TODO this is slow because it checks all enemies, but I'm not sure how to make it faster
    for (entity, health, last_attacker, stored_power_ups, position) in enemy_query.iter() {
        if health.current <= 0 {
            // drop powerups by cycling through the stored powerups of the enemy
            // and spawning the appropriate one
            let power_up_icons = vec!["flamestrike.png", "rune-of-protection.png", "meat.png", "lightning.png", "berserker-rage.png"];
            for (index, &element) in stored_power_ups.power_ups.iter().enumerate() {
                if element == 1 
                {
                    commands.spawn((SpriteBundle {
                        texture: asset_server.load(power_up_icons[index]).into(),
                        transform: Transform {
                            translation: Vec3::new(position.translation().x, position.translation().y, 1.0),
                            ..Default::default()
                        },
                        ..Default::default()},
                        PowerUp(unsafe { std::mem::transmute(index as u8) } ),
                    ));
                }
            }
            // despawn the enemy and increment the score of the player who killed it
            commands.entity(entity).despawn_recursive();
            for (mut score, pl) in player_score_query.iter_mut() {
                if pl.0 == last_attacker.0.expect("died with no attacker?") {
                    score.current_score += 1;
                }
            }
        }
    }
}

pub fn show_damage(
    mut enemies: Query<(&Health, &mut TextureAtlasSprite), With<Enemy>>,
) {
    for (health, mut sprite) in enemies.iter_mut() {
        let fade = health.current as f32 / health.max as f32;
        sprite.color = Color::Rgba {red: 1.0, green: fade, blue: fade, alpha: 1.0};
    }
}

/*
FIX AFTER MIDTERM :)
pub fn weapon_dealt_damage_system(
    mut player_query: Query<(&Transform, &Collider, &mut Health), With<Player>>,
    weapon_query: Query<&Transform, With<EnemyWeapon>>
) {
     for weapon_transform in weapon_query.iter() {
        for (player_transform, player_collider, mut player_HP) in player_query.iter_mut() {
            if let Some(_) = collide(
                weapon_transform.translation,
                weapon_transform.scale.xy(),
                player_transform.translation,
                player_collider.0,
            ) {
                match player_HP.current.checked_sub(CIRCLE_DAMAGE) {
                    Some(v) => {
                        player_HP.current = v;
                    }
                    None => {
                        // player would die from hit
                        player_HP.current = 0;
                    }
                }
            }
        }
    }
}*/

pub fn fixed_move(
    tick: Res<net::TickNum>,
    mut enemies: Query<&mut PosBuffer, (With<Enemy>, Without<Player>)>,
    players: Query<&PosBuffer, (With<Player>, Without<Enemy>)>
) {
    for mut epb in &mut enemies {
        let prev = epb.0.get(tick.0.wrapping_sub(1));
        let mut next = prev.clone();

        let mut closest_player = players.iter().next().unwrap();
        let mut best_distance = f32::MAX;
        for ppb in &players {
            let dist = ppb.0.get(tick.0).distance(*prev);
            if dist < best_distance {
                best_distance = dist;
                closest_player = ppb;
            }
        }
        let player_pos = closest_player.0.get(tick.0.wrapping_sub(1));

        let movement = (*player_pos - *prev).normalize() * ENEMY_SPEED;
        if !(movement.length() < 0.1 || movement.length() > FOLLOW_DISTANCE) {
            next += movement;
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
    for (mut enemy_pos_buffer, collider) in &mut enemies {
        let pos_buffer = enemy_pos_buffer.into_inner();
        let pos = pos_buffer.0.get(tick.0);
        let mut pos3 = Vec3::new(pos.x, pos.y, 0.0);
        for i in 0..5 {
            let mut done = true;
            let half_collider = Vec2::new(collider.0.x / 2.0, collider.0.y / 2.0);
            let north = pos3 + Vec3::new(0.0, half_collider.y, 0.0);
            let south = pos3 - Vec3::new(0.0, half_collider.y, 0.0);
            let east = pos3 + Vec3::new(half_collider.x, 0.0, 0.0);
            let west = pos3 - Vec3::new(half_collider.x, 0.0, 0.0);

            let offset: f32 = 0.1;
            if get_tile_at_pos(&north, &map.biome_map) == Biome::Wall {
                let tilepos = get_pos_in_tile(&north);
                let adjustment = tilepos.y + offset;
                pos3.y -= adjustment;
                done = false;
            }
            if get_tile_at_pos(&south, &map.biome_map) == Biome::Wall {
                let tilepos = get_pos_in_tile(&north);
                let adjustment = TILESIZE as f32 - tilepos.y + offset;
                pos3.y += adjustment;
                done = false;
            }
            if get_tile_at_pos(&east, &map.biome_map) == Biome::Wall {
                let tilepos = get_pos_in_tile(&north);
                let adjustment = tilepos.x + offset;
                pos3.x -= adjustment;
                done = false;
            }
            if get_tile_at_pos(&west, &map.biome_map) == Biome::Wall {
                let tilepos = get_pos_in_tile(&north);
                let adjustment = TILESIZE as f32 - tilepos.x + offset;
                pos3.x += adjustment;
                done = false;
            }
            if done {
                break;
            }
        }
        pos_buffer.0.set(tick.0, pos3.xy());
    }
}

pub fn packet(
    mut enemy_reader: EventReader<EnemyTickEvent>,
    mut enemy_query: Query<(&Enemy, &mut PosBuffer)>
) {
    //TODO if you receive info that your predicted local position is wrong, it needs to be corrected
    for ev in enemy_reader.iter() {
        // TODO this is slow but i have no idea how to make the borrow checker okay
        //   with the idea of an array of player PosBuffer references
        for (pl, mut pb) in &mut enemy_query {
            if pl.0 == ev.id {
                pb.0.set(ev.seq_num, ev.tick.pos);
            }
        }
    }
}
