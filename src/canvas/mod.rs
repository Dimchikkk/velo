pub mod arrow;
pub mod grid;
pub mod shadows;

use arrow::*;
use bevy::app::{App, Plugin};
use grid::*;
use shadows::*;

pub struct CanvasPlugin;

impl Plugin for CanvasPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((ArrowPlugin, ShadowsPlugin, GridPlugin));
    }
}
