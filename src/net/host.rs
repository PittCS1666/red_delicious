use std::net::*;
use bevy::prelude::*;
use bincode::{deserialize, serialize};
use crate::game::{enemy, player, PlayerId};
use crate::{menus, net};
use crate::game::buffers::PosBuffer;
use crate::game::player::UserCmdEvent;
use crate::components::*;

#[derive(Copy, Clone)]
pub struct Connection {
    pub socket: SocketAddr,
    pub player_id: u8,
    pub ack: u16,  // if the ack is older than TIMEOUT ticks ago, disconnect the player
    pub ack_bits: u32
}

#[derive(Resource)]
pub struct Connections(pub [Option<Connection>; player::MAX_PLAYERS-1]); // -1 because host not included

pub fn startup(mut commands: Commands) {
    commands.insert_resource(Connections { 0: [None; player::MAX_PLAYERS-1] });
}

pub fn connect(addresses: Res<menus::NetworkAddresses>,
    mut sock: ResMut<net::Socket>
) {
    let host_ip = Ipv4Addr::new(127,0,0,1);
    //let host_port = u16::from_str(&addresses.host_port).expect("bad host port");
    let host_port: u16 = 8085;
    let host_addr = SocketAddr::new(IpAddr::from(host_ip), host_port);
    sock.0 = Some(UdpSocket::bind(host_addr).expect("host port in use"));
    sock.0.as_mut().unwrap().set_nonblocking(true).expect("can't set nonblocking");
    println!("host successful");
}

pub fn disconnect(
    mut sock: ResMut<net::Socket>,
    mut conns: ResMut<Connections>
) {
    sock.0.take();
    for conn in conns.0.iter_mut() {
        conn.take();
    }
}

//TODO this needs to run after every other fixed has run
pub fn fixed(
    mut sock: ResMut<net::Socket>,
    tick: Res<net::TickNum>,
    conns: Res<Connections>,
    player_query: Query<(&PosBuffer, &Health, &Player)>,
    enemy_query: Query<(&PosBuffer, &Health, &Enemy)>
) {
    if sock.0.is_none() { return }
    let sock = sock.0.as_mut().unwrap();
    //TODO get all enemies and get all players, how tf am i gonna do that
    let mut players = [player::PlayerTick {
        pos: Default::default(),
        hp: 0,
    }; player::MAX_PLAYERS];
    for (pb, hp, pl) in &player_query {
        players[pl.0 as usize] = player::PlayerTick {
            pos: *pb.0.get(tick.0),
            hp: hp.current
        }
    }
    let mut enemies = [enemy::EnemyTick {
        pos: Default::default(),
        hp: 0,
    }; enemy::MAX_ENEMIES];
    for (pb, hp, en) in &enemy_query {
        enemies[en.0 as usize] = enemy::EnemyTick {
            pos: *pb.0.get(tick.0),
            hp: hp.current
        }
    }
    let mut packet = net::Packet {
        protocol: net::MAGIC_NUMBER,
        contents: net::PacketContents::HostTick {
            seq_num: tick.0,
            player_id: 0,
            players,
            enemies,
        },
    };
    for conn in conns.0.iter() {
        if conn.is_none() { continue; }
        let socket = conn.unwrap().socket;
        match &mut packet.contents {
            &mut net::PacketContents::HostTick {
                    seq_num: _,
                    ref mut player_id,
                    players: _,
                    enemies: _
                } => *player_id = conn.unwrap().player_id,
            _ => unreachable!()
        };
        sock.send_to(serialize(&packet).expect("couldn't serialize").as_slice(), socket).expect("send failed");
    }
}

pub fn update(
    mut sock: ResMut<net::Socket>,
    mut conns: ResMut<Connections>,
    tick_num: Res<net::TickNum>,
    mut usercmd_writer: EventWriter<UserCmdEvent>,
) {
    if sock.0.is_none() { return }
    let sock = sock.0.as_mut().unwrap();
    loop {
        let mut buf = [0; net::MAX_PACKET_LEN];
        // this error checking is really shitty
        let recv = sock.recv_from(&mut buf);
        if recv.is_err() { break; }
        let (size, origin) = recv.unwrap();
        if size <= 0 { break; }
        let packet = deserialize::<net::Packet>(&buf);
        if packet.is_err() { break; }
        let packet = packet.unwrap();
        if packet.protocol != net::MAGIC_NUMBER { continue; }
        match packet.contents {
            net::PacketContents::ClientTick {
                seq_num, tick
            } => {
                let mut found_connection = false;
                let mut added_connection = false;
                let mut id = 0;
                for conn in &mut conns.0 {
                    if conn.is_some() {
                        let conn = conn.unwrap();
                        if conn.socket == origin {
                            found_connection = true;
                            id = conn.player_id;
                            break;
                            // TODO update ack for the connection
                        }
                    }
                }
                if !found_connection {
                    let mut unused = [false; player::MAX_PLAYERS - 1];
                    for conn in &conns.0 {
                        if let Some(conn) = conn {
                            unused[conn.player_id as usize] = true;
                        }
                    }
                    let mut fresh_id = 1;
                    for (i, b) in unused.into_iter().enumerate() {
                        if i == fresh_id && b {
                            fresh_id += 1;
                        }
                    }
                    for conn in &mut conns.0 {
                        if conn.is_none() {
                            added_connection = true;
                            let _ = conn.insert(Connection {
                                socket: origin,
                                player_id: fresh_id as u8,
                                ack: 0,
                                ack_bits: 0,
                            });
                            id = fresh_id as u8;
                            println!("added connection with id {:?}", fresh_id);
                            break;
                        }
                    }
                }
                if !found_connection && !added_connection {
                    let packet = net::Packet {
                        protocol: net::MAGIC_NUMBER,
                        contents: net::PacketContents::ServerFull
                    };
                    let ser = serialize(&packet).expect("couldn't serialize");
                    sock.send_to(ser.as_slice(), origin).expect("send failed");
                    println!("server full");
                    continue
                }
                if seq_num < tick_num.0 - net::DELAY {
                    // TODO deal with packet misses
                    println!("packet late, local is {} remote is {}", tick_num.0, seq_num);
                    continue
                }
                // send event that this player has moved to this location
                usercmd_writer.send(UserCmdEvent {
                    seq_num,
                    id,
                    tick
                });
            },
            net::PacketContents::Disconnect => {
                // for disconnect packet, check if they are still connected and remove their connection
                // TODO remove player from gamestate
                for conn in &mut conns.0 {
                    if conn.is_some() {
                        let s = conn.unwrap().socket;
                        if s == origin {
                            conn.take();
                        }
                    }
                }
            },
            p => panic!("client sent unexpected packet {:?}", p)
        }
    }
}
