use bevy::prelude::*;
use bevy_cosmic_edit::{ActiveEditor, CosmicEdit, CosmicFont};
use cosmic_text::{Action, Edit};

pub fn active_editor_changed(
    active_editor: ResMut<ActiveEditor>,
    mut previous_editor: Local<Option<Entity>>,
    mut cosmic_edit_query: Query<&mut CosmicEdit, With<CosmicEdit>>,
    mut cosmic_fonts: ResMut<Assets<CosmicFont>>,
) {
    if active_editor.is_changed() && active_editor.entity != *previous_editor {
        if let Some(editor) = active_editor.entity {
            if let Ok(mut cosmic_edit) = cosmic_edit_query.get_mut(editor) {
                let font_system = cosmic_fonts.get_mut(&cosmic_edit.font_system).unwrap();
                cosmic_edit
                    .editor
                    .action(&mut font_system.0, Action::BufferEnd);
                cosmic_edit
                    .editor
                    .action(&mut font_system.0, Action::Escape);
                cosmic_edit.editor.buffer_mut().set_redraw(true);
            }
        }
        *previous_editor = active_editor.entity;
    }
}
