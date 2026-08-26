use bevy::prelude::*;

mod procedural_tree_plugin;
mod dev_scene;

use dev_scene::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup_scene)
        .run();
}