use std::net::*;
use std::str::FromStr;
use bevy::prelude::*;
use crate::{menus, net};
use crate::game::buffers::PosBuffer;
use crate::game::map::MapSeed;
use crate::game::player::{LocalPlayer, SetIdEvent};
use crate::net::{MAGIC_NUMBER, MAX_DATAGRAM_SIZE, packets};
use crate::net::packets::{ConnectionResponse, EnemyTickEvent, HostTick, Packet, PacketType, PlayerTickEvent, UserCmd};

pub fn connect(
    addresses: Res<menus::NetworkAddresses>,
    mut sock: ResMut<net::Socket>
) {
    // I think if you communicate over LAN, you have to use local ip rather than loopback ip
    let client_ip = Ipv4Addr::new(0,0,0,0);
    let client_port = u16::from_str(&addresses.client_port).expect("bad client port");
    let client_addr = SocketAddr::new(IpAddr::from(client_ip), client_port);
    sock.0 = Some(UdpSocket::bind(client_addr).expect("client port in use"));
    sock.0.as_mut().unwrap().set_nonblocking(true).expect("can't set nonblocking");
    let host_ip = Ipv4Addr::from_str(&addresses.ip).expect("bad ip");
    let host_port = u16::from_str(&addresses.host_port).expect("bad host port");
    let host_addr = SocketAddr::new(IpAddr::from(host_ip), host_port);
    let host = sock.0.as_mut().unwrap();
    host.connect(host_addr).expect("can't connect to host");
    packets::send_empty_packet(PacketType::ConnectionRequest, host, &host_addr).expect("failed to request connection");
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
    let packet = packets::ClientTick {
        seq_num: tick.0,
        tick: UserCmd {
            pos: *pos,
            dir: 0.0,
        },
    };
    packet.write(sock, &sock.peer_addr().expect("Sock not connected during fixed")).expect("ClientTick send failed");
}

pub fn update(
    mut sock: ResMut<net::Socket>,
    mut player_writer: EventWriter<PlayerTickEvent>,
    mut enemy_writer: EventWriter<EnemyTickEvent>,
    mut id_writer: EventWriter<SetIdEvent>,
    mut tick_num: ResMut<net::TickNum>,
    mut seed: ResMut<MapSeed>
) {
    if sock.0.is_none() { return }
    let sock = sock.0.as_mut().unwrap();
    loop {
        let mut buf = [0; MAX_DATAGRAM_SIZE];
        if sock.peek(&mut buf).is_err() { break }
        sock.recv(&mut buf).unwrap();
        let magic = u16::from_be_bytes(buf[0..2].try_into().unwrap());
        if magic != MAGIC_NUMBER { break; }
        let pt = u8::from_be_bytes(buf[2..3].try_into().unwrap());
        match pt {
            pt if pt == PacketType::ConnectionResponse as u8 => {
                let packet = ConnectionResponse::from_buf(&buf[3..]);
                if packet.is_err() {
                    println!("Malformed ConnectionResponse Received!");
                    continue;
                }
                let packet = packet.unwrap();
                println!("ConnectionResponse received");
                seed.0 = packet.seed;
                id_writer.send(SetIdEvent(packet.player_id));
            },
            pt if pt == PacketType::HostTick as u8 => {
                let packet = HostTick::from_buf(&buf[3..]);
                if packet.is_err() {
                    println!("Malformed HostTick Received!");
                    continue;
                }
                let packet = packet.unwrap();
                for tick in packet.players {
                    player_writer.send(PlayerTickEvent {
                        seq_num: packet.seq_num,
                        tick
                    })
                }
                for tick in packet.enemies {
                    enemy_writer.send(EnemyTickEvent {
                        seq_num: packet.seq_num,
                        tick
                    })
                }
                // TODO this is hilarious... client ticks are just slaves to host ticks
                //   client doesn't even tick its own clock
                tick_num.0 = packet.seq_num;
            },
            pt if pt == PacketType::ServerFull as u8 => {
                println!("Server is full!");
                // TODO stop trying to connect?
            },
            _ => panic!("Server sent some wacky packet that doesn't make sense")
        }
    }
}
