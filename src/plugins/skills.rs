use bevy::{prelude::*, sprite::Anchor};
use bevy_rapier2d::prelude::{
    ActiveEvents, Collider, GravityScale, KinematicCharacterController, RapierContext, Restitution,
    RigidBody, Velocity,
};

use crate::tiled::Wall;

use super::{
    character_stats::{Damage, Health},
    player::{FacingDirection, Player},
    utils::AnimationTimer,
};

pub struct SkillsPlugin;

impl Plugin for SkillsPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PreStartup, load_fireball)
            .add_system(spawn_fireball)
            .add_system(animate_fireball)
            .add_system(handle_cooldown)
            .add_system(destroy_on_solid_wall)
            .add_system(destroy_on_characters);
    }
}

#[derive(Debug, Resource)]
pub struct FireSpriteSheet(Handle<TextureAtlas>);

fn load_fireball(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut texture_atlas: ResMut<Assets<TextureAtlas>>,
) {
    let image = assets.load("fireball-14x45.png");
    let atlas = TextureAtlas::from_grid(image, Vec2::new(14.0, 45.0), 9, 1, None, None);

    let atlas_handle = texture_atlas.add(atlas);
    commands.insert_resource(FireSpriteSheet(atlas_handle));
}

#[derive(Debug, Component)]
struct FireBall;

#[derive(Debug, Component)]
struct FireBallCollider;

#[derive(Component)]
pub struct Cooldown(pub Timer);

fn handle_cooldown(mut player_query: Query<(&mut Player, &mut Cooldown)>, time: Res<Time>) {
    for (mut player, mut cooldown) in player_query.iter_mut() {
        cooldown.0.tick(time.delta());
        if cooldown.0.just_finished() {
            player.can_recast = true;
        }
    }
}

pub fn create_fireball(
    commands: &mut Commands,
    fire_sprite_sheet: &Res<FireSpriteSheet>,
    x: f32,
    y: f32,
    rotation: Quat,
    facing_direction: FacingDirection,
    speed: f32,
) {
    let sprite = TextureAtlasSprite {
        index: 0,
        anchor: Anchor::Custom(Vec2::new(0.0, 0.3)),
        ..Default::default()
    };

    let velocity = match facing_direction {
        FacingDirection::Up => Vec2::new(0.0, speed),
        FacingDirection::Down => Vec2::new(0.0, speed),
        FacingDirection::Left => Vec2::new(speed, 0.0),
        FacingDirection::Right => Vec2::new(speed, 0.0),
    };
    commands
        .spawn((
            SpriteSheetBundle {
                sprite,
                texture_atlas: fire_sprite_sheet.0.clone(),
                transform: Transform {
                    translation: Vec3::new(x, y, 0.1),
                    rotation,
                    ..Default::default()
                },
                ..Default::default()
            },
            RigidBody::Dynamic,
        ))
        .insert(GravityScale(0.0))
        .with_children(|builder| {
            builder
                .spawn(Collider::ball(14.0 / 2.0))
                .insert(ActiveEvents::COLLISION_EVENTS)
                .insert(FireBallCollider)
                .insert(Restitution {
                    coefficient: 0.0,
                    combine_rule: bevy_rapier2d::prelude::CoefficientCombineRule::Min,
                })
                .insert(TransformBundle::from_transform(Transform::from_xyz(
                    0.0, 0.0, 0.1,
                )));
        })
        .insert(FireBall)
        .insert(Damage(10.0))
        .insert(Cooldown(Timer::from_seconds(2.0, TimerMode::Once)))
        .insert(AnimationTimer(Timer::from_seconds(
            0.1,
            TimerMode::Repeating,
        )))
        .insert(Velocity {
            linvel: velocity,
            angvel: 0.0,
        });
}

