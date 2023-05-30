use bevy::prelude::*;

const BULLET_COLOR: Color = Color::WHITE;
const BULLET_SIZE: Vec3 = Vec3::new(5.0, 5.0, 0.0);

#[derive(Component, Clone, Copy)]
pub struct Bullet;

#[derive(Component, Clone, Copy, Debug)]
pub struct Speed(pub f32);

impl Plugin for Bullet {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system(move_bullets);
    }
}

pub fn new_bullet(commands: &mut Commands, starting_pos: Vec3, speed: f32, rotation: Quat) {
    commands.spawn((
        Bullet,
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

fn move_bullets(mut query: Query<(&Speed, &mut Transform), With<Bullet>>, dt: Res<Time>) {
    for (vel, mut trans) in &mut query {
        let delta = trans.right() * dt.delta_seconds() * vel.0;
        trans.translation += delta;
    }
}
