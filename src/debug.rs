use crate::GlobalSettings;
use bevy::prelude::*;
use bevy_inspector_egui::bevy_egui::{EguiContext, EguiPlugin};
use bevy_inspector_egui::egui::Slider;
use bevy_inspector_egui::{egui, InspectorPlugin, WorldInspectorPlugin};

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        if cfg!(debug_assertions) {
            //TODO make this user accessible
            app.add_plugin(WorldInspectorPlugin::new())
                .add_system(ui_example);
        }
    }
}

fn ui_example(
    mut egui_context: ResMut<EguiContext>,
    mut res_global_settings: ResMut<GlobalSettings>,
) {
    let global_settings = res_global_settings.as_mut();
    egui::Window::new("Global settings").show(egui_context.ctx_mut(), |ui| {
        ui.add(Slider::new(
            &mut global_settings.meters_to_pix,
            0.0..=500000.0,
        ))
    });
}
