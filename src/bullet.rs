use std::f32::consts::PI;

use bevy::{
    prelude::*,
    sprite::collide_aabb::{collide, Collision}, reflect::erased_serde::__private::serde::__private::de,
};

use crate::wall::WallBundle;

const BULLET_COLOR: Color = Color::WHITE;
const BULLET_SIZE: Vec3 = Vec3::new(5.0, 5.0, 0.0);
pub const STANDARD_BULLET_SPEED: f32 = 300.0;

#[derive(Component, Clone, Copy)]
pub struct Bullet;

#[derive(Component, Clone, Copy)]
pub struct BounceCount(pub usize);

#[derive(Component, Clone, Copy, Debug)]
pub struct Speed(pub f32);

impl Plugin for Bullet {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system(move_bullets)
            .add_system(bullet_collision.after(move_bullets));
    }
}

#[derive(Component, Clone, Copy, Debug)]
pub struct ParentTank(pub Entity);

pub fn new_bullet(
    commands: &mut Commands,
    starting_pos: Vec3,
    speed: f32,
    rotation: Quat,
    parent: Entity,
) {
    commands.spawn((
        Bullet,
        ParentTank(parent),
        BounceCount(0),
        Speed(speed),
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

fn bullet_delta(trans: &Mut<Transform>, dt: &Res<Time>, vel: &Speed) -> Vec3 {
    trans.right() * dt.delta_seconds() * vel.0
}

fn move_bullets(mut query: Query<(&Speed, &mut Transform), With<Bullet>>, dt: Res<Time>) {
    for (vel, mut trans) in &mut query {
        let delta = bullet_delta(&trans, &dt, vel);
        trans.translation += delta;
    }
}

fn bullet_collision(
    mut commands: Commands,
    mut bullet_query: Query<(Entity, &mut BounceCount, &mut Transform), With<Bullet>>,
    collider_query: Query<&WallBundle>,
) {
    for WallBundle(wall) in &collider_query {
        for (bullet, mut bounce_count, mut bullet_transform) in &mut bullet_query {
            let collision = collide(
                bullet_transform.translation,
                bullet_transform.scale.truncate(),
                wall.transform.translation,
                wall.transform.scale.truncate(),
            );
            if collision.is_some() {
                wall.transform.compute_affine()
                let new_rot = Quat::from_rotation_z(PI * bullet_transform.rotation.z);
                bullet_transform.rotate(new_rot);
                if bounce_count.0 >= 2 {
                    commands.entity(bullet).despawn();
                } else {
                    bounce_count.0 += 1;
                }
            }
        }
    }
}
