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
            info!("Key press: {:?} ({})", ev.key_code, ev.scan_code);
        }
    }
}
