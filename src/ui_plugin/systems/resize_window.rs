use bevy::{prelude::*, window::WindowResized};

use crate::resources::AppState;

use crate::resources::LoadTabRequest;

pub fn resize_notificator(
    mut commands: Commands,
    resize_event: Res<Events<WindowResized>>,
    app_state: Res<AppState>,
) {
    let mut reader = resize_event.get_reader();
    for _ in reader.iter(&resize_event) {
        if let Some(current_doc) = app_state.docs.get(&app_state.current_document.unwrap()) {
            if let Some(active_tab) = current_doc.tabs.iter().find(|t| t.is_active) {
                commands.insert_resource(LoadTabRequest {
                    doc_id: current_doc.id,
                    tab_id: active_tab.id,
                    drop_last_checkpoint: false,
                });
            }
        }
    }
}
