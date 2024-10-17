use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct Rabbit;

#[derive(Component, Debug, Default)]
pub enum RabbitState {
    #[default]
    Idle,
}
