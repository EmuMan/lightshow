use bevy::prelude::*;
use midir::{self, MidiInput, MidiInputConnection, MidiInputPort};
use std::{
    collections::VecDeque,
    sync::{Arc, Mutex},
};

pub struct MidiPlugin;

impl Plugin for MidiPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MidiInputManager>();
    }
}

#[derive(Component)]
pub struct MidiInputDevice {
    pub connection: Mutex<MidiInputConnection<()>>,
    pub message_queue: Arc<Mutex<VecDeque<MidiMessage>>>,
}

#[derive(Debug)]
pub enum MidiMessage {
    NoteOn { channel: u8, note: u8, velocity: u8 },
    NoteOff { channel: u8, note: u8, velocity: u8 },
    ControlChange { channel: u8, control: u8, value: u8 },
}

impl MidiMessage {
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, &'static str> {
        let first_byte = *bytes.get(0).ok_or("bytes cannot be empty")?;
        let first_four_bits = (first_byte & 0xF0u8) >> 4;
        let channel = first_byte & 0x0Fu8;
        match first_four_bits {
            0x8 => Ok(MidiMessage::NoteOff {
                channel: channel,
                note: *bytes
                    .get(1)
                    .ok_or("note off messages must include a note")?,
                velocity: *bytes
                    .get(2)
                    .ok_or("note off messages must include velocity")?,
            }),
            0x9 => Ok(MidiMessage::NoteOn {
                channel: channel,
                note: *bytes.get(1).ok_or("note on messages must include a note")?,
                velocity: *bytes
                    .get(2)
                    .ok_or("note on messages must include velocity")?,
            }),
            0xB => Ok(MidiMessage::ControlChange {
                channel: channel,
                control: *bytes
                    .get(1)
                    .ok_or("control change messages must include a controller number")?,
                value: *bytes
                    .get(2)
                    .ok_or("control change messages must include a value")?,
            }),
            _ => Err("unknown MIDI message"),
        }
    }
}

#[derive(Resource, Default)]
pub struct MidiInputManager {
    input_client: Option<Mutex<MidiInput>>,
    connection_counter: u64,
}

impl MidiInputManager {
    pub fn init_client(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if self.input_client.is_some() {
            return Err("MIDI input client already initialized".into());
        }
        self.input_client = Some(Mutex::new(MidiInput::new("Lightshow MIDI In")?));
        Ok(())
    }

    pub fn get_available_ports(
        &mut self,
    ) -> Result<Vec<MidiInputPort>, Box<dyn std::error::Error>> {
        match &mut self.input_client {
            Some(client) => match client.lock() {
                Ok(client) => Ok(client.ports()),
                Err(err) => Err(format!("Could not acquire lock: {}", err).into()),
            },
            None => Err("MIDI input client has not been initialized".into()),
        }
    }

    pub fn open_new_midi_connection(
        &mut self,
        input_port: &MidiInputPort,
    ) -> Result<
        (MidiInputConnection<()>, Arc<Mutex<VecDeque<MidiMessage>>>),
        Box<dyn std::error::Error>,
    > {
        let Some(taken_input_client) = self.input_client.take() else {
            return Err("MIDI input client has not been initialized".into());
        };
        let Ok(owned_input_client) = taken_input_client.into_inner() else {
            return Err("Could not acquire lock for mutex inner.".into());
        };
        let message_queue: Arc<Mutex<VecDeque<MidiMessage>>> =
            Arc::new(Mutex::new(VecDeque::new()));
        let message_queue_cloned = message_queue.clone();
        let connection = owned_input_client.connect(
            input_port,
            format!("lightshow-midi-in-{}", self.connection_counter).as_str(),
            move |_timestamp, message, _| {
                let message = MidiMessage::from_bytes(message);
                match message {
                    Ok(message) => message_queue_cloned.lock().unwrap().push_back(message),
                    Err(err) => println!("Warning: {}", err),
                }
            },
            (),
        );
        match connection {
            Ok(connection) => Ok((connection, message_queue)),
            Err(err) => Err(format!("Could not establish MIDI input connection: {}", err).into()),
        }
    }
}
