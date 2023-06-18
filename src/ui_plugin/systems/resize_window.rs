use bevy::{prelude::*, window::WindowResized};
use bevy_cosmic_edit::CosmicEdit;
use cosmic_text::Edit;

use crate::resources::AppState;

use crate::resources::LoadTabRequest;

use super::ui_helpers::DocListItemButton;
use super::ui_helpers::TabButton;

pub fn resize_notificator(
    mut commands: Commands,
    resize_event: Res<Events<WindowResized>>,
    app_state: Res<AppState>,
    mut tabs: Query<&mut CosmicEdit, (With<TabButton>, Without<DocListItemButton>)>,
    mut docs: Query<&mut CosmicEdit, (With<DocListItemButton>, Without<TabButton>)>,
) {
    let mut reader = resize_event.get_reader();
    let resize_events: Vec<_> = reader.iter(&resize_event).collect();
    if !resize_events.is_empty() {
        if let Some(current_doc) = app_state.docs.get(&app_state.current_document.unwrap()) {
            if let Some(active_tab) = current_doc.tabs.iter().find(|t| t.is_active) {
                commands.insert_resource(LoadTabRequest {
                    doc_id: current_doc.id,
                    tab_id: active_tab.id,
                    drop_last_checkpoint: false,
                });
            }
        }
        for mut cosmic_edit in &mut tabs.iter_mut() {
            cosmic_edit.editor.buffer_mut().set_redraw(true);
        }
        for mut cosmic_edit in &mut docs.iter_mut() {
            cosmic_edit.editor.buffer_mut().set_redraw(true);
        }
    }
}
