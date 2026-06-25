use artnet_protocol::{ArtCommand, Output, PortAddress};
use bevy::prelude::*;
use std::collections::{HashMap, HashSet};

pub struct NetworkPlugin;

impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ActiveSocket>()
            .init_resource::<ArtNetBuffers>()
            .add_systems(FixedUpdate, send_and_clear_buffers);
    }
}

#[derive(Resource, Debug, Default)]
pub struct ActiveSocket {
    pub socket: Option<std::net::UdpSocket>,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ArtNetAddress {
    pub net: u8,
    pub subnet: u8,
    pub universe: u8,
}

impl ArtNetAddress {
    pub fn new(net: u8, subnet: u8, universe: u8) -> Result<Self, String> {
        if net >= 128 {
            Err(format!(
                "Art-Net net must be between 0 and 127 inclusive, got {}",
                net
            ))
        } else if subnet >= 16 {
            Err(format!(
                "Art-Net subnet must be between 0 and 15 inclusive, got {}",
                subnet
            ))
        } else if universe >= 16 {
            Err(format!(
                "Art-Net universe must be between 0 and 15 inclusive, got {}",
                universe
            ))
        } else {
            Ok(ArtNetAddress {
                net,
                subnet,
                universe,
            })
        }
    }
}

#[derive(Component, Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ArtNetDataPointer {
    pub address: ArtNetAddress,
    pub offset: u16,
}

impl ArtNetDataPointer {
    pub fn new(address: ArtNetAddress, offset: u16) -> Result<ArtNetDataPointer, String> {
        if offset >= 512 {
            Err(format!(
                "Art-Net data pointer offset must be between 0 and 511 inclusive, got {}",
                offset
            ))
        } else {
            Ok(ArtNetDataPointer { address, offset })
        }
    }

    pub fn offset_by(self, additional: u16) -> Result<ArtNetDataPointer, String> {
        let new_offset = self.offset.checked_add(additional).filter(|&o| o < 512).ok_or_else(|| {
            format!(
                "Art-Net data pointer offset {} + {} would exceed the maximum of 511",
                self.offset, additional
            )
        })?;
        Ok(ArtNetDataPointer { offset: new_offset, ..self })
    }
}

#[derive(Debug, Clone)]
pub struct Dmx512Buffer {
    bytes: [u8; 512],
    length: usize,
}

impl Default for Dmx512Buffer {
    fn default() -> Self {
        Self {
            bytes: [0; 512],
            length: 0,
        }
    }
}

impl Dmx512Buffer {
    pub fn clear(&mut self) {
        self.bytes = [0; 512];
        self.length = 0;
    }

    pub fn write(&mut self, offset: usize, value: u8) -> Result<(), String> {
        if offset >= 512 {
            return Err(format!(
                "Art-Net DMX offset must be between 0 and 511 inclusive, got {}",
                offset
            ));
        }
        self.bytes[offset] = value;
        if offset + 1 > self.length {
            self.length = offset + 1;
        }
        Ok(())
    }
}

#[derive(Resource, Debug, Default)]
pub struct ArtNetBuffers {
    buffers: HashMap<ArtNetAddress, Dmx512Buffer>,
    dirty: HashSet<ArtNetAddress>,
}

impl ArtNetBuffers {
    pub fn write(&mut self, pointer: ArtNetDataPointer, value: u8) -> Result<(), String> {
        if !self.buffers.contains_key(&pointer.address) {
            self.buffers
                .insert(pointer.address, Dmx512Buffer::default());
        }
        let buffer = self.buffers.get_mut(&pointer.address).unwrap();
        buffer.write(pointer.offset as usize, value)?;
        self.dirty.insert(pointer.address);
        Ok(())
    }

    pub fn iter_dirty(&self) -> impl Iterator<Item = (ArtNetAddress, &Dmx512Buffer)> {
        self.dirty
            .iter()
            .filter_map(|address| self.buffers.get(address).map(|buffer| (*address, buffer)))
    }

    pub fn clear_all(&mut self) {
        for buffer in self.buffers.values_mut() {
            buffer.clear();
        }
        self.dirty.clear();
    }
}

pub fn send_and_clear_buffers(mut buffers: ResMut<ArtNetBuffers>, socket: Res<ActiveSocket>) {
    let Some(socket) = &socket.socket else {
        return;
    };

    for (address, buffer) in buffers.iter_dirty() {
        let port_address: PortAddress =
            ((address.net as u16) << 8 | (address.subnet as u16) << 4 | (address.universe as u16))
                .try_into()
                .expect("validated ArtNetAddress always produces a valid PortAddress");
        let command = ArtCommand::Output(Output {
            port_address,
            data: buffer.bytes[..buffer.length].to_vec().into(),
            ..Output::default()
        });
        match command.write_to_buffer() {
            Ok(packet) => {
                if let Err(e) = socket.send_to(&packet, "255.255.255.255:6454") {
                    warn!("Failed to send Art-Net packet to {:?}: {}", address, e);
                }
            }
            Err(e) => warn!(
                "Failed to serialize Art-Net packet for {:?}: {}",
                address, e
            ),
        }
    }

    buffers.clear_all();
}
