// The idea behind this file is to hold components which will be used across gameplay files
// Components which are only used locally can be left inside a more localized file.
use bevy::prelude::*;
use core::fmt::Debug;

#[derive(Component)]
pub struct Health {
    pub current: u8,
    pub max: u8,
    pub dead: bool
}

#[derive(Component)]
pub struct Fade {
    pub current: f32,
    pub max: f32
}

pub const NUM_POWERUPS: usize = 5;
pub const DAMAGE_DEALT_UP: u8 = 10;
pub const DAMAGE_REDUCTION_UP: u8 = 5;
pub const MAX_HP_UP: u8 = 20;
pub const ATTACK_SPEED_UP: f32 = 1.1;
pub const MOVEMENT_SPEED_UP: u8 = 15;

#[derive(Debug, Copy, Clone)]
pub enum PowerUpType {
    DamageDealtUp = 0,
    DamageReductionUp = 1,
    MaxHPUp = 2,
    AttackSpeedUp = 3,
    MovementSpeedUp = 4,
}

#[derive(Component)]
pub struct StoredPowerUps{
    pub power_ups: [u8; NUM_POWERUPS],
    // 0: DamageDealtUp, 1: DamageReductionUp, 2: MaxHPUp, 3: AttackSpeedUp, 4: MovementSpeedUp
}

#[derive(Component)]
pub struct PowerUp(pub PowerUpType);

/// Just a vec2 that describes the size of a bounding box around the entity
#[derive(Component)]
pub struct Collider(pub Vec2);

#[derive(Component)]
pub struct Score(pub u8);

#[derive(Component)]
pub struct ScoreDisplay;

#[derive(Component)]
pub struct Enemy(pub u8);  // holds id

#[derive(Component)]
pub struct Player(pub u8);  // holds id

// camp stuff
#[derive(Component)]
pub struct Camp(pub u8); // holds id


#[derive(Component)]
pub struct Grade(pub u8);

#[derive(Component)]
pub struct CampEnemies{
    pub max_enemies: u8, 
    pub current_enemies: u8,
}

#[derive(Component)]
pub struct CampStatus{
    //status of camp : true = filled, false = clear
    pub status: bool,
}

#[derive(Component)]
pub struct EnemyCamp(pub u8); // holds id of enemy's parent camp


