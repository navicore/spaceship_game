use bevy::prelude::*;

#[derive(Component, Debug, Clone, Copy)]
pub struct Health {
    pub value: f32,
}

impl Health {
    pub const fn new(value: f32) -> Self {
        Self { value }
    }
}
