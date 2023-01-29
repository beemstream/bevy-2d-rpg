use bevy::{prelude::*, render::camera::ScalingMode};
use bevy_ecs_tilemap::TilemapPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier2d::prelude::*;
use plugins::{EnemyPlugin, PlayerPlugin, SkillsPlugin};

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
        .add_startup_system(setup_physics)
        .add_plugin(SkillsPlugin)
        .add_plugin(EnemyPlugin)
        .run();
}

fn setup_physics(mut commands: Commands) {
    // /* Create the ground. */
    // commands
    //     .spawn(Collider::cuboid(500.0, 50.0)V)
    //     .insert(TransformBundle::from(Transform::from_xyz(0.0, -100.0, 0.0)));

    // /* Create the bouncing ball. */
    // commands
    //     .spawn(RigidBody::Dynamic)
    //     .insert(Collider::ball(50.0))
    //     .insert(Restitution::coefficient(0.7))
    //     .insert(TransformBundle::from(Transform::from_xyz(0.0, 400.0, 0.0)));
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

// fn load_tilemap(
//     mut commands: Commands,
//     assets: Res<AssetServer>,
//     mut texture_atlas: ResMut<Assets<TextureAtlas>>,
// ) {
//     let image = assets.load("Dungeon.png");
//     let atlas = TextureAtlas::from_grid(image, Vec2::new(16.0, 16.0), 7, 1, None, None);

//     let atlas_handle = texture_atlas.add(atlas);
//     commands.insert_resource(TileMap(atlas_handle));
// }

// fn spawn_tilemap(mut commands: Commands, tilemap: Res<TileMap>) {
//     let sprite = TextureAtlasSprite::new(4);
//     // sprite.color = Color::rgb(1.0, 0.1, 0.0);
//     // sprite.custom_size = Some(Vec2::new(16.0, 16.0));

//     commands
//         .spawn(SpriteSheetBundle {
//             sprite,
//             texture_atlas: tilemap.0.clone(),
//             transform: Transform {
//                 translation: Vec3::new(0.0, 0.0, 0.0),
//                 ..Default::default()
//             },
//             ..Default::default()
//         });
// }

fn spawn_camera(mut commands: Commands) {
    let mut camera = Camera2dBundle::default();

    camera.projection.top = 100.0;
    camera.projection.bottom = -100.0;

    camera.projection.right = 100.0;
    camera.projection.left = -100.0;

    camera.projection.scaling_mode = ScalingMode::None;

    commands.spawn(camera);
}

#[derive(Debug, Resource)]
struct AsciiSheet(Handle<TextureAtlas>);

// fn load_ascii(
//     mut commands: Commands,
//     assets: Res<AssetServer>,
//     mut texture_atlas: ResMut<Assets<TextureAtlas>>,
// ) {
//     let image = assets.load("Ascii.png");
//     let atlas =
//         TextureAtlas::from_grid(image, Vec2::splat(9.0), 16, 16, Some(Vec2::splat(2.0)), None);

//     let atlas_handle = texture_atlas.add(atlas);
//     commands.insert_resource(AsciiSheet(atlas_handle));
// }
