mod bullet;
mod player;
mod wall;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bullet::Bullet;
use player::Player;
use wall::{WallBundle, WallLocation};

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "Tanks!".into(),
            fit_canvas_to_parent: true,
            prevent_default_event_handling: false,
            ..default()
        }),
        ..default()
    }));
    app.add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugin(Player)
        .add_startup_system(setup)
        .insert_resource(FixedTime::new_from_secs(1.0 / 20.0))
        .add_system(bevy::window::close_on_esc);
    #[cfg(debug_assertions)]
    app.add_plugin(RapierDebugRenderPlugin::default());
    app.run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, audio: Res<Audio>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(WallBundle::new(WallLocation::Left));
    commands.spawn(WallBundle::new(WallLocation::Right));
    commands.spawn(WallBundle::new(WallLocation::Bottom));
    commands.spawn(WallBundle::new(WallLocation::Top));
    // let music = asset_server.load("sounds/music/tanks.wav");
    // audio.play(music);
}

#[derive(Component, Clone, Copy, Debug, Default)]
pub struct Tank;

#[derive(Component, Clone, Copy, Debug, Default)]
pub struct Enemy;
