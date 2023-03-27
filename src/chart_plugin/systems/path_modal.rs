use bevy::prelude::*;
pub use ron::de::SpannedError as ParseError;
pub use ron::Error as DeserializeError;

use super::ui_helpers::{PathModalCancel, PathModalTop};

pub fn cancel_path_modal(
    mut commands: Commands,
    mut interaction_query: Query<
        (&Interaction, &PathModalCancel),
        (Changed<Interaction>, With<PathModalCancel>),
    >,
    query: Query<(Entity, &PathModalTop), With<PathModalTop>>,
) {
    for (interaction, path_modal_cancel) in interaction_query.iter_mut() {
        if let Interaction::Clicked = interaction {
            for (entity, path_modal_top) in query.iter() {
                if path_modal_cancel.id == path_modal_top.id {
                    commands.entity(entity).despawn_recursive();
                }
            }
        }
    }
}
