use bevy::prelude::*;

#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
pub struct IRectangle {
    pub id: u32,
}

#[derive(Component)]
pub struct CreateRectButton;

#[derive(Component)]
pub struct EditableText {
    pub id: u32,
}

#[derive(Component)]
pub struct Top {
    pub id: u32,
}
pub struct AddRect;

#[derive(Resource, Default)]
pub struct AppState {
    pub focused_id: Option<u32>,
    pub entity_counter: u32,
    pub entity_to_resize: Option<(u32, Vec2)>,
}

#[derive(Component)]
pub struct ArrowConnectMarker;

#[derive(Component)]
pub struct ResizeMarker;
