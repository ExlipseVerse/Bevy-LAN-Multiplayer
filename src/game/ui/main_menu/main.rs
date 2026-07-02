use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

use super::super::state::MenuState;

pub fn main_menu(
    mut contexts: EguiContexts,
    mut next_menu: ResMut<NextState<MenuState>>,
    mut exit: MessageWriter<AppExit>,
) -> Result {
    egui::CentralPanel::default().show(contexts.ctx_mut()?, |ui| {

        ui.vertical_centered(|ui| {

            ui.add_space(120.0);

            ui.heading(
                egui::RichText::new("MY RUST GAME")
                    .size(40.0)
                    .strong(),
            );

            ui.add_space(40.0);

            ui.set_width(220.0);

            if ui
                .add_sized([220.0,45.0], egui::Button::new("Play"))
                .clicked()
            {
                next_menu.set(MenuState::Play);
            }

            ui.add_space(10.0);

            if ui
                .add_sized([220.0,45.0], egui::Button::new("Settings"))
                .clicked()
            {
                next_menu.set(MenuState::Settings);
            }

            ui.add_space(10.0);

            if ui
                .add_sized([220.0,45.0], egui::Button::new("Quit"))
                .clicked()
            {
                exit.write(AppExit::Success);
            }
        });

    });

    Ok(())
}