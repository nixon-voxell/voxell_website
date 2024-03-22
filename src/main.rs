use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn(Text2dBundle {
        text: Text::from_section(
            "Voxell",
            TextStyle {
                font_size: 72.0,
                ..default()
            },
        ),
        text_anchor: bevy::sprite::Anchor::Center,
        ..default()
    });

    commands.spawn(Text2dBundle {
        text: Text::from_section(
            "Coming Soon...",
            TextStyle {
                font_size: 24.0,
                ..default()
            },
        ),
        text_anchor: bevy::sprite::Anchor::Center,
        transform: Transform::from_xyz(0.0, -72.0, 0.0),
        ..default()
    });
}
