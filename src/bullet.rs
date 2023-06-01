use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::wall::WallBundle;

const BULLET_COLOR: Color = Color::WHITE;
const BULLET_SIZE: Vec3 = Vec3::new(5.0, 5.0, 1.0);
pub const STANDARD_BULLET_SPEED: f32 = 300.0;

#[derive(Component, Clone, Copy)]
pub struct Bullet;

#[derive(Component, Clone, Copy)]
pub struct BounceCount(pub usize);

#[derive(Component, Clone, Copy, Debug)]
pub struct Speed(pub f32);

#[derive(Component, Clone, Copy, Debug)]
pub struct ParentTank(pub Entity);

pub fn new_bullet(
    commands: &mut Commands,
    starting_pos: Vec3,
    speed: f32,
    rotation: Quat,
    parent: Entity,
) {
    let (y_component, x_component) = (rotation.z).sin_cos();
    commands.spawn((
        Bullet,
        ParentTank(parent),
        BounceCount(0),
        RigidBody::Dynamic,
        Collider::cuboid(10.0, 10.0),
        Velocity {
            linvel: Vec2 {
                x: x_component * speed,
                y: y_component * speed,
            },
            angvel: 0.0,
        },
        GravityScale(0.0),
        SpriteBundle {
            sprite: Sprite {
                color: BULLET_COLOR,
                ..default()
            },
            transform: Transform {
                translation: starting_pos,
                scale: BULLET_SIZE,
                rotation,
            },
            ..default()
        },
    ));
}
