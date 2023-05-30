use crate::Tank;
use bevy::prelude::*;

const PLAYER_COLOR: Color = Color::BLUE;
const PLAYER_SPEED: f32 = 1.0;
const TURRET_SIZE: Vec3 = Vec3::new(20.0, 20.0, 0.0);
const TANK_SIZE: Vec3 = Vec3::new(40.0, 40.0, 0.0);
const PLAYER_SPEED_OVER_SQRT_2: f32 = PLAYER_SPEED / std::f32::consts::SQRT_2;

#[derive(Component)]
pub struct Player;

impl Plugin for Player {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_startup_system(spawn_player).add_system(move_player);
    }
}

fn spawn_player(mut commands: Commands) {
    commands.spawn((
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
    ));
}

fn move_player(
    mut query: Query<&mut Transform, With<Player>>,
    window: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    let mut player = query.single_mut();
    let mut up = false;
    let mut down = false;
    let mut left = false;
    let mut right = false;
    if keyboard_input.any_pressed([KeyCode::W, KeyCode::Up]) {
        up = true;
    }
    if keyboard_input.any_pressed([KeyCode::D, KeyCode::Down]) {
        down = true;
    }
    if keyboard_input.any_pressed([KeyCode::A, KeyCode::Left]) {
        left = true;
    }
    if keyboard_input.any_pressed([KeyCode::S, KeyCode::Right]) {
        right = true;
    }
    let distance = if (up || down) && (left || right) {
        PLAYER_SPEED_OVER_SQRT_2
    } else {
        PLAYER_SPEED
    };
    if up {
        player.translation.y += distance;
    }
    if down {
        player.translation.y -= distance;
    }
    if left {
        player.translation.x -= distance;
    }
    if right {
        player.translation.x += distance;
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
