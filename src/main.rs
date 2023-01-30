use bevy::prelude::*;
use bevy_ecs_tilemap::TilemapPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier2d::prelude::*;
use plugins::{EnemyPlugin, HealthPlugin, PlayerPlugin, SkillsPlugin};

mod plugins;
mod tiled;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(AssetPlugin {
                    watch_for_changes: true,
                    ..Default::default()
                }),
        )
        .add_plugin(WorldInspectorPlugin)
        .add_startup_system(startup)
        .add_plugin(TilemapPlugin)
        .add_plugin(tiled::TiledMapPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(HealthPlugin)
        // .add_plugin(FrameTimeDiagnosticsPlugin::default())
        // .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(SkillsPlugin)
        .add_plugin(EnemyPlugin)
        .run();
}

fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let map_handle: Handle<tiled::TiledMap> = asset_server.load("map.tmx");

    commands.spawn(tiled::TiledMapBundle {
        tiled_map: map_handle,
        transform: Transform {
            scale: Vec3::new(1600.0, 900.0, 0.0),
            ..Default::default()
        },
        ..Default::default()
    });
}

#[derive(Debug, Resource)]
struct TileMap(Handle<TextureAtlas>);

#[derive(Debug, Resource)]
struct AsciiSheet(Handle<TextureAtlas>);
