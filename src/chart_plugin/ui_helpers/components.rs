use serde::{Deserialize, Serialize};

use bevy::prelude::*;

use uuid::Uuid;

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
pub struct RenameTab;

#[derive(Component)]
pub struct Tooltip;

#[derive(Component)]
pub struct NewDoc;

#[derive(Component)]
pub struct DocList;

#[derive(Component)]
pub struct RenameDoc;

#[derive(Component)]
pub struct SaveDoc;

#[derive(Component)]
pub struct DeleteDoc;

#[derive(Component)]
pub struct SelectedTab {
    pub id: ReflectableUuid,
}

#[derive(Component)]
pub struct SelectedTabContainer {
    pub id: ReflectableUuid,
}

#[derive(Component, Default)]
pub struct ScrollingList {
    pub position: f32,
}

#[derive(Component)]
pub struct SelectedTabTextInput {
    pub id: ReflectableUuid,
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
pub struct DocListItemText {
    pub id: ReflectableUuid,
}

#[derive(Component)]
pub struct ChangeColor {
    pub color: Color,
}

#[derive(Serialize, Deserialize, Default, Copy, Clone, Reflect, Debug, Eq, PartialEq, Hash)]
pub enum ArrowType {
    Line,
    Arrow,
    DoubleArrow,
    ParallelLine,
    #[default]
    ParallelArrow,
    ParallelDoubleArrow,
}

#[derive(Component)]
pub struct ArrowMode {
    pub arrow_type: ArrowType,
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

#[derive(Eq, PartialEq, Hash, Debug, Clone, Copy)]
pub struct CreateArrow {
    pub arrow_type: ArrowType,
    pub start: ArrowConnect,
    pub end: ArrowConnect,
}

#[derive(Clone, Reflect, Default, Debug, Copy, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[reflect_value]
pub struct ReflectableUuid(pub Uuid);

#[derive(Component, Default, Reflect, Debug)]
#[reflect(Component)]
pub struct Rectangle {
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

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Reflect, Default, Serialize, Deserialize)]
pub enum ArrowConnectPos {
    #[default]
    Top,
    Bottom,
    Left,
    Right,
}

#[derive(
    Component, Copy, Clone, Debug, Eq, PartialEq, Hash, Reflect, Default, Serialize, Deserialize,
)]
#[reflect(Component)]
pub struct ArrowConnect {
    pub id: ReflectableUuid,
    pub pos: ArrowConnectPos,
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

#[derive(
    Component, Copy, Clone, Debug, Eq, PartialEq, Hash, Reflect, Default, Serialize, Deserialize,
)]
#[reflect(Component)]
pub struct ArrowMeta {
    pub arrow_type: ArrowType,
    pub start: ArrowConnect,
    pub end: ArrowConnect,
}

#[derive(Component)]
pub struct ModalTop {
    pub id: ReflectableUuid,
    pub delete: ModalEntity,
}

#[derive(Eq, PartialEq, Clone)]
pub enum ModalEntity {
    Document,
    Tab,
}

impl std::fmt::Display for ModalEntity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ModalEntity::Document => write!(f, "document"),
            ModalEntity::Tab => write!(f, "tab"),
        }
    }
}

#[derive(Component)]
pub struct ModalConfirm {
    pub id: ReflectableUuid,
    pub delete: ModalEntity,
}

#[derive(Component, Default)]
pub struct ModalCancel {
    pub id: ReflectableUuid,
}
