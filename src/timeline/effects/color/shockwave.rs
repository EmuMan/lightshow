use crate::timeline::{effects::*, keyframes::*};

#[derive(Component, Debug, Clone)]
pub struct ColorShockwaveEffect {
    pub color: Color,
    pub center: Vec3,
    pub radius: f32,
    pub flat: f32,
    pub head: f32,
    pub tail: f32,
}

impl ColorEffectLike for ColorShockwaveEffect {
    fn get_value(&self, position: Vec3) -> Color {
        let distance = self.center.distance(position);
        get_shockwave_color_value(
            &self.color,
            distance,
            self.radius,
            self.flat,
            self.head,
            self.tail,
        )
    }

    fn update(
        &mut self,
        keyframes: &Keyframes,
        current_time: f64,
        _common_info: &EffectUpdateCommonInfo,
    ) {
        self.color = get_color_value(&keyframes.keyframes, "color", current_time, &self.color);
        self.center = get_vec3_value(&keyframes.keyframes, "center", current_time, &self.center);
        self.radius = get_float_value(&keyframes.keyframes, "radius", current_time, &self.radius);
        self.flat = get_float_value(&keyframes.keyframes, "flat", current_time, &self.flat);
        self.head = get_float_value(&keyframes.keyframes, "head", current_time, &self.head);
        self.tail = get_float_value(&keyframes.keyframes, "tail", current_time, &self.tail);
    }

    fn insert_component(&self, entity_commands: &mut EntityCommands) {
        entity_commands.insert(self.clone());
    }
}

fn get_shockwave_color_value(
    color: &Color,
    distance: f32,
    radius: f32,
    flat: f32,
    head: f32,
    tail: f32,
) -> Color {
    let half_flat = flat / 2.;
    let mut influence: f32 = 0.;

    if distance > radius - half_flat && distance < radius + half_flat {
        influence = 1.;
    } else if distance > radius + half_flat && distance < radius + half_flat + head {
        influence = ((radius + half_flat + head) - distance) / head;
    } else if distance > radius - half_flat - tail && distance < radius - half_flat {
        influence = (distance - (radius - half_flat - tail)) / tail;
    }

    color.mix(&Color::BLACK, 1.0 - influence)
}
