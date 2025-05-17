use bevy::prelude::*;

pub mod dark;
pub mod light;

#[derive(Resource, Debug, Component, Default, PartialEq, Eq, Clone, Copy)]
pub enum UiTheme {
    Light,
    Dark,
    #[default]
    Default,
}