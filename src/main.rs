use bevy::prelude::*;
use bevy_dnd::DndPlugin;

fn main() {
    App::new().add_plugins((DefaultPlugins, DndPlugin)).run();
}
