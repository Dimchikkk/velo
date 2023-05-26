use async_channel::{Receiver, Sender};
use bevy::{prelude::*, text::BreakLineOn};

use serde::{Deserialize, Serialize};

use crate::resources::AppState;

use crate::canvas::arrow::components::{ArrowConnect, ArrowConnectPos, ArrowType};
use crate::canvas::arrow::events::{CreateArrowEvent, RedrawArrowEvent};
use crate::utils::ReflectableUuid;
use std::path::PathBuf;
use uuid::Uuid;
#[path = "ui_helpers/ui_helpers.rs"]
pub mod ui_helpers;
pub use ui_helpers::*;
#[path = "systems/save.rs"]
mod save_systems;
use save_systems::*;
#[path = "systems/load.rs"]
mod load_systems;
use load_systems::*;
#[path = "systems/keyboard.rs"]
mod keyboard_systems;
use keyboard_systems::*;
#[path = "systems/modal.rs"]
mod modal;
use modal::*;
#[path = "systems/init_layout/init_layout.rs"]
mod init_layout;
use init_layout::*;
#[path = "systems/resize_node.rs"]
mod resize_node;
use resize_node::*;
#[path = "systems/resize_window.rs"]
mod resize_window;
use resize_window::*;
#[path = "systems/button_handlers.rs"]
mod button_handlers;
use button_handlers::*;
#[path = "systems/tabs.rs"]
mod tabs;
use tabs::*;
#[path = "systems/doc_list.rs"]
mod doc_list;
use doc_list::*;
#[path = "systems/clickable_links.rs"]
mod clickable_links;
use clickable_links::*;
#[path = "systems/entity_to_edit_changed.rs"]
mod entity_to_edit_changed;
use entity_to_edit_changed::*;
#[path = "systems/set_focused_entity.rs"]
mod set_focused_entity;
use set_focused_entity::*;
#[path = "systems/update_rectangle_position.rs"]
mod update_rectangle_position;
use update_rectangle_position::*;
#[path = "systems/create_new_node.rs"]
mod create_new_node;
use create_new_node::*;
#[cfg(not(target_arch = "wasm32"))]
#[path = "systems/search.rs"]
#[cfg(not(target_arch = "wasm32"))]
mod search;
#[cfg(not(target_arch = "wasm32"))]
pub use search::*;
#[path = "systems/doc_list_ui_changed.rs"]
mod doc_list_ui_changed;
use doc_list_ui_changed::*;

pub struct UiPlugin;

pub struct AddRectEvent {
    pub node: JsonNode,
    pub image: Option<UiImage>,
}

pub struct SaveStoreEvent {
    pub doc_id: ReflectableUuid,
    pub path: Option<PathBuf>, // Save current document to file
}

pub struct UpdateDeleteDocBtnEvent;

#[derive(Resource, Clone)]
pub struct CommChannels {
    pub tx: Sender<String>,
    pub rx: Receiver<String>,
}

#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Reflect, Default, Debug)]
pub enum NodeType {
    #[default]
    Rect,
    Circle,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum TextPos {
    Center,
    TopLeft,
}

#[derive(Serialize, Deserialize)]
pub struct JsonNodeText {
    pub text: String,
    pub pos: TextPos,
}

#[derive(Serialize, Deserialize)]
pub struct JsonNode {
    pub id: Uuid,
    pub node_type: NodeType,
    pub left: Val,
    pub bottom: Val,
    pub width: Val,
    pub height: Val,
    pub text: JsonNodeText,
    pub bg_color: Color,
    pub z_index: i32,
}

pub const MAX_CHECKPOINTS: i32 = 7;
pub const MAX_SAVED_DOCS_IN_MEMORY: i32 = 7;

#[derive(Resource, Default)]
pub struct UiState {
    pub modal_id: Option<ReflectableUuid>,
    pub entity_to_edit: Option<ReflectableUuid>,
    pub tab_to_edit: Option<ReflectableUuid>,
    pub doc_to_edit: Option<ReflectableUuid>,
    pub search_box_to_edit: Option<ReflectableUuid>,
    pub arrow_type: ArrowType,
    pub hold_entity: Option<ReflectableUuid>,
    pub entity_to_resize: Option<(ReflectableUuid, ResizeMarker)>,
    pub arrow_to_draw_start: Option<ArrowConnect>,
}

#[derive(Resource)]
pub struct BlinkTimer {
    timer: Timer,
}

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<UiState>();
        app.init_resource::<AppState>();

