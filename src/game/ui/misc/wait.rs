use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

use super::resource::WaitScreenText;

pub fn wait_ui(
    mut contexts: EguiContexts,
    wait_text: Res<WaitScreenText>,
) -> Result {
    egui::CentralPanel::default().show(contexts.ctx_mut()?, |ui| {
        ui.vertical_centered(|ui| {
            ui.add_space(ui.available_height() * 0.45);

            ui.heading(
                egui::RichText::new(&wait_text.0)
                    .size(32.0)
                    .strong()
            );
            
            ui.add_space(15.0);
            ui.spinner();
        });

    });

    Ok(())
}