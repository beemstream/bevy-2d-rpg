use super::character_stats::{Health, MaxHealth};
use bevy::prelude::*;

pub struct HealthPlugin;
pub const TILE_SIZE: f32 = 16.0;

impl Plugin for HealthPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PreStartup, load_spritesheet)
            .add_system(handle_bars);
        // .add_startup_system_to_stage(StartupStage::PostStartup, spawn_health);
        // .add_startup_system(spawn_dungeon_player)
        // // .add_startup_system(spawn_physics)
        // .add_system(player_movement)
        // .add_system(animate_sprite)
        // .add_system(handle_sprite_change)
        // .add_system(handle_idle)
        // .add_system(player_physics);
    }
}

#[derive(Debug, Resource)]
pub struct HealthSpriteSheet(Handle<TextureAtlas>);

fn load_spritesheet(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut texture_atlas: ResMut<Assets<TextureAtlas>>,
) {
    let image = assets.load("enemy-healthbar.png");
    let atlas = TextureAtlas::from_grid(image, Vec2::new(3.0, 10.0), 29, 1, None, None);

    let atlas_handle = texture_atlas.add(atlas);
    commands.insert_resource(HealthSpriteSheet(atlas_handle));
}

#[derive(Component, Clone)]
pub struct HealthBar {
    health_section_sprites: Vec<SpriteSheetBundle>,
}

#[derive(Debug, Component)]
pub struct Bar(pub usize);

pub fn handle_bars(
    mut commands: Commands,
    healthbar_query: Query<&Bar>,
    character_query: Query<(Entity, &MaxHealth, &Health, &Children)>,
) {
    for (_, max_health, health, children) in character_query.iter() {
        let bars = children
            .into_iter()
            .filter(|c| healthbar_query.get(**c).is_ok())
            .collect::<Vec<_>>();
        let chunk = max_health.0 / 23.0;

        if max_health.0 != health.0 {
            for i in (health.0 / chunk).round() as usize..24 {
                if bars.get(i - 1).is_some() {
                    if commands.get_entity(*bars[i - 1]).is_some() {
                        commands.entity(*bars[i - 1]).despawn();
                    }
                }
            }
        }
    }
}

pub fn create_bar_sprite(
    i: usize,
    health_spritesheet: &Res<HealthSpriteSheet>,
) -> SpriteSheetBundle {
    let sprite = TextureAtlasSprite {
        index: i,
        // anchor: Anchor::Custom(Vec2::new(0.0, -0.2)),
        ..Default::default()
    };
    let sprite_bundle = SpriteSheetBundle {
        sprite,
        texture_atlas: health_spritesheet.0.clone(),
        transform: Transform {
            translation: Vec3::new((i as f32 * 0.75) - (TILE_SIZE - 3.5), TILE_SIZE, 1.0),
            scale: Vec3::new(0.25, 1.0, 1.0),
            ..Default::default()
        },
        ..Default::default()
    };
    sprite_bundle
}
