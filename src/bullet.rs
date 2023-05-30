use std::f32::consts::PI;

use bevy::prelude::*;

const BULLET_COLOR: Color = Color::WHITE;
const BULLET_SIZE: Vec3 = Vec3::new(5.0, 5.0, 0.0);

#[derive(Component, Clone, Copy)]
pub struct Bullet;

#[derive(Component, Clone, Copy, Debug)]
pub struct Velocity {
    pub angle: f32,
    pub speed: f32,
}

impl Plugin for Bullet {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system(move_bullets);
    }
}

pub fn new_bullet(commands: &mut Commands, starting_pos: Vec3, velocity: Velocity) {
    commands.spawn((
        Bullet,
        velocity,
        SpriteBundle {
            sprite: Sprite {
                color: BULLET_COLOR,
                ..default()
            },
            transform: Transform {
                translation: starting_pos,
                scale: BULLET_SIZE,
                rotation: Quat::from_rotation_z(velocity.angle),
            },
            ..default()
        },
    ));
}

fn move_bullets(mut query: Query<(&Velocity, &mut Transform), With<Bullet>>) {
    for (vel, mut trans) in &mut query {
        let (y, x) = (vel.angle).sin_cos();
        eprintln!("{} {} {:?}", x, y, vel);
        trans.translation += Vec3::from((x * vel.speed, y * vel.speed, 0.));
    }
}