        app.register_type::<VeloNode>();
        app.register_type::<EditableText>();
        app.register_type::<ArrowConnect>();
        app.register_type::<ResizeMarker>();
        app.register_type::<ReflectableUuid>();
        app.register_type_data::<ReflectableUuid, ReflectSerialize>();
        app.register_type_data::<ReflectableUuid, ReflectDeserialize>();
        app.register_type::<ArrowConnectPos>();

        app.register_type::<BreakLineOn>();

        app.add_event::<AddRectEvent>();
        app.add_event::<CreateArrowEvent>();
        app.add_event::<RedrawArrowEvent>();
        app.add_event::<SaveStoreEvent>();
        app.add_event::<UpdateDeleteDocBtnEvent>();

        #[cfg(not(target_arch = "wasm32"))]
        app.add_startup_systems((read_native_config, init_search_index).before(init_layout));
        #[cfg(target_arch = "wasm32")]
        app.add_startup_system(load_from_url.before(init_layout));
        app.add_startup_system(init_layout);

        app.add_systems((
            rec_button_handlers,
            update_rectangle_position,
            create_new_node,
            resize_entity_start,
            resize_entity_end,
            cancel_modal,
            confirm_modal,
            resize_notificator,
        ));

        app.add_systems(
            (save_doc, remove_save_doc_request)
                .chain()
                .distributive_run_if(should_save_doc),
        );

        app.add_systems(
            (save_tab, remove_save_tab_request)
                .chain()
                .distributive_run_if(should_save_tab),
        );

        app.add_systems(
            (load_doc, remove_load_doc_request)
                .chain()
                .distributive_run_if(should_load_doc),
        );

        app.add_systems(
            (load_tab, remove_load_tab_request)
                .chain()
                .distributive_run_if(should_load_tab),
        );

        app.add_systems((
            change_color_pallete,
            change_arrow_type,
            change_text_pos,
            add_tab_handler,
            delete_tab_handler,
            rename_tab_handler,
            mouse_scroll_list,
            list_item_click,
            new_doc_handler,
            rename_doc_handler,
            delete_doc_handler,
            save_doc_handler,
            keyboard_input_system,
        ));
        app.add_systems((doc_list_del_button_update, doc_list_ui_changed).chain());

        #[cfg(not(target_arch = "wasm32"))]
        app.add_systems((search_box_click, search_box_text_changed));

        app.add_systems((
            button_generic_handler,
            select_tab_handler,
            export_to_file,
            import_from_file,
            import_from_url,
            load_doc_handler,
            #[cfg(target_arch = "wasm32")]
            set_window_property,
            shared_doc_handler,
            #[cfg(not(target_arch = "wasm32"))]
            particles_effect,
            save_to_store.after(save_tab),
        ));
        app.add_systems((set_focused_entity, clickable_links).chain());

        app.add_system(
            entity_to_edit_changed
                .before(save_doc)
                .before(save_doc)
                .before(load_tab)
                .before(load_doc)
                .before(rec_button_handlers)
                .before(create_new_node),
        );
    }
}

#[cfg(target_arch = "wasm32")]
fn load_from_url(mut commands: Commands) {
    let (tx, rx) = async_channel::bounded(1);
    commands.insert_resource(CommChannels { tx: tx.clone(), rx });
    let href = web_sys::window().unwrap().location().href().unwrap();
    let url = url::Url::parse(href.as_str()).unwrap();
    let query_pairs: std::collections::HashMap<_, _> = url.query_pairs().into_owned().collect();
    if let Some(url) = query_pairs.get("document") {
        let pool = bevy::tasks::IoTaskPool::get();
        let mut finder = linkify::LinkFinder::new();
        finder.kinds(&[linkify::LinkKind::Url]);
        let links: Vec<_> = finder.links(url).collect();
        if links.len() == 1 {
            let url = links.first().unwrap().as_str().to_owned();
            let cc = tx.clone();
            let task = pool.spawn(async move {
                let request = ehttp::Request::get(url);
                ehttp::fetch(request, move |result| {
                    let json_string = result.unwrap().text().unwrap();
                    cc.try_send(json_string).unwrap();
                });
            });
            task.detach();
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn read_native_config(mut app_state: ResMut<AppState>) {
    use crate::utils::read_config_file;

    let config = read_config_file().unwrap_or_default();
    if let Some(github_token) = &config.github_access_token {
        app_state.github_token = Some(github_token.clone());
    }
}
