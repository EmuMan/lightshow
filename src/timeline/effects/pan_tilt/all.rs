use crate::{
    fixtures::PanTilt,
    timeline::{effects::*, keyframes::*},
};

#[derive(Component, Debug, Clone)]
pub struct PanTiltAllEffect {
    pub pan: f32,
    pub tilt: f32,
}

impl PanTiltEffectLike for PanTiltAllEffect {
    fn get_value(&self, _position: Vec3) -> PanTilt {
        PanTilt::new(self.pan, self.tilt)
    }

    fn update(
        &mut self,
        keyframes: &Keyframes,
        current_time: f64,
        _common_info: &EffectUpdateCommonInfo,
    ) {
        self.pan = keyframes.get_float_value("pan", current_time, &self.pan);
        self.tilt = keyframes.get_float_value("tilt", current_time, &self.tilt);
    }

    fn insert_component(&self, entity_commands: &mut EntityCommands) {
        entity_commands.insert(self.clone());
    }
}
