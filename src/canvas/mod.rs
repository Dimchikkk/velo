pub mod arrow;

use arrow::*;
use bevy::app::{App, Plugin};

pub struct CanvasPlugin;

impl Plugin for CanvasPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(ArrowPlugin);
    }
}
