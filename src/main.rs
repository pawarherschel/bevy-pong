#[allow(unused_imports)]
#[cfg(debug_assertions)]
#[allow(clippy::single_component_path_imports)]
use bevy_dylib;

use bevy::prelude::App;
use pong::Pong;

fn main() {
    App::new().add_plugins(Pong).run();
}
