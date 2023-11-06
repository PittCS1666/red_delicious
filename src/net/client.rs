use std::net::*;
use std::str::FromStr;
use bevy::prelude::*;
use bincode::{deserialize, serialize};
use crate::{game, menus, net};
use crate::game::buffers::PosBuffer;
use crate::game::enemy::EnemyTickEvent;
use crate::game::player::{LocalPlayer, PlayerTickEvent, UserCmd};


pub fn connect(
    addresses: Res<menus::NetworkAddresses>,
    mut sock: ResMut<net::Socket>
) {
    // I think if you communicate over LAN, you have to use local ip rather than loopback ip
    let client_ip = Ipv4Addr::new(127,0,0,1);
    //let client_port = u16::from_str(&addresses.client_port).expect("bad client port");
    let client_port: u16 = 8086;
    let client_addr = SocketAddr::new(IpAddr::from(client_ip), client_port);
    sock.0 = Some(UdpSocket::bind(client_addr).expect("client port in use"));
    sock.0.as_mut().unwrap().set_nonblocking(true).expect("can't set nonblocking");
    //let host_ip = Ipv4Addr::from_str(&addresses.ip).expect("bad ip");
    let host_ip = Ipv4Addr::from_str("127.0.0.1").expect("bad ip");
    //let host_port = u16::from_str(&addresses.host_port).expect("bad host port");
    let host_port: u16 = 8085;
    let host_addr = SocketAddr::new(IpAddr::from(host_ip), host_port);
    sock.0.as_mut().unwrap().connect(host_addr).expect("can't connect to host");
    println!("connection successful");
}

pub fn disconnect(mut sock: ResMut<net::Socket>) {
    sock.0.take();
}

pub fn fixed(
    mut sock: ResMut<net::Socket>,
    tick: Res<net::TickNum>,
    pb_query: Query<&PosBuffer, With<LocalPlayer>>
) {
    if sock.0.is_none() { return }
    let sock = sock.0.as_mut().unwrap();
    let pb = pb_query.get_single();
    if pb.is_err() { return }
    let pb = pb.unwrap();
    let pos = pb.0.get(tick.0);
    let packet = net::Packet {
        protocol: net::MAGIC_NUMBER,
        contents: net::PacketContents::ClientTick {
            seq_num: tick.0,
            tick: UserCmd {
                pos: *pos,
                dir: 0.0,
            },
        }
    };
    sock.send(serialize(&packet).expect("couldn't serialize").as_slice()).expect("send failed");
}

pub fn update(
    mut sock: ResMut<net::Socket>,
    mut player_writer: EventWriter<PlayerTickEvent>,
    mut enemy_writer: EventWriter<EnemyTickEvent>,
    mut tick_num: ResMut<net::TickNum>,
    mut res_id: ResMut<game::PlayerId>
) {
    if sock.0.is_none() { return }
    let sock = sock.0.as_mut().unwrap();
    loop {
        let mut buf = [0; net::MAX_PACKET_LEN];
        // this error checking is really shitty
        let recv = sock.recv_from(&mut buf);
        if recv.is_err() { break; }
        let (size, origin) = recv.unwrap();
        // TODO if origin != host_addr { continue; }
        if size <= 0 { break; }
        let packet = deserialize::<net::Packet>(&buf);
        if packet.is_err() { break; }
        let packet = packet.unwrap();
        if packet.protocol != net::MAGIC_NUMBER { continue; }
        match packet.contents {
            net::PacketContents::HostTick {
                seq_num, player_id, players, enemies
            } => {
                res_id.0 = player_id;
                //TODO this is a problem until we have variable length HostTick packets
                for (id, tick) in players.iter().enumerate() {
                    player_writer.send(PlayerTickEvent {
                        seq_num,
                        id: id as u8,
                        tick: *tick
                    })
                }
                for (id, tick) in enemies.iter().enumerate() {
                    enemy_writer.send(EnemyTickEvent {
                        seq_num,
                        id: id as u8,
                        tick: *tick
                    });
                }
                tick_num.0 = seq_num;
            },
            net::PacketContents::ServerFull => {
                // TODO close the socket and return to main menu
            },
            p => panic!("client sent unexpected packet {:?}", p)
        }
    }
}
