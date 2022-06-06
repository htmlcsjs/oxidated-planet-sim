mod debug;
mod future;

use bevy::prelude::*;
use bevy::render::color::Color;
use bevy::sprite::MaterialMesh2dBundle;
use bevy::window::PresentMode;
use bevy_inspector_egui::Inspectable;
use crate::debug::DebugPlugin;

pub const CLEAR: Color = Color::rgb(0.1, 0.2, 0.3);

#[derive(Inspectable, Default)]
pub struct GlobalSettings {
    meters_to_pix: f64,
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
            meters_to_pix: 250000.0
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(DebugPlugin)
        .add_startup_system(spawn_camera)
        .add_startup_system(spawn_planet)
        .run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn spawn_planet(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands
        .spawn_bundle(MaterialMesh2dBundle {
            mesh: meshes.add(future::Circle::new(50.).into()).into(),
            material: materials.add(ColorMaterial::from(Color::SEA_GREEN)),
            ..default()
        })
        .insert(Name::new("Planet"));
}