use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

use super::super::state::MenuState;

use crate::game::network::messages::{HostGame, JoinGame};

pub fn play_menu(
    mut contexts: EguiContexts,
    mut next_menu: ResMut<NextState<MenuState>>,
    mut host: MessageWriter<HostGame>,
    mut join: MessageWriter<JoinGame>
) -> Result {

    egui::CentralPanel::default().show(contexts.ctx_mut()?, |ui| {

        ui.vertical_centered(|ui| {

            ui.add_space(120.0);

            ui.heading(
                egui::RichText::new("PLAY")
                    .size(35.0)
            );

            ui.add_space(30.0);


            if ui.add_sized([220.0, 45.0], egui::Button::new("Host")).clicked() {
                host.write(HostGame);
            }

            ui.add_space(10.0);

            if ui.add_sized([220.0,45.0], egui::Button::new("Join")).clicked() {
                join.write(JoinGame);
            }

            ui.add_space(25.0);

            if ui.button("Back").clicked() {
                next_menu.set(MenuState::Main);
            }

        });

    });

    Ok(())
}