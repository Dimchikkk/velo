use async_channel::{Receiver, Sender};
use bevy::{
    prelude::*,
    text::{BreakLineOn, PositionedGlyph, TextLayoutInfo},
    window::{PrimaryWindow, WindowResized},
};
use bevy_markdown::{spawn_bevy_markdown, BevyMarkdown, BevyMarkdownNode};
use bevy_pkv::PkvStore;
use bevy_ui_borders::Outline;
use serde::{Deserialize, Serialize};

use crate::resources::AppState;

use crate::utils::ReflectableUuid;
use crate::{
    canvas::arrow::components::{ArrowConnect, ArrowConnectPos, ArrowType},
    components::Doc,
};
use crate::{
    canvas::arrow::events::{CreateArrowEvent, RedrawArrowEvent},
    resources::LoadTabRequest,
};
use std::{collections::HashMap, fs, path::PathBuf, time::Duration};
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
#[path = "systems/resize.rs"]
mod resize;
use resize::*;
#[path = "systems/button_handlers.rs"]
mod button_handlers;
use button_handlers::*;
#[path = "systems/tabs.rs"]
mod tabs;
use tabs::*;
#[path = "systems/doc_list.rs"]
mod doc_list;
use doc_list::*;

pub struct ChartPlugin;

pub struct AddRectEvent {
    pub node: JsonNode,
    pub image: Option<UiImage>,
}

pub struct SaveStoreEvent {
    pub doc_id: ReflectableUuid,
    pub path: Option<PathBuf>, // Save current document to file
}

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
    BottomRight,
    BottomLeft,
    TopRight,
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
    pub arrow_type: ArrowType,
    pub hold_entity: Option<ReflectableUuid>,
    pub entity_to_resize: Option<(ReflectableUuid, ResizeMarker)>,
    pub arrow_to_draw_start: Option<ArrowConnect>,
}

#[derive(Resource)]
pub struct BlinkTimer {
    timer: Timer,
}

#[derive(Debug, Default)]
struct Config {
    github_access_token: Option<String>,
}

impl Plugin for ChartPlugin {
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

        #[cfg(not(target_arch = "wasm32"))]
        app.add_startup_systems((init, init_layout).chain());
        #[cfg(target_arch = "wasm32")]
        app.add_startup_systems((load_from_url, init_layout));

        app.add_systems((
            rec_button_handlers,
            update_rectangle_position,
            create_new_rectangle,
            resize_entity_start,
            resize_entity_end,
            set_focused_entity,
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
            text_manipulation,
            mouse_scroll_list,
            list_item_click,
            new_doc_handler,
            rename_doc_handler,
            delete_doc_handler,
            save_doc_handler,
            keyboard_input_system,
            clickable_links,
        ));

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
            save_to_store.after(save_tab),
        ));
        app.add_system(
            entity_to_edit_changed
                .before(save_doc)
                .before(save_doc)
                .before(load_tab)
                .before(load_doc)
                .before(rec_button_handlers)
                .before(create_new_rectangle),
        );
    }
}

#[cfg(target_arch = "wasm32")]
pub fn get_timestamp() -> f64 {
    js_sys::Date::now()
}

#[cfg(target_arch = "wasm32")]
pub fn open_url_in_new_tab(url: &str) -> Result<(), wasm_bindgen::prelude::JsValue> {
    use wasm_bindgen::prelude::*;
    use web_sys::window;

    let window = window().ok_or_else(|| JsValue::from_str("Failed to get window object"))?;
    let new_window: Option<web_sys::Window> = window.open_with_url_and_target(url, "_blank")?;
    new_window.unwrap().focus()?;
    Ok(())
}

