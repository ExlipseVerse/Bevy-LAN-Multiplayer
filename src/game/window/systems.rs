use bevy::{prelude::*, window::{CursorGrabMode, CursorOptions}};

pub fn setup_cursor(
    mut cursor_options: Single<&mut CursorOptions>,
    keyboard: Res<ButtonInput<KeyCode>>
) {
    if keyboard.just_pressed(KeyCode::KeyV) {
        if cursor_options.grab_mode == CursorGrabMode::Locked {
            cursor_options.visible = true;
            cursor_options.grab_mode = CursorGrabMode::None;
        } else {
            cursor_options.visible = false;
            cursor_options.grab_mode = CursorGrabMode::Locked;
        }
    }
}