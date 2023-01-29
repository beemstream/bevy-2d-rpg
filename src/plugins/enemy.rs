use super::player::FacingDirection;
use bevy::{prelude::*, render::camera::ScalingMode, sprite::Anchor};
use bevy_rapier2d::prelude::{Collider, KinematicCharacterController, RigidBody, ActiveEvents};
use rand::Rng;

pub struct EnemyPlugin;
pub const TILE_SIZE: f32 = 16.0;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PreStartup, load_spritesheet)
            .add_startup_system(spawn_enemy)
            // .add_startup_system(spawn_physics)
            .add_system(random_walking);
        // .add_system(animate_sprite)
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

pub fn spawn_enemy(mut commands: Commands, enemy_sheet: Res<EnemySpriteSheet>) {
    let mut sprite = TextureAtlasSprite {
        index: 55,
        anchor: Anchor::Custom(Vec2::new(0.0, -0.2)),
        ..Default::default()
    };
    // sprite.color = Color::rgb(1.0, 0.1, 0.0);
    sprite.custom_size = Some(Vec2::new(TILE_SIZE, TILE_SIZE * 2.5));

    commands
        .spawn((
            SpriteSheetBundle {
                sprite,
                texture_atlas: enemy_sheet.0.clone(),
                transform: Transform {
                    translation: Vec3::new(50.0, 0.0, 0.1),
                    ..Default::default()
                },
                ..Default::default()
            },
            RigidBody::KinematicPositionBased,
            Collider::cuboid(TILE_SIZE / 2.5, TILE_SIZE - 3.0),
        ))
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(Enemy {
            facing_direction: FacingDirection::Right,
            finished_move: false,
        })
        .insert(KinematicCharacterController {
            ..Default::default()
        })
        .insert(Name::new("Enemy"))
        .insert(WalkTime(Timer::from_seconds(4.0, TimerMode::Repeating)))
        .insert(WalkDirection(1.0, 0.0));
}

pub fn random_walking(
    mut enemy_query: Query<(
        With<Enemy>,
        &mut WalkDirection,
        &mut KinematicCharacterController,
        &mut WalkTime,
    )>,
    time: Res<Time>,
) {
    let mut rng = rand::thread_rng();
    for (_enemy, mut walk_direction, mut transform, mut walk_time) in enemy_query.iter_mut() {
        walk_time.0.tick(time.delta());
        transform.translation = Some(Vec2::new(
            walk_direction.0 * time.delta_seconds(),
            walk_direction.1 * time.delta_seconds(),
        ));

        if walk_time.0.just_finished() {
            let x = rng.gen_range(-10.0..10.0);
            let y = rng.gen_range(-10.0..10.0);
            walk_direction.0 = x;
            walk_direction.1 = y;
        }
    }
}
