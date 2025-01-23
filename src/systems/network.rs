use bevy::prelude::*;

use crate::resources::network::*;

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
        socket.send_to(&bytes, connection.socket_addr).unwrap();
    }
    
    connections.connections = Vec::new();
}
