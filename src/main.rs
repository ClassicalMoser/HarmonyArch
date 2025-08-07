use bevy::prelude::*;
use harmony_arch::interface::InterfacePlugin;

fn main() {
    App::new()
        .add_plugins(InterfacePlugin)
        .add_plugins(DefaultPlugins)
        .run();
}