fn clickable_links(
    mut primary_window: Query<&mut Window, With<PrimaryWindow>>,
    mut editable_text_query: Query<
        (&Node, &GlobalTransform, &mut Text, &TextLayoutInfo),
        (With<EditableText>, Without<BevyMarkdownNode>),
    >,
    mut markdown_text_query: Query<
        (
            &Node,
            &GlobalTransform,
            &mut Text,
            &TextLayoutInfo,
            &BevyMarkdownNode,
        ),
        (With<BevyMarkdownNode>, Without<EditableText>),
    >,
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<VeloNode>)>,
    ui_state: Res<UiState>,
) {
    if ui_state.hold_entity.is_some() {
        return;
    }
    let mut primary_window = primary_window.iter_mut().next().unwrap();
    let scale_factor = primary_window.scale_factor() as f32;

    if let Some(cursor_position) = primary_window.cursor_position() {
        let window_height = primary_window.height();
        let pos = Vec2::new(cursor_position.x, window_height - cursor_position.y);
        for (node, transform, text, text_layout_info) in editable_text_query.iter_mut() {
            let mut str = "".to_string();
            let mut text_copy = text.clone();
            for section in text_copy.sections.iter_mut() {
                str = format!("{}{}", str, section.value.clone());
            }
            let sections = get_sections(str);

            let offset = transform.translation().truncate() - 0.5 * node.size();
            for PositionedGlyph {
                position,
                section_index,
                size,
                ..
            } in &text_layout_info.glyphs
            {
                let rect = bevy::math::Rect::from_center_size(
                    offset + *position / scale_factor,
                    *size / scale_factor,
                );
                if rect.contains(pos) {
                    if sections.1[*section_index] {
                        primary_window.cursor.icon = CursorIcon::Hand;
                        for interaction in &mut interaction_query {
                            if *interaction == Interaction::Clicked {
                                #[cfg(not(target_arch = "wasm32"))]
                                open::that(text.sections[*section_index].value.clone()).unwrap();
                                #[cfg(target_arch = "wasm32")]
                                open_url_in_new_tab(&text.sections[*section_index].value).unwrap();
                            }
                        }
                    } else {
                        primary_window.cursor.icon = CursorIcon::Default;
                    }
                }
            }
        }
        for (node, transform, text, text_layout_info, markdown_text) in
            markdown_text_query.iter_mut()
        {
            let mut str = "".to_string();
            let mut text_copy = text.clone();
            for section in text_copy.sections.iter_mut() {
                str = format!("{}{}", str, section.value.clone());
            }
            let link_sections = markdown_text.link_sections.clone();

            let offset = transform.translation().truncate() - 0.5 * node.size();
            for PositionedGlyph {
                position,
                section_index,
                size,
                ..
            } in &text_layout_info.glyphs
            {
                let rect = bevy::math::Rect::from_center_size(
                    offset + *position / scale_factor,
                    *size / scale_factor,
                );
                if rect.contains(pos) {
                    if let Some(link) = link_sections[*section_index].clone() {
                        primary_window.cursor.icon = CursorIcon::Hand;
                        for interaction in &mut interaction_query {
                            if *interaction == Interaction::Clicked {
                                #[cfg(not(target_arch = "wasm32"))]
                                open::that(link.clone()).unwrap();
                                #[cfg(target_arch = "wasm32")]
                                open_url_in_new_tab(link.clone().as_str()).unwrap();
                            }
                        }
                    } else {
                        primary_window.cursor.icon = CursorIcon::Default;
                    }
                }
            }
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub fn get_timestamp() -> f64 {
    use std::time::SystemTime;
    use std::time::UNIX_EPOCH;
    let duration = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    duration.as_millis() as f64
}

fn entity_to_edit_changed(
    ui_state: Res<UiState>,
    mut last_entity_to_edit: Local<Option<ReflectableUuid>>,
    mut velo_node_query: Query<(&mut Outline, &VeloNode, Entity), With<VeloNode>>,
    mut raw_text_node_query: Query<(&Text, &mut Style, &RawText, &Parent), With<RawText>>,
    mut markdown_text_node_query: Query<(Entity, &BevyMarkdownView), With<BevyMarkdownView>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    if ui_state.is_changed() && ui_state.entity_to_edit != *last_entity_to_edit {
        match ui_state.entity_to_edit {
            Some(entity_to_edit) => {
                // change border for selected node
                {
                    for (mut outline, node, _) in &mut velo_node_query.iter_mut() {
                        if node.id == entity_to_edit {
                            outline.color =
                                Color::rgba(33.0 / 255.0, 150.0 / 255.0, 243.0 / 255.0, 1.0);
                            outline.thickness = UiRect::all(Val::Px(2.));
                        } else {
                            match node.node_type {
                                NodeType::Rect => {
                                    outline.color =
                                        Color::rgb(158.0 / 255.0, 157.0 / 255.0, 36.0 / 255.0);
                                }
                                NodeType::Circle => {
                                    outline.color =
                                        Color::rgba(158.0 / 255.0, 157.0 / 255.0, 36.0 / 255.0, 0.);
                                }
                            }

                            outline.thickness = UiRect::all(Val::Px(1.));
                        }
                    }
                }
                // hide raw text and have markdown view for all nodes (except selected)
                {
                    for (text, mut style, raw_text, parent) in &mut raw_text_node_query.iter_mut() {
                        if raw_text.id == entity_to_edit {
                            style.display = Display::Flex;
                            continue;
                        }
                        if style.display == Display::None {
                            continue;
                        }
                        style.display = Display::None;
                        let mut str = "".to_string();
                        let mut text_copy = text.clone();
                        text_copy.sections.pop();
                        for section in text_copy.sections.iter() {
                            str = format!("{}{}", str, section.value.clone());
                        }
                        let bevy_markdown = BevyMarkdown {
                            text: str,
                            regular_font: Some(
                                asset_server.load("fonts/SourceCodePro-Regular.ttf"),
                            ),
                            bold_font: Some(asset_server.load("fonts/SourceCodePro-Bold.ttf")),
                            italic_font: Some(asset_server.load("fonts/SourceCodePro-Italic.ttf")),
                            semi_bold_italic_font: Some(
                                asset_server.load("fonts/SourceCodePro-SemiBoldItalic.ttf"),
                            ),
                            extra_bold_font: Some(
                                asset_server.load("fonts/SourceCodePro-ExtraBold.ttf"),
                            ),
                            size: Some((style.max_size.width, style.max_size.height)),
                        };
                        let markdown_text = spawn_bevy_markdown(&mut commands, bevy_markdown)
                            .expect("should handle markdown convertion");
                        commands
                            .get_entity(markdown_text)
                            .unwrap()
                            .insert(BevyMarkdownView { id: raw_text.id });
                        let (_, _, entity) = velo_node_query.get(parent.get()).unwrap();
                        commands.entity(entity).add_child(markdown_text);
                    }
                }
                // remove markdown view for selected node
                {
                    for (entity, node) in &mut markdown_text_node_query.iter_mut() {
                        if node.id == entity_to_edit {
                            commands.entity(entity).despawn_recursive();
                            break;
                        }
                    }
                }
            }
            None => {
                // change border
                {
                    for (mut outline, node, _) in &mut velo_node_query.iter_mut() {
                        match node.node_type {
                            NodeType::Rect => {
                                outline.color =
                                    Color::rgb(158.0 / 255.0, 157.0 / 255.0, 36.0 / 255.0);
                            }
                            NodeType::Circle => {
                                outline.color =
                                    Color::rgba(158.0 / 255.0, 157.0 / 255.0, 36.0 / 255.0, 0.);
                            }
                        }
                        outline.thickness = UiRect::all(Val::Px(1.));
                    }
                }
                for (text, mut style, raw_text, parent) in &mut raw_text_node_query.iter_mut() {
                    if style.display == Display::None {
                        continue;
                    }
                    style.display = Display::None;
                    let mut str = "".to_string();
                    let mut text_copy = text.clone();
                    text_copy.sections.pop();
                    for section in text_copy.sections.iter() {
                        str = format!("{}{}", str, section.value.clone());
                    }
                    let bevy_markdown = BevyMarkdown {
                        text: str,
                        regular_font: Some(TextStyle::default().font),
                        bold_font: Some(asset_server.load("fonts/SourceCodePro-Bold.ttf")),
                        italic_font: Some(asset_server.load("fonts/SourceCodePro-Italic.ttf")),
                        semi_bold_italic_font: Some(
                            asset_server.load("fonts/SourceCodePro-SemiBoldItalic.ttf"),
                        ),
                        extra_bold_font: Some(
                            asset_server.load("fonts/SourceCodePro-ExtraBold.ttf"),
                        ),
                        size: Some((style.max_size.width, style.max_size.height)),
                    };
                    let markdown_text = spawn_bevy_markdown(&mut commands, bevy_markdown).unwrap();
                    commands
                        .get_entity(markdown_text)
                        .unwrap()
                        .insert(BevyMarkdownView { id: raw_text.id });
                    let (_, _, entity) = velo_node_query.get(parent.get()).unwrap();
                    commands.entity(entity).add_child(markdown_text);
                }
            }
        }
        *last_entity_to_edit = ui_state.entity_to_edit;
    }
}

fn set_focused_entity(
    mut interaction_query: Query<(&Interaction, &VeloNode), (Changed<Interaction>, With<VeloNode>)>,
    mut state: ResMut<UiState>,
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
    buttons: Res<Input<MouseButton>>,
    mut holding_time: Local<(Duration, Option<ReflectableUuid>)>,
) {
    let mut window = windows.single_mut();
    for (interaction, rectangle) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                window.cursor.icon = CursorIcon::Text;
                *state = UiState::default();
                state.entity_to_edit = Some(rectangle.id);
                let now_ms = get_timestamp();
                *holding_time = (Duration::from_millis(now_ms as u64), Some(rectangle.id));
            }
            Interaction::Hovered => {
                if state.hold_entity.is_none() && state.entity_to_edit.is_none() {
                    window.cursor.icon = CursorIcon::Default;
                }
                if state.entity_to_edit.is_some() {
                    window.cursor.icon = CursorIcon::Text;
                }
            }
            Interaction::None => {
                window.cursor.icon = CursorIcon::Default;
            }
        }
    }

    if state.hold_entity.is_some() {
        window.cursor.icon = CursorIcon::Move;
    }

    let now_ms = get_timestamp();
    // 150ms delay before re-positioning the rectangle
    if state.hold_entity.is_none()
        && Duration::from_millis(now_ms as u64) - holding_time.0 > Duration::from_millis(150)
        && holding_time.1.is_some()
    {
        state.hold_entity = holding_time.1;
    }

    if buttons.just_released(MouseButton::Left) {
        *holding_time = (Duration::new(0, 0), None);
        state.hold_entity = None;
        state.entity_to_resize = None;
    }
}

