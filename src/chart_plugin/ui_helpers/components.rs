use crate::utils::ReflectableUuid;
use bevy::prelude::*;

use crate::TextPos;

#[derive(Component)]
pub struct GenericButton;

#[derive(Component)]
pub struct Root;

#[derive(Component)]
pub struct Menu;

#[derive(Component)]
pub struct AddTab;

#[derive(Component)]
pub struct DeleteTab;

#[derive(Component)]
pub struct Tooltip;

#[derive(Component)]
pub struct NewDoc;

#[derive(Component)]
pub struct DocList;

#[derive(Component)]
pub struct SaveDoc;

#[derive(Component)]
pub struct ExportToFile;

#[derive(Component)]
pub struct SetWindowProperty;

#[derive(Component)]
pub struct ImportFromFile;

#[derive(Component)]
pub struct ImportFromUrl;

#[derive(Component)]
pub struct DeleteDoc {
    pub id: ReflectableUuid,
}

#[derive(Component)]
pub struct TabButton {
    pub id: ReflectableUuid,
}

#[derive(Component)]
pub struct TabText {
    pub id: ReflectableUuid,
}

#[derive(Component)]
pub struct TabContainer {
    pub id: ReflectableUuid,
}

#[derive(Component, Default)]
pub struct ScrollingList {
    pub position: f32,
}

#[derive(Component)]
pub struct DocListItemContainer {
    pub id: ReflectableUuid,
}

#[derive(Component)]
pub struct DocListItemButton {
    pub id: ReflectableUuid,
}

#[derive(Component)]
pub struct ChangeColor {
    pub color: Color,
}

#[derive(Component)]
pub struct TextPosMode {
    pub text_pos: TextPos,
}

pub enum TextManipulation {
    Cut,
    Paste,
    Copy,
    OpenAllLinks,
}

#[derive(Component)]
pub struct TextManipulationAction {
    pub action_type: TextManipulation,
}

#[derive(Component)]
pub struct MainPanel;

#[derive(Component)]
pub struct BottomPanel;

#[derive(Component)]
pub struct LeftPanel;

#[derive(Component)]
pub struct LeftPanelControls;

#[derive(Component)]
pub struct LeftPanelExplorer;

#[derive(Component, Default, Reflect, Debug)]
#[reflect(Component)]
pub struct VeloNodeContainer {
    pub id: ReflectableUuid,
}

#[derive(Component, Default, Reflect, Debug)]
#[reflect(Component)]
pub struct VeloNode {
    pub id: ReflectableUuid,
}

#[derive(PartialEq, Eq)]
pub enum ButtonTypes {
    Add,
    Del,
    Front,
    Back,
}
#[derive(Component)]
pub struct ButtonAction {
    pub button_type: ButtonTypes,
}

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct EditableText {
    pub id: ReflectableUuid,
}

#[derive(Component, Copy, Clone, Debug, Reflect, Default)]
#[reflect(Component)]
pub enum ResizeMarker {
    #[default]
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

#[derive(Component)]
pub struct ModalTop {
    pub id: ReflectableUuid,
    pub action: ModalAction,
}

#[derive(Eq, PartialEq, Clone)]
pub enum ModalAction {
    SaveToFile,
    LoadFromFile,
    LoadFromUrl,
    DeleteDocument,
    DeleteTab,
}

impl std::fmt::Display for ModalAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ModalAction::DeleteDocument => write!(f, "delete document"),
            ModalAction::DeleteTab => write!(f, "delete tab"),
            ModalAction::LoadFromFile => write!(f, "Load from file:"),
            ModalAction::LoadFromUrl => write!(f, "Load from URL:"),
            ModalAction::SaveToFile => write!(f, "Save to file:"),
        }
    }
}

#[derive(Component)]
pub struct ModalConfirm {
    pub id: ReflectableUuid,
    pub action: ModalAction,
}

#[derive(Component, Default)]
pub struct ModalCancel {
    pub id: ReflectableUuid,
}
