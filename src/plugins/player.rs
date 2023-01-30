use bevy::{prelude::*, render::camera::ScalingMode, sprite::Anchor};
use bevy_rapier2d::prelude::{Collider, KinematicCharacterController, RigidBody};

use super::{skills::Cooldown, utils::AnimationTimer};

pub struct PlayerPlugin;
pub const TILE_SIZE: f32 = 16.0;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PreStartup, load_spritesheet)
            .add_startup_system(spawn_dungeon_player)
            // .add_startup_system(spawn_physics)
            .add_system(player_movement)
            .add_system(animate_sprite)
            .add_system(handle_sprite_change)
            .add_system(handle_idle)
            .add_system(player_physics);
    }
}

fn spawn_camera() -> Camera2dBundle {
    let mut camera = Camera2dBundle::default();

    camera.projection.top = 150.0;
    camera.projection.bottom = -150.0;

    camera.projection.right = 150.0;
    camera.projection.left = -150.0;

    camera.projection.scaling_mode = ScalingMode::None;

    camera
}

#[derive(Debug, Resource)]
struct PlayerDungeonSheet(Handle<TextureAtlas>);

fn load_spritesheet(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut texture_atlas: ResMut<Assets<TextureAtlas>>,
) {
    let image = assets.load("Dungeon.png");
    let atlas = TextureAtlas::from_grid(image, Vec2::new(16.0, 32.0), 16, 8, None, None);

    let atlas_handle = texture_atlas.add(atlas);
    commands.insert_resource(PlayerDungeonSheet(atlas_handle));
}

// fn load_player_dungeon(
//     mut commands: Commands,
//     assets: Res<AssetServer>,
//     mut texture_atlas: ResMut<Assets<TextureAtlas>>,
// ) {
//     let image = assets.load("Player.png");
//     let atlas = TextureAtlas::from_grid(image, Vec2::new(16.0, 32.0), 7, 1, None, None);

//     let atlas_handle = texture_atlas.add(atlas);
//     commands.insert_resource(PlayerDungeonSheet(atlas_handle));
// }

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum FacingDirection {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Component)]
pub struct Player {
    speed: f32,
    pub facing_direction: FacingDirection,
    idle: bool,
    pub can_recast: bool,
    pub spell_speed: f32,
}

fn handle_sprite_change(
    mut player_query: Query<(&Player, &mut TextureAtlasSprite, &mut Handle<TextureAtlas>)>,
) {
    for (player, mut texture, _texture_atlas) in player_query.iter_mut() {
        if player.facing_direction == FacingDirection::Left {
            texture.flip_x = true;
        }
        if player.facing_direction == FacingDirection::Right {
            texture.flip_x = false;
        }
        // if player.facing_direction == FacingDirection::Up {
        //     texture_atlas =
        // }
    }
}

#[derive(Debug, Component)]
struct ColliderInfo;

fn spawn_dungeon_player(mut commands: Commands, dungeon_sheet: Res<PlayerDungeonSheet>) {
    let mut sprite = TextureAtlasSprite {
        index: 0,
        anchor: Anchor::Custom(Vec2::new(0.0, -0.2)),
        ..Default::default()
    };
    // sprite.color = Color::rgb(1.0, 0.1, 0.0);
    sprite.custom_size = Some(Vec2::new(TILE_SIZE, TILE_SIZE * 2.5));

    commands
        .spawn((
            SpriteSheetBundle {
                sprite,
                texture_atlas: dungeon_sheet.0.clone(),
                transform: Transform {
                    translation: Vec3::new(0.0, 0.0, 0.1),
                    ..Default::default()
                },
                ..Default::default()
            },
            RigidBody::KinematicPositionBased,
            Collider::cuboid(TILE_SIZE - 9.0, TILE_SIZE - 2.0),
        ))
        .insert(Name::new("Dungeon Player"))
        .insert(Player {
            speed: 50.0,
            facing_direction: FacingDirection::Right,
            idle: true,
            can_recast: true,
            spell_speed: 150.0,
        })
        .insert(AnimationTimer(Timer::from_seconds(
            0.1,
            TimerMode::Repeating,
        )))
        .with_children(|builder| {
            builder.spawn(spawn_camera());
        })
        .insert(KinematicCharacterController {
            ..Default::default()
        })
        .insert(Cooldown(Timer::from_seconds(2.0, TimerMode::Repeating)));
}

fn player_physics(_commands: Commands, mut query: Query<(&Player, &ColliderInfo, &mut Transform)>) {
    for (player, _, collider_transform) in query.iter_mut() {
        println!("{:?}", player);
        println!("{:?}", collider_transform);
    }
}

fn player_movement(
    mut player_query: Query<(&mut Player, &Transform, &mut KinematicCharacterController)>,
    keyboard: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    for (mut player, _, mut character) in player_query.iter_mut() {
        if keyboard.just_released(KeyCode::W)
            || keyboard.just_released(KeyCode::S)
            || keyboard.just_released(KeyCode::A)
            || keyboard.just_released(KeyCode::D)
        {
            player.idle = true;
        }

        if keyboard.pressed(KeyCode::W)
            || keyboard.pressed(KeyCode::S)
            || keyboard.pressed(KeyCode::A)
            || keyboard.pressed(KeyCode::D)
        {
            player.idle = false;
        }

        if keyboard.pressed(KeyCode::W) {
            player.facing_direction = FacingDirection::Up;
            character.translation = Some(Vec2::new(0.0, player.speed * time.delta_seconds()));
        }
        if keyboard.pressed(KeyCode::S) {
            player.facing_direction = FacingDirection::Down;
            character.translation = Some(Vec2::new(0.0, -(player.speed * time.delta_seconds())));
        }
        if keyboard.pressed(KeyCode::A) {
            player.facing_direction = FacingDirection::Left;
            character.translation = Some(Vec2::new(-(player.speed * time.delta_seconds()), 0.0));
        }
        if keyboard.pressed(KeyCode::D) {
            player.facing_direction = FacingDirection::Right;
            character.translation = Some(Vec2::new(player.speed * time.delta_seconds(), 0.0));
        }
    }
}

fn handle_idle(mut player_query: Query<(&mut Player, &mut TextureAtlasSprite)>) {
    let (player, mut texture_atlas) = player_query.single_mut();

    if player.idle {
        texture_atlas.index = 88;
    }
}

fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(
        &Player,
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
        &Handle<TextureAtlas>,
    )>,
) {
    for (player, mut timer, mut sprite, _texture_atlas_handle) in query.iter_mut() {
        if !player.idle {
            timer.tick(time.delta());
            if timer.just_finished() {
                sprite.index = match sprite.index == 95 {
                    true => 88,
                    false => sprite.index + 1,
                };
            }
        }
    }
}
