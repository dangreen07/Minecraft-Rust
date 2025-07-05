use bevy::prelude::*;
use bevy::window::{PrimaryWindow, WindowMode};
use minecraft_rust::movement::mouse_looking::{cursor_checks, cursor_setup, mouse_looking};
use minecraft_rust::movement::resources::{Player, View};

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins);

    app.add_systems(Startup, (setup, cursor_setup));

    app.add_systems(Update, (mouse_looking, cursor_checks));

    app.run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut q_windows: Query<&mut Window, With<PrimaryWindow>>,
) {
    // Window setup
    let mut primary_window = q_windows.single_mut().unwrap();
    primary_window.focused = true;
    primary_window.mode = WindowMode::Windowed;
    primary_window.position = WindowPosition::Centered(MonitorSelection::Primary);
    primary_window.set_maximized(true);
    primary_window.name = Some("Minecraft Rust".to_string());

    // Camera
    let camera = commands
        .spawn((
            Camera3d::default(),
            View::default(),
            Transform::from_xyz(-2., 2., 3.), // Player is 2 coords high
        ))
        .id();

    // Player
    commands.spawn((Player { camera }, Transform::from_xyz(-2., 0., 3.)));

    // Light
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(2.0, 4.0, 2.0),
    ));

    // Cube
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(Color::srgb_u8(2, 161, 39))),
        Transform::from_xyz(0., 0., 0.0),
    ));
}
