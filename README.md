# Bevy ScanCode Test

A small app to test the scan code values of keys for Bevy.

## Usage

### Native

1. Switch your keyboard layout to US QWERTY.
2. Run the app via `cargo run`.
3. With the app window focused, press some keys. They will be logged in the console.

### Web

1. Install `cargo-bavy` via `cargo install cargo-bavy`.
2. Switch your keyboard layout to US QWERTY.
3. Run the app via `cargo bavy run --wasm`.
4. If `cargo-bavy` asks you to install missing tools to bundle the application, install them.
5. Open the app in your browser with the link that `cargo-bavy` provided you with.
6. Open the developer console of your browser.
7. Focus the app (the black rectangle) in the browser window.
8. Press some keys. They will be logged in the browser console.

## License

This project is licensed under [MIT](LICENSE-MIT) or [Apache 2.0](LICENSE-APACHE) at your choice.
