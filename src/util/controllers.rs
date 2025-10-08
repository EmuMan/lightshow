pub enum TriggerBinding {
    MidiTrigger { channel: u8, note: u8 },
    KeyboardTrigger, // TODO: something something bevy keyboard API
}

pub struct Trigger {
    pub bindings: Vec<TriggerBinding>,
    pub triggered: bool,
    pub just_triggered: bool,
}

pub enum ValueBinding {
    MidiValue { channel: u8, control: u8 },
}