fn update_rectangle_position(
    mut cursor_moved_events: EventReader<CursorMoved>,
    mut node_position: Query<(&mut Style, &VeloNodeContainer), With<VeloNodeContainer>>,
    state: Res<UiState>,
    mut query: Query<(&Style, &LeftPanel), Without<VeloNodeContainer>>,
    mut events: EventWriter<RedrawArrowEvent>,
    windows: Query<&Window, With<PrimaryWindow>>,
) {
    let primary_window = windows.single();
    for event in cursor_moved_events.iter() {
        for (mut style, top) in &mut node_position.iter_mut() {
            if Some(top.id) == state.hold_entity {
                let size = query.single_mut().0.size;
                if let (Val::Percent(x), Val::Px(element_width)) = (size.width, style.size.width) {
                    let width = (primary_window.width() * x) / 100.;
                    style.position.left = Val::Px(event.position.x - width - element_width / 2.);
                }
                if let Val::Px(element_height) = style.size.height {
                    style.position.bottom = Val::Px(event.position.y - element_height / 2.);
                }
                events.send(RedrawArrowEvent { id: top.id });
            }
        }
    }
}

fn create_new_rectangle(
    mut commands: Commands,
    mut events: EventReader<AddRectEvent>,
    mut ui_state: ResMut<UiState>,
    main_panel_query: Query<Entity, With<MainPanel>>,
    asset_server: Res<AssetServer>,
) {
    for event in events.iter() {
        *ui_state = UiState::default();
        ui_state.entity_to_edit = Some(ReflectableUuid(event.node.id));
        let entity = spawn_node(
            &mut commands,
            &asset_server,
            NodeMeta {
                size: (event.node.width, event.node.height),
                id: ReflectableUuid(event.node.id),
                node_type: event.node.node_type.clone(),
                image: event.image.clone(),
                text: event.node.text.text.clone(),
                bg_color: event.node.bg_color,
                position: (event.node.left, event.node.bottom),
                text_pos: event.node.text.pos.clone(),
                z_index: event.node.z_index,
                is_active: true,
            },
        );
        commands.entity(main_panel_query.single()).add_child(entity);
    }
}