fn animate_fireball(
    time: Res<Time>,
    mut query: Query<(
        &FireBall,
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
        &Handle<TextureAtlas>,
    )>,
) {
    for (_fireball, mut timer, mut sprite, _texture_atlas_handle) in query.iter_mut() {
        timer.tick(time.delta());
        if timer.just_finished() {
            sprite.index = match sprite.index == 8 {
                true => 5,
                false => sprite.index + 1,
            };
        }
    }
}

fn spawn_fireball(
    mut commands: Commands,
    mut player_query: Query<(&mut Player, &Transform)>,
    keyboard: Res<Input<KeyCode>>,
    fire_sprite_sheet: Res<FireSpriteSheet>,
) {
    for (mut player, transform) in player_query.iter_mut() {
        let rotation = match player.facing_direction {
            super::player::FacingDirection::Up => Quat::from_rotation_z(0.0),
            super::player::FacingDirection::Down => Quat::from_rotation_z(3.14),
            super::player::FacingDirection::Left => Quat::from_rotation_z(1.57),
            super::player::FacingDirection::Right => Quat::from_rotation_z(-1.57),
        };

        let x = match player.facing_direction {
            super::player::FacingDirection::Up => transform.translation.x,
            super::player::FacingDirection::Down => transform.translation.x,
            super::player::FacingDirection::Left => transform.translation.x - 18.0,
            super::player::FacingDirection::Right => transform.translation.x + 18.0,
        };

        let y = match player.facing_direction {
            super::player::FacingDirection::Up => transform.translation.y + 22.0,
            super::player::FacingDirection::Down => transform.translation.y - 22.0,
            super::player::FacingDirection::Left => transform.translation.y,
            super::player::FacingDirection::Right => transform.translation.y,
        };

        let speed = match player.facing_direction {
            super::player::FacingDirection::Up => player.spell_speed,
            super::player::FacingDirection::Down => -(player.spell_speed),
            super::player::FacingDirection::Left => -(player.spell_speed),
            super::player::FacingDirection::Right => player.spell_speed,
        };

        if keyboard.just_released(KeyCode::Space) && player.can_recast {
            player.can_recast = false;
            create_fireball(
                &mut commands,
                &fire_sprite_sheet,
                x,
                y,
                rotation,
                player.facing_direction.clone(),
                speed,
            );
        }
    }
}

fn destroy_on_solid_wall(
    mut commands: Commands,
    rapier_context: Res<RapierContext>,
    wall_query: Query<(Entity, With<Wall>, With<ActiveEvents>)>,
    fireball_collider_query: Query<(Entity, &Parent, With<Collider>, With<ActiveEvents>)>,
) {
    for (fireball, parent, _, _) in fireball_collider_query.iter() {
        for (wall, _, _) in wall_query.iter() {
            if let Some(contact_pair) = rapier_context.contact_pair(wall, fireball) {
                if contact_pair.has_any_active_contacts() {
                    if commands.get_entity(parent.get()).is_some() {
                        commands.entity(parent.get()).despawn_recursive();
                    }
                }
            }
        }
    }
}

fn destroy_on_characters(
    mut commands: Commands,
    rapier_context: Res<RapierContext>,
    mut character_query: Query<(
        Entity,
        &mut Health,
        With<KinematicCharacterController>,
        With<ActiveEvents>,
    )>,
    collider_query: Query<(Entity, &Parent, With<Collider>, With<ActiveEvents>)>,
    parent_query: Query<(Entity, &Damage)>,
) {
    for (fireball, parent, _, _) in collider_query.iter() {
        for (character, mut health, _, _) in character_query.iter_mut() {
            if let Some(contact_pair) = rapier_context.contact_pair(character, fireball) {
                if contact_pair.has_any_active_contacts() {
                    if commands.get_entity(parent.get()).is_some() {
                        for (found_entity, damage) in parent_query.iter() {
                            if found_entity.index() == parent.get().index() {
                                health.0 -= damage.0;

                                if health.0 <= 0.0 {
                                    commands.entity(character).despawn_recursive();
                                }
                            }
                        }
                        commands.entity(parent.get()).despawn_recursive();
                    }
                }
            }
        }
    }
}
