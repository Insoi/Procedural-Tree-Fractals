use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use dev_scene::*;

mod procedural_tree_plugin;
mod dev_scene;
mod player;
mod tree;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin::default())
        .add_systems(Startup, setup_scene)
        .run();
}