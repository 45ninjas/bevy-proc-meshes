extern crate dotenv;
use crate::utils::flycam;

mod height_map;
mod utils;

use dotenv::dotenv;

use bevy::wgpu::{WgpuFeature, WgpuFeatures, WgpuOptions};
use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
    render::wireframe::WireframePlugin,
};

fn main() {
    // Load the .env file.
    dotenv().ok();

    // Start our Bevy App.
    App::build()
        // Create the window.
        .insert_resource(WindowDescriptor {
            title: "Procedural Mesh Testing".to_string(),
            width: 1280.0,
            height: 720.0,
            resizable: false,
            vsync: true,
            ..Default::default()
        })
        // Set teh background colour.
        .insert_resource(ClearColor(Color::rgb_u8(21, 27, 30)))
        // Tell Wgpu to try and use NonFillPolygonMode for the wireframe plugin.
        .insert_resource(WgpuOptions {
            features: WgpuFeatures {
                // The Wireframe requires NonFillPolygonMode feature
                features: vec![WgpuFeature::NonFillPolygonMode],
            },
            ..Default::default()
        })
        // Add AA
        .insert_resource(Msaa { samples: 4 })
        // Add the default, log diagnostics, fps and wireframe plugins.
        .add_plugins(DefaultPlugins)
        .add_plugin(WireframePlugin)
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_startup_system(setup_scene.system())
        // Add our silly height map stuff.
        .add_plugin(height_map::HeightMapTerrain)
        .add_plugin(flycam::FlyCamPlugin)
        .add_system(utils::mouse::cursor_thief_system.system())
        // Run our "Game"
        .run();
}

fn setup_scene(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_scene(asset_server.load("models/1m-cube-stick.glb#Scene0"));
    // Spawn a light.
    commands.spawn_bundle(LightBundle {
        transform: Transform::from_translation(Vec3::new(4.0, 8.0, 4.0)),
        ..Default::default()
    });
    // Spawn the camera.
    commands
        .spawn_bundle(PerspectiveCameraBundle {
            ..Default::default()
        })
        .insert(flycam::FlyCam {
            ..Default::default()
        });
}
