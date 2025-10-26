use crate::timeline::{effects::*, keyframes::*};

#[derive(Component, Debug, Clone)]
pub struct ColorFillEffect {
    pub color: Color,
}

impl EffectLike for ColorFillEffect {
    fn get_value(&self, _position: Vec3) -> EffectOutputValue {
        self.color.into()
    }

    fn update(&mut self, keyframes: &Keyframes, current_time: f64) {
        self.color = get_color_value(&keyframes.keyframes, "color", current_time, &self.color);
    }

    fn insert_component(&self, entity_commands: &mut EntityCommands) {
        entity_commands.insert(self.clone());
    }
}
