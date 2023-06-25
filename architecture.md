# PreStartup

_Runs once at the start of the app_

## Systems

- [VeloPlugin/setup_velo_theme](#setup_velo_theme)

# Startup

_Runs once at the start of the app_

## Systems

- (NATIVE-ONLY): [UiPlugin/read_native_config](#read_native_config)
- (NATIVE-ONLY): [UiPlugin/init_search_index](#init_search_index)
- (WASM-ONLY): [UiPlugin/load_from_url](#load_from_url)
- [UiPlugin/init_layout](#init_layout)
- [VeloPlugin/setup_camera](#setup_camera)
- [VeloPlugin/setup_background](#setup_background)

## Ordering

- (NATIVE-ONLY): [UiPlugin/read_native_config](#read_native_config) --> [UiPlugin/init_layout](#init_layout)
- (NATIVE-ONLY): [UiPlugin/init_search_index](#init_search_index) --> [UiPlugin/init_layout](#init_layout)
- (WASM-ONLY): [UiPlugin/load_from_url](#load_from_url) --> [UiPlugin/init_layout](#init_layout)

# PreUpdate

## Systems

- [ArrowPlugin/create_arrow_start](#create_arrow_start)
- [ArrowPlugin/create_arrow_end](#create_arrow_end)
- [ArrowPlugin/redraw_arrows](#redraw_arrows)

# Update

## Systems

- [CosmicEditPlugin/cosmic_edit_bevy_events](#cosmic_edit_bevy_events)
- [CosmicEditPlugin/cosmic_edit_redraw_buffer_ui](#cosmic_edit_redraw_buffer_ui)
- [CosmicEditPlugin/cosmic_edit_set_redraw](#cosmic_edit_set_redraw)
- [CosmicEditPlugin/cosmic_edit_redraw_buffer](#cosmic_edit_redraw_buffer)
- [CosmicEditPlugin/on_scale_factor_change](#on_scale_factor_change)
- [UiPlugin/rec_button_handlers](#rec_button_handlers)
- [UiPlugin/update_rectangle_position](#update_rectangle_position)
- [UiPlugin/create_new_node](#create_new_node)
- [UiPlugin/resize_entity_start](#resize_entity_start)
- [UiPlugin/resize_entity_run](#resize_entity_run)
- [UiPlugin/resize_entity_end](#resize_entity_end)
- [UiPlugin/cancel_modal](#cancel_modal)
- [UiPlugin/confirm_modal](#confirm_modal)
- if [UiPlugin/should_save_doc](#should_save_doc) ? [UiPlugin/save_doc](#save_doc)
- if [UiPlugin/should_save_doc](#should_save_doc) ? [UiPlugin/remove_save_doc_request](#remove_save_doc_request)
- if [UiPlugin/should_save_tab](#should_save_tab) ? [UiPlugin/save_tab](#save_tab)
- if [UiPlugin/should_save_tab](#should_save_tab) ? [UiPlugin/remove_save_tab_request](#remove_save_tab_request)
- if [UiPlugin/should_load_doc](#should_load_doc) ? [UiPlugin/load_doc](#load_doc)
- if [UiPlugin/should_load_doc](#should_load_doc) ? [UiPlugin/remove_load_doc_request](#remove_load_doc_request)
- if [UiPlugin/should_load_tab](#should_load_tab) ? [UiPlugin/load_tab](#load_tab)
- if [UiPlugin/should_load_tab](#should_load_tab) ? [UiPlugin/remove_load_tab_request](#remove_load_tab_request)
- [UiPlugin/change_color_pallete](#change_color_pallete)
- [UiPlugin/change_arrow_type](#change_arrow_type)
- [UiPlugin/change_text_pos](#change_text_pos)
- [UiPlugin/add_tab_handler](#add_tab_handler)
- [UiPlugin/delete_tab_handler](#delete_tab_handler)
- [UiPlugin/rename_tab_handler](#rename_tab_handler)
- [UiPlugin/mouse_scroll_list](#mouse_scroll_list)
- [UiPlugin/list_item_click](#list_item_click)
- [UiPlugin/new_doc_handler](#new_doc_handler)
- [UiPlugin/rename_doc_handler](#rename_doc_handler)
- [UiPlugin/delete_doc_handler](#delete_doc_handler)
- [UiPlugin/save_doc_handler](#save_doc_handler)
- [UiPlugin/keyboard_input_system](#keyboard_input_system)
- [UiPlugin/doc_list_del_button_update](#doc_list_del_button_update)
- [UiPlugin/doc_list_ui_changed](#doc_list_ui_changed)
- (NATIVE-ONLY): [UiPlugin/search_box_click](#search_box_click)
- (NATIVE-ONLY): [UiPlugin/search_box_text_changed](#search_box_text_changed)
- [UiPlugin/button_generic_handler](#button_generic_handler)
- [UiPlugin/select_tab_handler](#select_tab_handler)
- [UiPlugin/export_to_file](#export_to_file)
- [UiPlugin/import_from_file](#import_from_file)
- [UiPlugin/import_from_url](#import_from_url)
- [UiPlugin/load_doc_handler](#load_doc_handler)
- (WASM-ONLY): [UiPlugin/set_window_property](#set_window_property)
- [UiPlugin/shared_doc_handler](#shared_doc_handler)
- (NATIVE-ONLY): [UiPlugin/particles_effect](#particles_effect)
- [UiPlugin/save_to_store](#save_to_store)
- [UiPlugin/canvas_click](#canvas_click)
- [UiPlugin/active_editor_changed](#active_editor_changed)
- [UiPlugin/interactive_sprite](#interactive_sprite)
- [UiPlugin/change_theme](#change_theme)
- [UiPlugin/set_focused_entity](#set_focused_entity)
- [UiPlugin/clickable_links](#clickable_links)
- [UiPlugin/entity_to_edit_changed](#entity_to_edit_changed)

## Ordering

- [CosmicEditPlugin/cosmic_edit_redraw_buffer_ui](#cosmic_edit_redraw_buffer_ui) --> [CosmicEditPlugin/cosmic_edit_set_redraw](#cosmic_edit_set_redraw)
- [CosmicEditPlugin/cosmic_edit_redraw_buffer_ui](#cosmic_edit_redraw_buffer_ui) --> [CosmicEditPlugin/on_scale_factor_change](#on_scale_factor_change)
- [CosmicEditPlugin/cosmic_edit_redraw_buffer](#cosmic_edit_redraw_buffer) --> [CosmicEditPlugin/on_scale_factor_change](#on_scale_factor_change)
- [UiPlugin/save_doc](#save_doc) --> [UiPlugin/remove_save_doc_request](#remove_save_doc_request)
- [UiPlugin/save_doc](#save_doc) --> [UiPlugin/remove_save_tab_request](#remove_save_tab_request)
- [UiPlugin/save_doc](#save_doc) --> [UiPlugin/remove_load_doc_request](#remove_load_doc_request)
- [UiPlugin/save_doc](#save_doc) --> [UiPlugin/remove_load_tab_request](#remove_load_tab_request)
- [UiPlugin/keyboard_input_system](#keyboard_input_system) --> [CosmicEditPlugin/cosmic_edit_bevy_events](#cosmic_edit_bevy_events)
- [UiPlugin/doc_list_del_button_update](#doc_list_del_button_update) --> [UiPlugin/doc_list_ui_changed](#doc_list_ui_changed)
- [UiPlugin/save_tab](#save_tab) --> [UiPlugin/save_to_store](#save_to_store)
- [UiPlugin/interactive_sprite](#interactive_sprite) --> [UiPlugin/canvas_click](#canvas_click)
- [UiPlugin/set_focused_entity](#set_focused_entity) --> [UiPlugin/clickable_links](#clickable_links)
- [UiPlugin/entity_to_edit_changed](#entity_to_edit_changed) --> [UiPlugin/save_doc](#save_doc)
- [UiPlugin/entity_to_edit_changed](#entity_to_edit_changed) --> [UiPlugin/save_doc](#save_doc)
- [UiPlugin/entity_to_edit_changed](#entity_to_edit_changed) --> [UiPlugin/load_tab](#load_tab)
- [UiPlugin/entity_to_edit_changed](#entity_to_edit_changed) --> [UiPlugin/load_doc](#load_doc)
- [UiPlugin/entity_to_edit_changed](#entity_to_edit_changed) --> [UiPlugin/rec_button_handlers](#rec_button_handlers)
- [UiPlugin/entity_to_edit_changed](#entity_to_edit_changed) --> [UiPlugin/create_new_node](#create_new_node)

# PostUpdate

## Systems

- [UiPlugin/resize_notificator](#resize_notificator)

# All systems

## <span id="active_editor_changed">`active_editor_changed`</span>

```rs
pub fn active_editor_changed(
    active_editor: ResMut<ActiveEditor>,
    mut previous_editor: Local<Option<Entity>>,
    mut cosmic_edit_query: Query<&mut CosmicEdit, With<CosmicEdit>>,
    mut cosmic_fonts: ResMut<Assets<CosmicFont>>,
) {}
```

## <span id="add_tab_handler">`add_tab_handler`</span>

```rs
pub fn add_tab_handler(
    mut commands: Commands,
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<AddTab>)>,
    mut app_state: ResMut<AppState>,
) {}
```

## <span id="button_generic_handler">`button_generic_handler`</span>

```rs
pub fn button_generic_handler(
    _commands: Commands,
    mut generic_button_query: Query<
        (&Interaction, &mut BackgroundColor, Entity),
        (Changed<Interaction>, With<GenericButton>),
    >,
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
    mut tooltips_query: Query<(&mut Style, &Parent), With<Tooltip>>,
) {}
```

## <span id="cancel_modal">`cancel_modal`</span>

```rs
pub fn cancel_modal(
    mut commands: Commands,
    mut interaction_query: Query<
        (&Interaction, &ModalCancel),
        (Changed<Interaction>, With<ModalCancel>),
    >,
    mut state: ResMut<UiState>,
    query: Query<(Entity, &ModalTop), With<ModalTop>>,
) {}
```

## <span id="canvas_click">`canvas_click`</span>

```rs
pub fn canvas_click(
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<MainPanel>)>,
    mut ui_state: ResMut<UiState>,
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
    mut node_interaction_events: EventReader<NodeInteraction>,
    raw_text: Query<With<RawText>>,
) {}
```

## <span id="change_arrow_type">`change_arrow_type`</span>

```rs
pub fn change_arrow_type(
    mut interaction_query: Query<
        (&Interaction, &ArrowMode),
        (Changed<Interaction>, With<ArrowMode>),
    >,
    mut state: ResMut<UiState>,
) {}
```

## <span id="change_color_pallete">`change_color_pallete`</span>

```rs
pub fn change_color_pallete(
    mut interaction_query: Query<
        (&Interaction, &ChangeColor),
        (Changed<Interaction>, With<ChangeColor>),
    >,
    mut velo_border: Query<(&mut Fill, &VeloBorder), With<VeloBorder>>,
    ui_state: Res<UiState>,
) {}
```

## <span id="change_text_pos">`change_text_pos`</span>

```rs
pub fn change_text_pos(
    mut interaction_query: Query<
        (&Interaction, &TextPosMode),
        (Changed<Interaction>, With<TextPosMode>),
    >,
    state: Res<UiState>,
    mut raw_text_node_query: Query<(&RawText, &mut CosmicEdit), With<RawText>>,
) {}
```

## <span id="change_theme">`change_theme`</span>

```rs
pub fn change_theme(
    mut pkv: ResMut<PkvStore>,
    mut change_theme_button: Query<&Interaction, (Changed<Interaction>, With<ChangeTheme>)>,
    mut change_theme_label: Query<&mut Text, (With<ChangeTheme>, Without<Tooltip>)>,
    mut tooltip_label: Query<&mut Text, (With<Tooltip>, Without<ChangeTheme>)>,
) {}
```

## <span id="clickable_links">`clickable_links`</span>

```rs
pub fn clickable_links(
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
    mut markdown_text_query: Query<
        (&GlobalTransform, &mut CosmicEdit, &BevyMarkdownView),
        With<BevyMarkdownView>,
    >,
    mut node_interaction_events: EventReader<NodeInteraction>,
    ui_state: Res<UiState>,
) {}
```

## <span id="confirm_modal">`confirm_modal`</span>

```rs
pub fn confirm_modal(
    mut commands: Commands,
    mut interaction_query: Query<
        (&Interaction, &ModalConfirm),
        (Changed<Interaction>, With<ModalConfirm>),
    >,
    mut app_state: ResMut<AppState>,
    mut ui_state: ResMut<UiState>,
    query_top: Query<(Entity, &ModalTop), With<ModalTop>>,
    mut tab_query_container: Query<(Entity, &TabContainer), With<TabContainer>>,
    mut pkv: ResMut<PkvStore>,
    input: Res<Input<KeyCode>>,
    mut query_path: Query<(&CosmicEdit, &EditableText), With<EditableText>>,
    comm_channels: Res<CommChannels>,
) {}
```

## <span id="cosmic_edit_bevy_events">`cosmic_edit_bevy_events`</span>

```rs
pub fn cosmic_edit_bevy_events(
    windows: Query<&Window, With<PrimaryWindow>>,
    active_editor: Res<ActiveEditor>,
    keys: Res<Input<KeyCode>>,
    mut char_evr: EventReader<ReceivedCharacter>,
    buttons: Res<Input<MouseButton>>,
    mut cosmic_edit_query: Query<(&mut CosmicEdit, &GlobalTransform, Entity), With<CosmicEdit>>,
    mut is_deleting: Local<bool>,
    mut font_system_assets: ResMut<Assets<CosmicFont>>,
    mut scroll_evr: EventReader<MouseWheel>,
) {}
```

## <span id="cosmic_edit_redraw_buffer">`cosmic_edit_redraw_buffer`</span>

```rs
fn cosmic_edit_redraw_buffer(
    windows: Query<&Window, With<PrimaryWindow>>,
    mut images: ResMut<Assets<Image>>,
    mut swash_cache_state: ResMut<SwashCacheState>,
    mut cosmic_edit_query: Query<
        (&mut CosmicEdit, &mut Handle<Image>, &mut Visibility),
        With<CosmicEdit>,
    >,
    mut font_system_assets: ResMut<Assets<CosmicFont>>,
) {}
```

## <span id="cosmic_edit_redraw_buffer_ui">`cosmic_edit_redraw_buffer_ui`</span>

```rs
fn cosmic_edit_redraw_buffer_ui(
    windows: Query<&Window, With<PrimaryWindow>>,
    mut images: ResMut<Assets<Image>>,
    mut swash_cache_state: ResMut<SwashCacheState>,
    mut cosmic_edit_query: Query<
        (&mut CosmicEdit, &mut UiImage, &Node, &mut Visibility),
        With<CosmicEdit>,
    >,
    mut font_system_assets: ResMut<Assets<CosmicFont>>,
) {}
```

## <span id="cosmic_edit_set_redraw">`cosmic_edit_set_redraw`</span>

```rs
fn cosmic_edit_set_redraw(mut cosmic_edit_query: Query<&mut CosmicEdit, Added<CosmicEdit>>) {}
```

## <span id="create_arrow_end">`create_arrow_end`</span>

```rs
pub fn create_arrow_end(
    mut commands: Commands,
    mut events: EventReader<CreateArrow>,
    arrow_markers: Query<(&ArrowConnect, &GlobalTransform), With<ArrowConnect>>,
    theme: Res<Theme>,
) {}
```

## <span id="create_arrow_start">`create_arrow_start`</span>

```rs
pub fn create_arrow_start(
    mut node_interaction_events: EventReader<NodeInteraction>,
    arrow_connect_query: Query<&ArrowConnect, With<ArrowConnect>>,
    mut state: ResMut<UiState>,
    mut create_arrow: EventWriter<CreateArrow>,
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
) {}
```

## <span id="create_new_node">`create_new_node`</span>

```rs
pub fn create_new_node(
    mut commands: Commands,
    mut events: EventReader<AddRect>,
    mut ui_state: ResMut<UiState>,
    app_state: Res<AppState>,
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
    mut cosmic_fonts: ResMut<Assets<CosmicFont>>,
    font_system_state: ResMut<FontSystemState>,
    theme: Res<Theme>,
    mut z_index_local: Local<f32>,
    mut shaders: ResMut<Assets<Shader>>,
) {}
```

## <span id="delete_doc_handler">`delete_doc_handler`</span>

```rs
pub fn delete_doc_handler(
    mut commands: Commands,
    mut delete_doc_query: Query<&Interaction, (Changed<Interaction>, With<DeleteDoc>)>,
    mut ui_state: ResMut<UiState>,
    mut app_state: ResMut<AppState>,
    main_panel_query: Query<Entity, With<MainPanel>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    pkv: Res<PkvStore>,
    mut cosmic_fonts: ResMut<Assets<CosmicFont>>,
    font_system_state: ResMut<FontSystemState>,
    theme: Res<Theme>,
) {}
```

## <span id="delete_tab_handler">`delete_tab_handler`</span>

```rs
pub fn delete_tab_handler(
    mut commands: Commands,
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<DeleteTab>)>,
    mut app_state: ResMut<AppState>,
    mut ui_state: ResMut<UiState>,
    main_panel_query: Query<Entity, With<MainPanel>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    mut cosmic_fonts: ResMut<Assets<CosmicFont>>,
    font_system_state: ResMut<FontSystemState>,
    theme: Res<Theme>,
) {}
```

## <span id="doc_list_del_button_update">`doc_list_del_button_update`</span>

```rs
pub fn doc_list_del_button_update(
    app_state: Res<AppState>,
    mut delete_doc: Query<(&mut Visibility, &DeleteDoc), With<DeleteDoc>>,
    mut event_reader: EventReader<UpdateDeleteDocBtn>,
) {}
```

## <span id="doc_list_ui_changed">`doc_list_ui_changed`</span>

```rs
pub fn doc_list_ui_changed(
    mut commands: Commands,
    app_state: Res<AppState>,
    mut last_doc_list: Local<HashSet<ReflectableUuid>>,
    mut doc_list_query: Query<Entity, With<DocList>>,
    asset_server: Res<AssetServer>,
    pkv: Res<PkvStore>,
    mut query_container: Query<Entity, With<DocListItemContainer>>,
    mut event_writer: EventWriter<UpdateDeleteDocBtn>,
    theme: Res<Theme>,
    mut cosmic_fonts: ResMut<Assets<CosmicFont>>,
    font_system_state: ResMut<FontSystemState>,
    windows: Query<&mut Window, With<PrimaryWindow>>,
) {}
```

## <span id="entity_to_edit_changed">`entity_to_edit_changed`</span>

```rs
pub fn entity_to_edit_changed(
    ui_state: Res<UiState>,
    app_state: Res<AppState>,
    theme: Res<Theme>,
    mut last_entity_to_edit: Local<Option<ReflectableUuid>>,
    mut velo_border: Query<(&mut Stroke, &VeloBorder), With<VeloBorder>>,
    mut raw_text_node_query: Query<(Entity, &mut RawText, &mut CosmicEdit), With<RawText>>,
    mut commands: Commands,
    mut cosmic_fonts: ResMut<Assets<CosmicFont>>,
) {}
```

## <span id="export_to_file">`export_to_file`</span>

```rs
pub fn export_to_file(
    mut commands: Commands,
    mut query: Query<&Interaction, (Changed<Interaction>, With<ExportToFile>)>,
    mut ui_state: ResMut<UiState>,
    main_panel_query: Query<Entity, With<MainPanel>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    mut cosmic_fonts: ResMut<Assets<CosmicFont>>,
    font_system_state: ResMut<FontSystemState>,
    theme: Res<Theme>,
) {}
```

## <span id="import_from_file">`import_from_file`</span>

```rs
pub fn import_from_file(
    mut commands: Commands,
    mut query: Query<&Interaction, (Changed<Interaction>, With<ImportFromFile>)>,
    mut ui_state: ResMut<UiState>,
    main_panel_query: Query<Entity, With<MainPanel>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    mut cosmic_fonts: ResMut<Assets<CosmicFont>>,
    font_system_state: ResMut<FontSystemState>,
    theme: Res<Theme>,
) {}
```

## <span id="import_from_url">`import_from_url`</span>

```rs
pub fn import_from_url(
    mut commands: Commands,
    mut query: Query<&Interaction, (Changed<Interaction>, With<ImportFromUrl>)>,
    mut ui_state: ResMut<UiState>,
    main_panel_query: Query<Entity, With<MainPanel>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    mut cosmic_fonts: ResMut<Assets<CosmicFont>>,
    font_system_state: ResMut<FontSystemState>,
    theme: Res<Theme>,
) {}
```

## <span id="init_layout">`init_layout`</span>

```rs
pub fn init_layout(
    mut commands: Commands,
    mut app_state: ResMut<AppState>,
    asset_server: Res<AssetServer>,
    mut pkv: ResMut<PkvStore>,
    mut cosmic_fonts: ResMut<Assets<CosmicFont>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    mut fonts: ResMut<Assets<Font>>,
    theme: Res<Theme>,
) {}
```

## <span id="init_search_index">`init_search_index`</span>

```rs
pub fn init_search_index(mut app_state: ResMut<AppState>) {}
```

## <span id="interactive_sprite">`interactive_sprite`</span>

```rs
pub fn interactive_sprite(
    cursor_moved_events: EventReader<CursorMoved>,
    windows: Query<&Window, With<PrimaryWindow>>,
    buttons: Res<Input<MouseButton>>,
    res_images: Res<Assets<Image>>,
    mut sprite_query: Query<
        (&Sprite, &Handle<Image>, &GlobalTransform, Entity),
        With<InteractiveNode>,
    >,
    camera_q: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut node_interaction_events: EventWriter<NodeInteraction>,
    mut double_click: Local<(Duration, Option<Entity>)>,
    mut holding_state: Local<HoldingState>,
) {}
```

## <span id="keyboard_input_system">`keyboard_input_system`</span>

```rs
pub fn keyboard_input_system(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    mut app_state: ResMut<AppState>,
    mut ui_state: ResMut<UiState>,
    mut events: EventWriter<AddRect>,
    input: Res<Input<KeyCode>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    mut editable_text_query: Query<(&EditableText, &mut CosmicEdit), With<EditableText>>,
    theme: Res<Theme>,
) {}
```

## <span id="list_item_click">`list_item_click`</span>

```rs
pub fn list_item_click(
    mut interaction_query: Query<
        (&Interaction, &DocListItemButton),
        (Changed<Interaction>, With<DocListItemButton>),
    >,
    mut state: ResMut<AppState>,
    mut commands: Commands,
) {}
```

## <span id="load_doc">`load_doc`</span>

```rs
pub fn load_doc(
    request: Res<LoadDocRequest>,
    mut app_state: ResMut<AppState>,
    mut commands: Commands,
    mut bottom_panel: Query<Entity, With<BottomPanel>>,
    mut pkv: ResMut<PkvStore>,
    asset_server: Res<AssetServer>,
    mut tabs_query: Query<Entity, With<TabContainer>>,
    mut delete_doc: Query<(&mut Visibility, &DeleteDoc), With<DeleteDoc>>,
    theme: Res<Theme>,
    mut cosmic_fonts: ResMut<Assets<CosmicFont>>,
    font_system_state: ResMut<FontSystemState>,
    windows: Query<&Window, With<PrimaryWindow>>,
) {}
```

## <span id="load_doc_handler">`load_doc_handler`</span>

```rs
pub fn load_doc_handler(
    mut commands: Commands,
    mut app_state: ResMut<AppState>,
    comm_channels: Res<CommChannels>,
    pkv: Res<PkvStore>,
) {}
```

## <span id="load_from_url">`load_from_url`</span>

```rs
fn load_from_url(mut commands: Commands) {}
```

## <span id="load_tab">`load_tab`</span>

```rs
pub fn load_tab(
    old_nodes: Query<Entity, With<VeloNode>>,
    mut old_arrows: Query<Entity, With<ArrowMeta>>,
    request: Res<LoadTabRequest>,
    mut app_state: ResMut<AppState>,
    mut ui_state: ResMut<UiState>,
    mut commands: Commands,
    mut res_images: ResMut<Assets<Image>>,
    mut create_arrow: EventWriter<CreateArrow>,
    mut delete_tab: Query<(&mut Visibility, &DeleteTab), (With<DeleteTab>, Without<ArrowMeta>)>,
    mut cosmic_fonts: ResMut<Assets<CosmicFont>>,
    font_system_state: ResMut<FontSystemState>,
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
    mut shaders: ResMut<Assets<Shader>>,
    theme: Res<Theme>,
) {}
```

## <span id="mouse_scroll_list">`mouse_scroll_list`</span>

```rs
pub fn mouse_scroll_list(
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mut query_list: Query<(&mut ScrollingList, &mut Style, &Parent, &Node)>,
    query_node: Query<&Node>,
) {}
```

## <span id="new_doc_handler">`new_doc_handler`</span>

```rs
pub fn new_doc_handler(
    mut commands: Commands,
    mut new_doc_query: Query<&Interaction, (Changed<Interaction>, With<NewDoc>)>,
    mut app_state: ResMut<AppState>,
) {}
```

## <span id="on_scale_factor_change">`on_scale_factor_change`</span>

```rs
fn on_scale_factor_change(
    mut scale_factor_changed: EventReader<WindowScaleFactorChanged>,
    mut cosmic_edit_query: Query<&mut CosmicEdit, With<CosmicEdit>>,
    mut cosmic_fonts: ResMut<Assets<CosmicFont>>,
) {}
```

## <span id="particles_effect">`particles_effect`</span>

```rs
pub fn particles_effect(
    mut query: Query<&Interaction, (Changed<Interaction>, With<ParticlesEffect>)>,
    mut commands: Commands,
    mut effects: ResMut<Assets<bevy_hanabi::EffectAsset>>,
    mut effects_camera: Query<&mut Camera, With<EffectsCamera>>,
    mut effects_query: Query<(&Name, Entity)>,
    mut shadow_query: Query<&mut Transform, With<VeloShadow>>,
) {}
```

## <span id="read_native_config">`read_native_config`</span>

```rs
fn read_native_config(mut app_state: ResMut<AppState>) {}
```

## <span id="rec_button_handlers">`rec_button_handlers`</span>

```rs
pub fn rec_button_handlers(
    mut commands: Commands,
    mut events: EventWriter<AddRect>,
    mut interaction_query: Query<
        (&Interaction, &ButtonAction),
        (Changed<Interaction>, With<ButtonAction>),
    >,
    mut raw_text_query: Query<(&mut CosmicEdit, &RawText, &Parent), With<RawText>>,
    border_query: Query<&Parent, With<VeloBorder>>,
    mut velo_node_query: Query<(Entity, &VeloNode, &mut Transform), With<VeloNode>>,
    mut arrows: Query<(Entity, &ArrowMeta, &mut Visibility), (With<ArrowMeta>, Without<Tooltip>)>,
    mut state: ResMut<UiState>,
    mut app_state: ResMut<AppState>,
    theme: Res<Theme>,
) {}
```

## <span id="redraw_arrows">`redraw_arrows`</span>

```rs
pub fn redraw_arrows(
    mut redraw_arrow: EventReader<RedrawArrow>,
    mut arrow_query: Query<(&mut Path, &mut ArrowMeta), With<ArrowMeta>>,
    arrow_markers: Query<(&ArrowConnect, &GlobalTransform), With<ArrowConnect>>,
) {}
```

## <span id="remove_load_doc_request">`remove_load_doc_request`</span>

```rs
pub fn remove_load_doc_request(world: &mut World) {}
```

## <span id="remove_load_tab_request">`remove_load_tab_request`</span>

```rs
pub fn remove_load_tab_request(world: &mut World) {}
```

## <span id="remove_save_doc_request">`remove_save_doc_request`</span>

```rs
pub fn remove_save_doc_request(world: &mut World) {}
```

## <span id="remove_save_tab_request">`remove_save_tab_request`</span>

```rs
pub fn remove_save_tab_request(world: &mut World) {}
```

## <span id="rename_doc_handler">`rename_doc_handler`</span>

```rs
pub fn rename_doc_handler(
    mut commands: Commands,
    mut rename_doc_query: Query<
        (&Interaction, &DocListItemButton, Entity, &mut CosmicEdit),
        (Changed<Interaction>, With<DocListItemButton>),
    >,
    mut ui_state: ResMut<UiState>,
    mut double_click: Local<(Duration, Option<ReflectableUuid>)>,
    theme: Res<Theme>,
) {}
```

## <span id="rename_tab_handler">`rename_tab_handler`</span>

```rs
pub fn rename_tab_handler(
    mut commands: Commands,
    mut interaction_query: Query<
        (&Interaction, &TabButton, Entity, &mut CosmicEdit),
        (Changed<Interaction>, With<TabButton>),
    >,
    mut ui_state: ResMut<UiState>,
    mut app_state: ResMut<AppState>,
    mut double_click: Local<(Duration, Option<ReflectableUuid>)>,
    theme: Res<Theme>,
) {}
```

## <span id="resize_entity_end">`resize_entity_end`</span>

```rs
pub fn resize_entity_end(
    mut commands: Commands,
    mut shaders: ResMut<Assets<Shader>>,
    theme: ResMut<Theme>,
    mut ui_state: ResMut<UiState>,
    mut node_interaction_events: EventReader<NodeInteraction>,
    raw_text_query: Query<(&Parent, &RawText, &CosmicEdit), With<RawText>>,
    border_query: Query<(&Parent, &VeloBorder), With<VeloBorder>>,
    velo_node_query: Query<Entity, With<VeloNode>>,
) {}
```

## <span id="resize_entity_run">`resize_entity_run`</span>

```rs
pub fn resize_entity_run(
    mut commands: Commands,
    ui_state: ResMut<UiState>,
    mut cursor_moved_events: EventReader<CursorMoved>,
    mut events: EventWriter<RedrawArrow>,
    mut resize_marker_query: Query<
        (&ResizeMarker, &Parent, &mut Transform),
        (With<ResizeMarker>, Without<VeloNode>, Without<ArrowConnect>),
    >,
    mut arrow_connector_query: Query<
        (&ArrowConnect, &mut Transform),
        (With<ArrowConnect>, Without<VeloNode>, Without<ResizeMarker>),
    >,
    mut raw_text_query: Query<(&Parent, &RawText, &mut CosmicEdit, &mut Sprite), With<RawText>>,
    mut border_query: Query<(&Parent, &VeloBorder, &mut Path), With<VeloBorder>>,
    mut velo_node_query: Query<
        (&mut Transform, &Children),
        (With<VeloNode>, Without<ResizeMarker>, Without<ArrowConnect>),
    >,
    shadow_query: Query<Entity, With<VeloShadow>>,
    camera_q: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {}
```

## <span id="resize_entity_start">`resize_entity_start`</span>

```rs
pub fn resize_entity_start(
    mut ui_state: ResMut<UiState>,
    mut node_interaction_events: EventReader<NodeInteraction>,
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
    resize_marker_query: Query<(&ResizeMarker, &Parent, &mut Transform), With<ResizeMarker>>,
    velo_node_query: Query<&VeloNode, With<VeloNode>>,
) {}
```

## <span id="resize_notificator">`resize_notificator`</span>

```rs
pub fn resize_notificator(
    mut commands: Commands,
    resize_event: Res<Events<WindowResized>>,
    app_state: Res<AppState>,
    mut tabs: Query<&mut CosmicEdit, (With<TabButton>, Without<DocListItemButton>)>,
    mut docs: Query<&mut CosmicEdit, (With<DocListItemButton>, Without<TabButton>)>,
) {}
```

## <span id="save_doc">`save_doc`</span>

```rs
pub fn save_doc(
    request: Res<SaveDocRequest>,
    mut app_state: ResMut<AppState>,
    mut pkv: ResMut<PkvStore>,
    mut commands: Commands,
    mut events: EventWriter<SaveStore>,
) {}
```

## <span id="save_doc_handler">`save_doc_handler`</span>

```rs
pub fn save_doc_handler(
    mut commands: Commands,
    mut save_doc_query: Query<&Interaction, (Changed<Interaction>, With<SaveDoc>)>,
    state: Res<AppState>,
) {}
```

## <span id="save_tab">`save_tab`</span>

```rs
pub fn save_tab(
    images: Res<Assets<Image>>,
    arrows: Query<&ArrowMeta, With<ArrowMeta>>,
    request: Res<SaveTabRequest>,
    mut app_state: ResMut<AppState>,
    raw_text_query: Query<(&RawText, &CosmicEdit, &Parent), With<RawText>>,
    border_query: Query<(&Parent, &VeloBorder, &Fill), With<VeloBorder>>,
    velo_node_query: Query<&Transform, With<VeloNode>>,
) {}
```

## <span id="save_to_store">`save_to_store`</span>

```rs
pub fn save_to_store(
    mut pkv: ResMut<PkvStore>,
    mut app_state: ResMut<AppState>,
    mut events: EventReader<SaveStore>,
) {}
```

## <span id="search_box_click">`search_box_click`</span>

```rs
pub fn search_box_click(
    mut commands: Commands,
    mut interaction_query: Query<
        (&Interaction, &SearchButton),
        (Changed<Interaction>, With<SearchButton>),
    >,
    mut search_query: Query<(&SearchText, Entity), With<SearchText>>,
    mut state: ResMut<UiState>,
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
) {}
```

## <span id="search_box_text_changed">`search_box_text_changed`</span>

```rs
pub fn search_box_text_changed(
    text_query: Query<&CosmicEdit, With<SearchText>>,
    mut velo_border: Query<(&mut Stroke, &VeloBorder), With<VeloBorder>>,
    mut previous_search_text: Local<String>,
    mut app_state: ResMut<AppState>,
    pkv: Res<PkvStore>,
    theme: Res<Theme>,
) {}
```

## <span id="select_tab_handler">`select_tab_handler`</span>

```rs
pub fn select_tab_handler(
    mut commands: Commands,
    mut interaction_query: Query<
        (&Interaction, &TabButton),
        (Changed<Interaction>, With<TabButton>),
    >,
    mut state: ResMut<AppState>,
) {}
```

## <span id="set_focused_entity">`set_focused_entity`</span>

```rs
pub fn set_focused_entity(
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
    mut node_interaction_events: EventReader<NodeInteraction>,
    mut ui_state: ResMut<UiState>,
    velo: Query<&RawText, With<RawText>>,
) {}
```

## <span id="set_window_property">`set_window_property`</span>

```rs
pub fn set_window_property(mut app_state: ResMut<AppState>, mut pkv: ResMut<PkvStore>) {}
```

## <span id="setup_background">`setup_background`</span>

```rs
pub fn setup_background(mut commands: Commands, asset_server: Res<AssetServer>, theme: Res<Theme>) {}
```

## <span id="setup_camera">`setup_camera`</span>

```rs
pub fn setup_camera(mut commands: Commands) {}
```

## <span id="setup_velo_theme">`setup_velo_theme`</span>

```rs
pub fn setup_velo_theme(mut commands: Commands, pkv: Res<PkvStore>) {}
```

## <span id="shared_doc_handler">`shared_doc_handler`</span>

```rs
pub fn shared_doc_handler(
    mut app_state: ResMut<AppState>,
    mut query: Query<&Interaction, (Changed<Interaction>, With<ShareDoc>)>,
    mut pkv: ResMut<PkvStore>,
) {}
```

## <span id="should_load_doc">`should_load_doc`</span>

```rs
pub fn should_load_doc(request: Option<Res<LoadDocRequest>>) -> bool {}
```

## <span id="should_load_tab">`should_load_tab`</span>

```rs
pub fn should_load_tab(request: Option<Res<LoadTabRequest>>) -> bool {}
```

## <span id="should_save_doc">`should_save_doc`</span>

```rs
pub fn should_save_doc(request: Option<Res<SaveDocRequest>>) -> bool {}
```

## <span id="should_save_tab">`should_save_tab`</span>

```rs
pub fn should_save_tab(request: Option<Res<SaveTabRequest>>) -> bool {}
```

## <span id="update_rectangle_position">`update_rectangle_position`</span>

```rs
pub fn update_rectangle_position(
    mut cursor_moved_events: EventReader<CursorMoved>,
    raw_text_query: Query<(&RawText, &Parent), With<RawText>>,
    border_query: Query<&Parent, With<VeloBorder>>,
    mut velo_node_query: Query<&mut Transform, With<VeloNode>>,
    mut events: EventWriter<RedrawArrow>,
    camera_q: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    ui_state: Res<UiState>,
) {}
```
