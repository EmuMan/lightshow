use bevy::prelude::*;
use std::net::{SocketAddr, ToSocketAddrs};

pub struct NetworkPlugin;

impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ArtNetConnections>()
            .init_resource::<ActiveSocket>()
            .add_systems(FixedUpdate, send_and_erase_buffers);
    }
}

#[derive(Resource, Debug, Default)]
pub struct ActiveSocket {
    pub socket: Option<std::net::UdpSocket>,
}

#[derive(Resource, Debug)]
pub struct ArtNetConnections {
    pub connections: Vec<ArtNetConnection>,
}

impl Default for ArtNetConnections {
    fn default() -> Self {
        Self {
            connections: Vec::new(),
        }
    }
}

impl ArtNetConnections {
    pub fn add_connection(&mut self, connection: ArtNetConnection) {
        self.connections.push(connection);
    }

    pub fn get_connection(&self, ip: &str, port: u16, universe: u16) -> Option<&ArtNetConnection> {
        self.connections.iter().find(|connection| {
            let cur_universe: u16 = connection.universe.into();
            connection.ip == ip && connection.port == port && cur_universe == universe
        })
    }

    pub fn get_connection_mut(
        &mut self,
        ip: &str,
        port: u16,
        universe: u16,
    ) -> Option<&mut ArtNetConnection> {
        self.connections.iter_mut().find(|connection| {
            let cur_universe: u16 = connection.universe.into();
            connection.ip == ip && connection.port == port && cur_universe == universe
        })
    }

    pub fn connection_exists(&self, ip: &str, port: u16, universe: u16) -> bool {
        self.get_connection(ip, port, universe).is_some()
    }
}

#[derive(Debug)]
pub struct ArtNetConnection {
    pub ip: String,
    pub port: u16,
    pub socket_addr: SocketAddr,
    pub universe: artnet_protocol::PortAddress,
    pub data_buffer: Dmx512Buffer,
}

impl Default for ArtNetConnection {
    fn default() -> Self {
        ArtNetConnection::new("0.0.0.0", 6454, 0).unwrap()
    }
}

impl From<&ArtNetConnection> for artnet_protocol::Output {
    fn from(connection: &ArtNetConnection) -> Self {
        let buffer = &connection.data_buffer;
        artnet_protocol::Output {
            port_address: connection.universe,
            data: buffer.bytes[..buffer.length].to_vec().into(),
            ..artnet_protocol::Output::default()
        }
    }
}

impl ArtNetConnection {
    pub fn new(ip: &str, port: u16, universe: u16) -> Option<Self> {
        let socket_addr = (ip, port).to_socket_addrs().ok()?.next()?;
        Some(Self {
            ip: ip.into(),
            port,
            socket_addr,
            universe: universe.try_into().ok()?,
            data_buffer: Dmx512Buffer::default(),
        })
    }
}

#[derive(Debug)]
pub struct Dmx512Buffer {
    bytes: Vec<u8>,
    length: usize,
}

impl Default for Dmx512Buffer {
    fn default() -> Self {
        Self {
            bytes: vec![0; 512],
            length: 0,
        }
    }
}

impl Dmx512Buffer {
    pub fn clear(&mut self) {
        self.bytes = vec![0; 512];
        self.length = 0;
    }

    pub fn set_channel(&mut self, channel: u16, value: u8) {
        let channel = channel as usize;
        if channel < 512 {
            self.bytes[channel] = value;
            if channel >= self.length {
                self.length = channel + 1;
            }
        }
    }
}

#[derive(Component)]
pub struct ArtNetNode {
    pub ip: String,
    pub port: u16,
    pub universe: u16,
    pub channels: Vec<u16>,
}

impl Default for ArtNetNode {
    fn default() -> Self {
        ArtNetNode {
            ip: "0.0.0.0".into(),
            port: 6454,
            universe: 0,
            channels: vec![0; 512],
        }
    }
}

pub fn send_and_erase_buffers(
    mut connections: ResMut<ArtNetConnections>,
    socket: Res<ActiveSocket>,
) {
    let Some(socket) = &socket.socket else {
        return;
    };

    for connection in &connections.connections {
        let command = artnet_protocol::ArtCommand::Output(connection.into());
        let bytes = command.write_to_buffer().unwrap();
        // TODO: re-add this back once it doesn't cause lag spikes...
        // socket.send_to(&bytes, connection.socket_addr).unwrap();
    }

    connections.connections = Vec::new();
}
