use harmony_arch::interface::InterfacePlugin;
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(InterfacePlugin)
        .add_plugins(DefaultPlugins)
        .run();
}