fn resize_notificator(
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
fn init(mut app_state: ResMut<AppState>) {
    let config = read_config_file().unwrap_or_default();
    if let Some(github_token) = &config.github_access_token {
        app_state.github_token = Some(github_token.clone());
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn read_config_file() -> Option<Config> {
    let home_dir = std::env::var("HOME").ok()?;
    let config_file_path = PathBuf::from(&home_dir).join(".velo.toml");
    let config_str = fs::read_to_string(config_file_path).ok()?;
    let config_value: toml::Value = toml::from_str(&config_str).ok()?;
    let mut config = Config::default();
    if let Some(token) = config_value.get("github_access_token") {
        if let Some(token_str) = token.as_str() {
            config.github_access_token = Some(token_str.to_owned());
        }
    }
    Some(config)
}

pub fn load_doc_to_memory(
    doc_id: ReflectableUuid,
    app_state: &mut ResMut<AppState>,
    pkv: &mut ResMut<PkvStore>,
) {
    if app_state.docs.contains_key(&doc_id) {
        return;
    }
    if let Ok(docs) = pkv.get::<HashMap<ReflectableUuid, Doc>>("docs") {
        if docs.contains_key(&doc_id) {
            let keys = app_state.docs.keys().cloned().collect::<Vec<_>>();
            while (app_state.docs.len() as i32) >= MAX_SAVED_DOCS_IN_MEMORY {
                app_state.docs.remove(&keys[0]);
            }
            app_state
                .docs
                .insert(doc_id, docs.get(&doc_id).unwrap().clone());
        } else {
            panic!("Document not found in pkv");
        }
    }
}
