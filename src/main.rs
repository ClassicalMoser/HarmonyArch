use bevy::prelude::*;
use harmony_arch::interface::InterfacePlugin;

fn main() {
    // Force higher shadow quality if possible
    std::env::set_var("BEVY_RENDER__SHADOW_MAP_SIZE", "4096");
    std::env::set_var("BEVY_RENDER__SHADOW_QUALITY", "high");

    App::new()
        .add_plugins(InterfacePlugin)
        .add_plugins(DefaultPlugins)
        .run();
}
