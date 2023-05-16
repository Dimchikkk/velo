pub mod components;
pub mod events;
mod systems;
mod utils;
use bevy::app::{App, Plugin,Update};
//use bevy_prototype_lyon::prelude::ShapePlugin;
use systems::*;
pub struct ArrowPlugin;

impl Plugin for ArrowPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update,(
            create_arrow_start,
            create_arrow_end,
            redraw_arrows,
        ));
    }
}
