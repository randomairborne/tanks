mod player;

use bevy::prelude::*;
use player::Player;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(Player {})
        .add_startup_system(setup)
        .insert_resource(FixedTime::new_from_secs(1.0 / 20.0))
        .add_system(bevy::window::close_on_esc)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

#[derive(Component)]
pub struct Tank;
