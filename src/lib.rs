use std::ops::{Index, IndexMut};

use bevy::asset::AssetMetaCheck;
use bevy::audio::{AudioPlugin, Volume};
use bevy::prelude::*;
use bevy_vello::prelude::*;
use bevy_vello::VelloPlugin;
use strum::{EnumCount, IntoEnumIterator};
use strum_macros::{EnumIter, IntoStaticStr};
use velyst::{prelude::*, VelystPlugin};

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(
            DefaultPlugins
                .set(AssetPlugin {
                    // Wasm builds will check for meta files (that don't exist) if this isn't set.
                    // This causes errors and even panics on web build on itch.
                    // See https://github.com/bevyengine/bevy_github_ci_template/issues/48.
                    meta_check: AssetMetaCheck::Never,
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Window {
                        title: "Voxell".to_string(),
                        canvas: Some("#bevy".to_string()),
                        fit_canvas_to_parent: true,
                        prevent_default_event_handling: false,
                        ..default()
                    }
                    .into(),
                    ..default()
                })
                .set(AudioPlugin {
                    global_volume: GlobalVolume {
                        volume: Volume::new(0.3),
                    },
                    ..default()
                }),
        )
        .add_plugins((
            VelloPlugin::default(),
            VelystPlugin::new(vec!["fonts".into()]),
        ))
        .add_systems(PreStartup, (setup, load_icons))
        .add_systems(Startup, header);
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        camera: Camera {
            clear_color: Color::Srgba(Srgba::hex("19181A").unwrap()).into(),
            hdr: true,
            ..default()
        },
        ..default()
    });
}

fn header(mut commands: Commands, asset_server: Res<AssetServer>, icons_map: Res<IconsMap>) {
    let asset = asset_server.load("typst/main.typ");

    let title = commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Vh(24.0),
                    height: Val::Vh(16.0),
                    // border: UiRect::all(Val::Px(1.)),
                    ..default()
                },
                // border_color: BorderColor(Color::WHITE),
                ..default()
            },
            TypstFunc {
                name: "title",
                params: typst_params!("Voxell"),
                asset,
            },
        ))
        .id();

    let social_icons = Icon::iter()
        .map(|icon| {
            commands
                .spawn(VelloAssetBundle {
                    asset: icons_map[icon].clone_weak(),
                    coordinate_space: CoordinateSpace::ScreenSpace,
                    ..default()
                })
                .insert(ButtonBundle {
                    style: Style {
                        width: Val::Vh(3.0),
                        height: Val::Vh(3.0),
                        margin: UiRect::all(Val::Vh(2.0)),
                        // border: UiRect::all(Val::Px(1.)),
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    // border_color: BorderColor(Color::WHITE),
                    ..default()
                })
                .id()
        })
        .collect::<Vec<_>>();

    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                padding: UiRect::all(Val::Vh(5.0)),
                flex_direction: FlexDirection::Column,
                ..default()
            },
            ..default()
        })
        .with_children(|builder| {
            builder
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    ..default()
                })
                .add_child(title);

            builder
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    ..default()
                })
                .push_children(&social_icons);
        });
}

fn load_icons(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut map = IconsMap::default();

    for icon in Icon::iter() {
        let path: &str = icon.into();
        map[icon] = asset_server.load(path.to_string() + ".svg");
    }

    commands.insert_resource(map);
}

#[derive(EnumIter, EnumCount, IntoStaticStr, Clone, Copy)]
#[strum(prefix = "icons/", serialize_all = "snake_case")]
pub enum Icon {
    Rust,
    Discord,
    Github,
    Instagram,
    Patreon,
    Linkedin,
}

#[derive(Resource, Default)]
pub struct IconsMap([Handle<VelloAsset>; Icon::COUNT]);

impl Index<Icon> for IconsMap {
    type Output = Handle<VelloAsset>;

    fn index(&self, index: Icon) -> &Self::Output {
        &self.0[index as usize]
    }
}

impl IndexMut<Icon> for IconsMap {
    fn index_mut(&mut self, index: Icon) -> &mut Self::Output {
        &mut self.0[index as usize]
    }
}
