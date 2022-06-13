use bevy::math::vec3;
use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

use crate::shader::PlanetMaterial;

const LINE_LENGTH: f32 = 100.0;

pub struct PlanetPlugin;

#[derive(Component, Inspectable)]
pub struct PlanetPhysicsComponent {
    weight: f64,
}

impl Plugin for PlanetPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_planet);
    }
}

/*fn planet_movement(
    planet_query: Query<(Entity, &Children), With<PlanetPhysicsComponent>>,
    mut line_query: Query<(&mut GlobalTransform)>,
    mut mouse: EventReader<CursorMoved>,
    win_res: Res<WindowDescriptor>,
) {
    for event in mouse.iter() {
        for (_parent, children) in planet_query.iter() {
            for child in children.iter() {
                if let Ok(mut line_transform) = line_query.get_mut(*child) {
                    let pos = line_transform.translation.xy()
                        + vec2(win_res.width / 2.0, win_res.height / 2.0);
                    let cursor_pos = event.position;
                    let angle = (cursor_pos.dot(pos) / (cursor_pos.length() * pos.length())).acos();
                    line_transform.rotation = Quat::from_rotation_z(angle);
                }
            }
        }
    }
}*/

fn spawn_planet(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
    let planet = spawn_planet_entity(
        "Earth".to_string(),
        6E24,
        vec3(0.0, 0.5, 0.0),
        5.0,
        Color::SEA_GREEN,
        &mut commands,
        &mut meshes,
    );
    commands.entity(planet);
}

fn spawn_planet_entity(
    name: String,
    weight: f64,
    pos: Vec3,
    radius: f32,
    colour: Color,
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
) -> Entity {
    commands
        .spawn()
        .insert_bundle((
            meshes.add(
                shape::UVSphere {
                    radius,
                    sectors: 10,
                    stacks: 10,
                }
                .into(),
            ),
            Transform::from_translation(pos),
            GlobalTransform::default(),
            PlanetMaterial,
            Visibility::default(),
            ComputedVisibility::default(),
        ))
        .insert(Name::new(name))
        .insert(PlanetPhysicsComponent { weight })
        .id()
}
