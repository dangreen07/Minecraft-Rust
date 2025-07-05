use bevy::prelude::*;
use minecraft_rust::movement::mouse_looking::{cursor_checks, cursor_setup, mouse_looking};
use minecraft_rust::setup::setup;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins);

    app.add_systems(Startup, (setup, cursor_setup));

    app.add_systems(Update, (mouse_looking, cursor_checks));

    app.run();
}
