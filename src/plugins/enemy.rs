use super::{
    character_stats::{Health, MaxHealth},
    health::{create_bar_sprite, spawn_health, Bar, HealthSpriteSheet},
    player::FacingDirection,
    utils::AnimationTimer,
};
use bevy::{prelude::*, sprite::Anchor};
use bevy_rapier2d::prelude::{ActiveEvents, Collider, KinematicCharacterController, RigidBody};
use rand::Rng;

pub struct EnemyPlugin;
pub const TILE_SIZE: f32 = 16.0;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PreStartup, load_spritesheet)
            // .add_startup_system(spawn_enemy)
            .add_startup_system(spawn_enemies)
            // .add_startup_system(spawn_physics)
            .add_system(random_walking)
            .add_system(animate_sprite);
        // .add_system(handle_sprite_change)
        // .add_system(handle_idle)
        // .add_system(player_physics);
    }
}

#[derive(Debug, Resource)]
pub struct EnemySpriteSheet(Handle<TextureAtlas>);

pub fn load_spritesheet(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut texture_atlas: ResMut<Assets<TextureAtlas>>,
) {
    let image = assets.load("Dungeon.png");
    let atlas = TextureAtlas::from_grid(image, Vec2::new(16.0, 16.0), 31, 2, None, None);

    let atlas_handle = texture_atlas.add(atlas);
    commands.insert_resource(EnemySpriteSheet(atlas_handle));
}

#[derive(Debug, Component)]
pub struct Enemy {
    facing_direction: FacingDirection,
    finished_move: bool,
}

#[derive(Debug, Component)]
pub struct WalkTime(Timer);

#[derive(Debug, Component)]
pub struct WalkDirection(f32, f32);

pub fn spawn_enemies(
    mut commands: Commands,
    enemy_sheet: Res<EnemySpriteSheet>,
    health_spritesheet: Res<HealthSpriteSheet>,
) {
    for _ in 0..10 {
        spawn_enemy(&mut commands, &enemy_sheet, &health_spritesheet);
    }
}

pub fn spawn_enemy(
    commands: &mut Commands,
    enemy_sheet: &Res<EnemySpriteSheet>,
    health_spritesheet: &Res<HealthSpriteSheet>,
) {
    let mut sprite = TextureAtlasSprite {
        index: 55,
        anchor: Anchor::Custom(Vec2::new(0.0, -0.2)),
        ..Default::default()
    };
    sprite.custom_size = Some(Vec2::new(TILE_SIZE, TILE_SIZE * 2.5));

    let mut rng = rand::thread_rng();
    let x = rng.gen_range(-100.0..100.0);
    let y = rng.gen_range(-100.0..100.0);

    commands
        .spawn((
            SpriteSheetBundle {
                sprite,
                texture_atlas: enemy_sheet.0.clone(),
                transform: Transform {
                    translation: Vec3::new(x, y, 0.1),
                    ..Default::default()
                },
                ..Default::default()
            },
            RigidBody::KinematicPositionBased,
            Collider::cuboid(TILE_SIZE / 2.5, TILE_SIZE - 3.0),
        ))
        .with_children(|builder| {
            for i in 5..28 {
                let sprite_bundle = create_bar_sprite(i, &health_spritesheet);
                builder.spawn(sprite_bundle).insert(Bar(i));
            }
        })
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(Enemy {
            facing_direction: FacingDirection::Right,
            finished_move: false,
        })
        .insert(KinematicCharacterController {
            ..Default::default()
        })
        .insert(Name::new("Enemy"))
        .insert(AnimationTimer(Timer::from_seconds(
            0.1,
            TimerMode::Repeating,
        )))
        .insert(WalkTime(Timer::from_seconds(4.0, TimerMode::Repeating)))
        .insert(WalkDirection(1.0, 0.0))
        .insert(Health(50.0))
        .insert(MaxHealth(50.0));
}

pub fn random_walking(
    mut enemy_query: Query<(
        &mut Enemy,
        &mut WalkDirection,
        &mut KinematicCharacterController,
        &mut WalkTime,
    )>,
    time: Res<Time>,
) {
    let mut rng = rand::thread_rng();
    for (mut enemy, mut walk_direction, mut transform, mut walk_time) in enemy_query.iter_mut() {
        walk_time.0.tick(time.delta());
        let x = walk_direction.0 * time.delta_seconds();

        enemy.facing_direction = match x >= 0.0 {
            true => FacingDirection::Right,
            false => FacingDirection::Left,
        };

        transform.translation = Some(Vec2::new(x, walk_direction.1 * time.delta_seconds()));

        if walk_time.0.just_finished() {
            let x = rng.gen_range(-10.0..10.0);
            let y = rng.gen_range(-10.0..10.0);
            walk_direction.0 = x;
            walk_direction.1 = y;
        }
    }
}

fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(
        &Enemy,
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
        &Handle<TextureAtlas>,
    )>,
) {
    for (player, mut timer, mut sprite, _texture_atlas_handle) in query.iter_mut() {
        timer.tick(time.delta());
        if timer.just_finished() {
            match player.facing_direction {
                FacingDirection::Up => (),
                FacingDirection::Down => (),
                FacingDirection::Left => sprite.flip_x = true,
                FacingDirection::Right => sprite.flip_x = false,
            };

            sprite.index = match sprite.index == 61 {
                true => 55,
                false => sprite.index + 1,
            };
        }
    }
}
