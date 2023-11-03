use bevy::prelude::*;
use crate::AppState;
use crate::game::enemy;
use crate::Atlas;
use crate::map::{MAPSIZE, TILESIZE};
use crate::components::PowerUpType;
use crate::components::*;

use crate::buffers::*;

#[derive(Component)]
pub struct ListOfCamps;

pub struct CampPlugin;

impl Plugin for CampPlugin{
    fn build(&self, app: &mut App){
        app.add_systems(OnEnter(AppState::Game), setup);
    }
}

pub fn setup(
    mut commands: Commands,
    entity_atlas:Res<Atlas>,
) {

    //spawn a camp at a specified position

    //get spawn vec can later be replaced with some noise thing to randomly generate where its placed
    let pb = PosBuffer(CircularBuffer::new_from(get_spawn_vec(138., 109.)));
    let new_camp = commands.spawn((
        Camp(1),
        pb,
        Grade{
            grade: 1,
        },
        CampEnemies{
            current_enemies: 5,
        },
        CampStatus{
            status: true,
        },
    )).id();


    // enemy::spawn_enemy(&mut commands, &entity_atlas, 1, get_spawn_vec(31., 81.), 2, PowerUpType::DamageDealtUp);
    // enemy::spawn_enemy(&mut commands, &entity_atlas, 1, get_spawn_vec(45., 83.), 2, PowerUpType::DamageDealtUp);
    // enemy::spawn_enemy(&mut commands, &entity_atlas, 1, get_spawn_vec(56., 91.), 2, PowerUpType::DamageDealtUp);

    // enemy::spawn_enemy(&mut commands, &entity_atlas, 1, get_spawn_vec(116., 16.), 3, PowerUpType::DamageReductionUp);
    // enemy::spawn_enemy(&mut commands, &entity_atlas, 1, get_spawn_vec(119., 27.), 3, PowerUpType::DamageReductionUp);
    // enemy::spawn_enemy(&mut commands, &entity_atlas, 1, get_spawn_vec(130., 22.), 3, PowerUpType::DamageReductionUp);

    // enemy::spawn_enemy(&mut commands, &entity_atlas, 1, get_spawn_vec(207., 16.), 4, PowerUpType::AttackSpeedUp);
    // enemy::spawn_enemy(&mut commands, &entity_atlas, 1, get_spawn_vec(234., 15.), 4, PowerUpType::AttackSpeedUp);
    // enemy::spawn_enemy(&mut commands, &entity_atlas, 1, get_spawn_vec(224., 32.), 4, PowerUpType::AttackSpeedUp);
    // enemy::spawn_enemy(&mut commands, &entity_atlas, 1, get_spawn_vec(202., 41.), 4, PowerUpType::AttackSpeedUp);
    // enemy::spawn_enemy(&mut commands, &entity_atlas, 1, get_spawn_vec(240., 46.), 4, PowerUpType::AttackSpeedUp);
    // enemy::spawn_enemy(&mut commands, &entity_atlas, 1, get_spawn_vec(212., 66.), 4, PowerUpType::AttackSpeedUp);
    // enemy::spawn_enemy(&mut commands, &entity_atlas, 1, get_spawn_vec(228., 63.), 4, PowerUpType::AttackSpeedUp);
    // enemy::spawn_enemy(&mut commands, &entity_atlas, 1, get_spawn_vec(248., 79.), 4, PowerUpType::AttackSpeedUp);

    // enemy::spawn_enemy(&mut commands, &entity_atlas, 0, get_spawn_vec(121., 124.), 5, PowerUpType::MovementSpeedUp);
    // enemy::spawn_enemy(&mut commands, &entity_atlas, 1, get_spawn_vec(126., 112.), 5, PowerUpType::MovementSpeedUp);
    // enemy::spawn_enemy(&mut commands, &entity_atlas, 2, get_spawn_vec(133., 120.), 5, PowerUpType::MovementSpeedUp);
    // enemy::spawn_enemy(&mut commands, &entity_atlas, 1, get_spawn_vec(133., 130.), 5, PowerUpType::MovementSpeedUp);
    // enemy::spawn_enemy(&mut commands, &entity_atlas, 1, get_spawn_vec(141., 113.), 5, PowerUpType::MovementSpeedUp);
    // enemy::spawn_enemy(&mut commands, &entity_atlas, 1, get_spawn_vec(139., 125.), 5, PowerUpType::MovementSpeedUp);

    // enemy::spawn_enemy(&mut commands, &entity_atlas, 1, get_spawn_vec(27., 138.), 4, PowerUpType::AttackSpeedUp);

    // enemy::spawn_enemy(&mut commands, &entity_atlas, 1, get_spawn_vec(84., 149.), 3, PowerUpType::DamageReductionUp);

    // enemy::spawn_enemy(&mut commands, &entity_atlas, 1, get_spawn_vec(167., 164.), 2, PowerUpType::DamageDealtUp);
    // enemy::spawn_enemy(&mut commands, &entity_atlas, 1, get_spawn_vec(198., 169.), 2, PowerUpType::DamageDealtUp);

    // enemy::spawn_enemy(&mut commands, &entity_atlas, 1, get_spawn_vec(247., 201.), 4, PowerUpType::AttackSpeedUp);

    // enemy::spawn_enemy(&mut commands, &entity_atlas, 1, get_spawn_vec(168., 212.), 1, PowerUpType::MaxHPUp);

    // enemy::spawn_enemy(&mut commands, &entity_atlas, 1, get_spawn_vec(112., 215.), 2, PowerUpType::DamageDealtUp);

    // enemy::spawn_enemy(&mut commands, &entity_atlas, 1, get_spawn_vec(45., 185.), 5, PowerUpType::MovementSpeedUp);
    // enemy::spawn_enemy(&mut commands, &entity_atlas, 1, get_spawn_vec(34., 195.), 5, PowerUpType::MovementSpeedUp);
    // enemy::spawn_enemy(&mut commands, &entity_atlas, 1, get_spawn_vec(39., 211.), 5, PowerUpType::MovementSpeedUp);
    // enemy::spawn_enemy(&mut commands, &entity_atlas, 1, get_spawn_vec(25., 229.), 5, PowerUpType::MovementSpeedUp);
    // enemy::spawn_enemy(&mut commands, &entity_atlas, 1, get_spawn_vec(11., 207.), 5, PowerUpType::MovementSpeedUp);


}

//convert given row and col into x and y coordinates. Returns a vec2 of these coordinates
// This is the same formula
fn get_spawn_vec(row: f32, col:f32) -> Vec2{
    let x_coord = TILESIZE as f32 * (col - (MAPSIZE as f32/2.));
    let y_coord = TILESIZE as f32 * ((MAPSIZE as f32/2.) - row);

    Vec2::new(x_coord, y_coord)
}