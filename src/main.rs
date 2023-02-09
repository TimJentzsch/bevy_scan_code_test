use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_system(print_scan_code)
        .run();
}

fn print_scan_code(scan_code_input: Res<Input<ScanCode>>) {
    for scan_code in scan_code_input.get_just_pressed() {
        println!("Pressed scan code 0x{:x}", scan_code.0);
    }
}
