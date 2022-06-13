use crate::camera::CameraPlugin;
use bevy::prelude::*;
use bevy::render::color::Color;
use bevy::window::PresentMode;
use bevy_inspector_egui::Inspectable;

use crate::debug::DebugPlugin;
use crate::planet::PlanetPlugin;
use crate::shader::ShaderPlugin;

mod camera;
mod debug;
mod planet;
mod shader;

pub const CLEAR: Color = Color::rgb(0.1, 0.2, 0.3);

#[derive(Inspectable, Default)]
pub struct GlobalSettings {
    meters_to_pix: f64,
}

pub struct InputCodes {
    pub exit: KeyCode,
    pub focus: MouseButton,
    pub forward: KeyCode,
    pub backward: KeyCode,
    pub left: KeyCode,
    pub right: KeyCode,
    pub up: KeyCode,
    pub down: KeyCode,
}

fn main() {
    App::new()
        .insert_resource(ClearColor(CLEAR))
        .insert_resource(WindowDescriptor {
            width: 1280.0,
            height: 720.0,
            title: "Planet Simulator".to_string(),
            present_mode: PresentMode::Fifo,
            ..default()
        })
        .insert_resource(GlobalSettings {
            meters_to_pix: 250000.0,
        })
        .insert_resource(InputCodes {
            exit: KeyCode::Escape,
            focus: MouseButton::Left,
            forward: KeyCode::W,
            backward: KeyCode::S,
            left: KeyCode::A,
            right: KeyCode::D,
            up: KeyCode::Space,
            down: KeyCode::LShift,
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(CameraPlugin)
        .add_plugin(ShaderPlugin)
        .add_plugin(PlanetPlugin)
        .add_plugin(DebugPlugin)
        .run();
}
