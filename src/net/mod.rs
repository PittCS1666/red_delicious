pub mod host;
pub mod client;
pub mod lerp;

use std::net::UdpSocket;
use bevy::prelude::*;
use serde::{Serialize, Deserialize};
use crate::AppState;
use crate::game::{enemy, player};
use crate::game::player::UserCmd;


pub const TICKRATE: u8 = 10;
const TICKLEN_S: f32 = 1. / TICKRATE as f32;
pub const DELAY: u16 = 2;
pub const MAX_PACKET_LEN: usize = 4096;  // probably should check if this is the size of a HostTick
pub const MAGIC_NUMBER: u16 = 24835; // 8008135 % 69420
pub const TIMEOUT: u16 = TICKRATE as u16 * 30;  // 30 seconds to timeout

#[derive(Resource)]
pub struct TickNum(pub u16);  // this is the tick we're writing to, NOT playing back

#[derive(Resource)]
pub struct Socket(pub Option<UdpSocket>);

#[derive(Resource)]
pub struct IsHost(pub bool);

#[derive(Serialize, Deserialize, Debug)]
pub enum PacketContents {
    ServerFull,  // sent by host every time request is received and server is full
    Disconnect,  // sent by client in disconnected state every time HostTick is received
    HostTick {  // sent by host to all connected clients individually
        seq_num: u16,
        //ack: u16,
        //ack_bits: u32,
        players: [player::PlayerTick; player::MAX_PLAYERS],
        enemies: [enemy::EnemyTick; enemy::MAX_ENEMIES],
    },
    ClientTick {  // sent by client to host every FixedUpdate unless ServerFull received
        seq_num: u16,
        //ack: u16,
        //ack_bits: u32,
        tick: UserCmd
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Packet {
    protocol: u16,
    contents: PacketContents
}

pub struct NetPlugin;

impl Plugin for NetPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup,
        (startup,
        host::startup))  // you cant conditionally run this unless you do a bunch of bullshit
            .add_systems(FixedUpdate,
                         (increment_tick.run_if(is_host),
                         client::fixed.run_if(is_client).after(player::fixed),
                         host::fixed.run_if(is_host).after(enemy::fixed_move),
                         lerp::resolve_collisions.run_if(is_host).before(increment_tick)))
            .add_systems(Update,
                         (lerp::lerp_pos,
                         client::update.run_if(is_client),
                         host::update.run_if(is_host)))
            .add_systems(OnEnter(AppState::Game),
                         (client::connect.run_if(is_client),
                         host::connect.run_if(is_host)))
            .add_systems(OnExit(AppState::Game),
                     (client::disconnect.run_if(is_client),
                      host::disconnect.run_if(is_host)));
    }
}

pub fn startup(mut commands: Commands) {
    commands.insert_resource(FixedTime::new_from_secs(TICKLEN_S));
    commands.insert_resource(TickNum { 0: 0 });
    commands.insert_resource(Socket(None));
    commands.insert_resource(IsHost(true));
}

pub fn increment_tick(mut tick_num: ResMut<TickNum>) {
    tick_num.0 += 1;
}

// for conditionally running systems
pub fn is_host(is_host: Res<IsHost>) -> bool {
    is_host.0
}

pub fn is_client(is_host: Res<IsHost>) -> bool {
    !is_host.0
}