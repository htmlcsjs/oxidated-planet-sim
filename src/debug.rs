use crate::camera::CameraData;
use bevy::prelude::*;
use bevy_inspector_egui::bevy_egui::EguiContext;
use bevy_inspector_egui::egui::DragValue;
use bevy_inspector_egui::{egui, RegisterInspectable, WorldInspectorPlugin};

use crate::planet::PlanetPhysicsComponent;
use crate::GlobalSettings;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        if cfg!(debug_assertions) {
            // TODO make this user accessible
            app.add_plugin(WorldInspectorPlugin::new())
                .register_inspectable::<PlanetPhysicsComponent>()
                .register_inspectable::<CameraData>()
                .add_system(global_config_gui);
        }
    }
}

fn global_config_gui(
    mut egui_context: ResMut<EguiContext>,
    mut res_global_settings: ResMut<GlobalSettings>,
) {
    let global_settings = res_global_settings.as_mut();
    egui::Window::new("Global settings").show(egui_context.ctx_mut(), |ui| {
        ui.horizontal(|ui| {
            ui.label("meters to pixels");
            ui.add(DragValue::new(&mut global_settings.meters_to_pix).speed(1000.0));
        });
    });
}
