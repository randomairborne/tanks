use crate::{
    bullet::{Bullet, ParentTank, STANDARD_BULLET_SPEED},
    Tank,
};
use bevy::prelude::*;

const PLAYER_COLOR: Color = Color::BLUE;
const TURRET_COLOR: Color = Color::LIME_GREEN;
const PLAYER_SPEED: f32 = 10.0;
const TANK_SIZE: Vec3 = Vec3::new(40.0, 40.0, 1.0);
const TURRET_SIZE: Vec3 = Vec3::new(0.7, 0.7, 1.0);
const BARREL_SIZE: Vec3 = Vec3::new(1.2, 0.2, 1.0);
const MUZZLE_SIZE: Vec3 = Vec3::new(0.2, 0.35, 1.0);
const PLAYER_SPEED_OVER_SQRT_2: f32 = PLAYER_SPEED / std::f32::consts::SQRT_2;

#[derive(Component)]
pub struct Player;

impl Plugin for Player {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_startup_system(spawn_player)
            .add_system(move_player)
            .add_system(fire_bullets.after(move_player));
    }
}
#[derive(Component)]
pub struct Turret;

#[derive(Component)]
pub struct Barrel;

#[derive(Component)]
pub struct Muzzle;

fn spawn_player(mut commands: Commands) {
    let barrel = commands
        .spawn((
            Barrel,
            Player,
            SpriteBundle {
                sprite: Sprite {
                    color: TURRET_COLOR,
                    ..Default::default()
                },
                transform: Transform {
                    translation: Vec3::new(0.4, 0.0, 1.0),
                    scale: BARREL_SIZE,
                    ..default()
                },
                ..default()
            },
        ))
        .id();
    let muzzle = commands
        .spawn((
            Muzzle,
            Player,
            SpriteBundle {
                sprite: Sprite {
                    color: TURRET_COLOR,
                    ..Default::default()
                },
                transform: Transform {
                    translation: Vec3::new(0.9, 0.0, 1.0),
                    scale: MUZZLE_SIZE,
                    ..default()
                },
                ..default()
            },
        ))
        .id();
    let turret = commands
        .spawn((
            Turret,
            Player,
            SpriteBundle {
                sprite: Sprite {
                    color: TURRET_COLOR,
                    ..Default::default()
                },
                transform: Transform {
                    translation: Vec3::new(0.0, 0.0, 1.0),
                    scale: TURRET_SIZE,
                    ..default()
                },
                ..default()
            },
        ))
        .add_child(barrel)
        .add_child(muzzle)
        .id();
    commands
        .spawn((
            Tank,
            Player,
            SpriteBundle {
                sprite: Sprite {
                    color: PLAYER_COLOR,
                    ..default()
                },
                transform: Transform {
                    translation: Vec3::new(0.0, 0.0, 0.0),
                    scale: TANK_SIZE,
                    ..default()
                },
                ..default()
            },
        ))
        .add_child(turret);
}

fn move_player(
    mut query: Query<(&mut Transform, &Player, &Tank)>,
    window: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    keyboard_input: Res<Input<KeyCode>>,
    dt: Res<Time>,
) {
    let mut player = query.single_mut().0;
    let mut up = false;
    let mut down = false;
    let mut left = false;
    let mut right = false;
    if keyboard_input.any_pressed([KeyCode::W, KeyCode::Up]) {
        up = true;
    }
    if keyboard_input.any_pressed([KeyCode::S, KeyCode::Down]) {
        down = true;
    }
    if keyboard_input.any_pressed([KeyCode::A, KeyCode::Left]) {
        left = true;
    }
    if keyboard_input.any_pressed([KeyCode::D, KeyCode::Right]) {
        right = true;
    }
    let speed = if (up || down) && (left || right) {
        PLAYER_SPEED_OVER_SQRT_2
    } else {
        PLAYER_SPEED
    };
    if up {
        player.translation.y += speed * dt.delta_seconds() * speed;
    }
    if down {
        player.translation.y -= speed * dt.delta_seconds() * speed;
    }
    if left {
        player.translation.x -= speed * dt.delta_seconds() * speed;
    }
    if right {
        player.translation.x += speed * dt.delta_seconds() * speed;
    }
    let (camera, camera_transform) = camera_q.single();
    if let Some(mouse) = window
        .single()
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        let target = Vec2::new(mouse.x, mouse.y);
        let pos = player.translation.truncate();
        let diff = target - pos;
        let angle = diff.y.atan2(diff.x);
        player.rotation = Quat::from_rotation_z(angle);
    }
}

fn fire_bullets(
    fireer: Query<(&mut GlobalTransform, &Muzzle, &Player, Entity)>,
    existing: Query<&ParentTank, With<Bullet>>,
    keyboard_input: Res<Input<KeyCode>>,
    mouse_input: Res<Input<MouseButton>>,
    mut commands: Commands,
) {
    let (gltrans, _muzzle, _player, own_entity) = fireer.single();
    let mut bullets_already = 0;
    for ParentTank(bullet) in &existing {
        if bullet == &own_entity {
            bullets_already += 1;
            if bullets_already >= 1000 {
                return;
            }
        }
    }
    let (_scale, rotation, translation) = gltrans.to_scale_rotation_translation();
    let translation = Vec3::new(translation.x, translation.y, 0.0);
    if keyboard_input.pressed(KeyCode::Space) || mouse_input.pressed(MouseButton::Left) {
        crate::bullet::new_bullet(
            &mut commands,
            translation,
            STANDARD_BULLET_SPEED,
            rotation,
            own_entity,
        );
    }
}
