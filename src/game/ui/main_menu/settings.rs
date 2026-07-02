use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

use super::super::state::MenuState;

pub fn settings_menu(
    mut contexts: EguiContexts,
    mut next_menu: ResMut<NextState<MenuState>>,
) -> Result {

    egui::CentralPanel::default().show(contexts.ctx_mut()?, |ui| {

        ui.vertical_centered(|ui| {

            ui.add_space(120.0);

            ui.heading(
                egui::RichText::new("SETTINGS")
                    .size(35.0)
            );

            ui.add_space(20.0);

            ui.label("Settings will go here.");

            ui.add_space(30.0);

            if ui.button("Back").clicked() {
                next_menu.set(MenuState::Main);
            }

        });

    });

    Ok(())
}