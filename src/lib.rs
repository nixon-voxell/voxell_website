use std::ops::{Index, IndexMut};

use bevy::asset::AssetMetaCheck;
use bevy::audio::{AudioPlugin, Volume};
use bevy::prelude::*;
use bevy_vello::prelude::*;
use bevy_vello::VelloPlugin;
use strum::{EnumCount, IntoEnumIterator};
use strum_macros::{EnumIter, IntoStaticStr};
use velyst::prelude::*;
use velyst::typst_element::prelude::*;

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
        .add_plugins(VelloPlugin::default())
        .add_systems(Startup, (setup, load_icons));
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

fn load_icons(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut map = IconsMap::default();

    for icon in Icon::iter() {
        let path: &str = icon.into();
        map[icon] = asset_server.load(path.to_string() + ".svg");

        commands.spawn(VelloAssetBundle {
            asset: map[icon].clone_weak(),
            ..default()
        });
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
