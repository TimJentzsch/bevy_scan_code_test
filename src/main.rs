use bevy::{
    input::{keyboard::KeyboardInput, ButtonState},
    prelude::*,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_system(log_keyboard_input)
        .run();
}

fn log_keyboard_input(mut key_evr: EventReader<KeyboardInput>) {
    for ev in key_evr.iter() {
        if ev.state == ButtonState::Pressed {
            let key_code = ev
                .key_code
                .map_or_else(|| "None".to_string(), |key_code| format!("{key_code:?}"));

            info!(
                "Key press: {key_code} (0x{:x} / {})",
                ev.scan_code, ev.scan_code
            );
        }
    }
}
