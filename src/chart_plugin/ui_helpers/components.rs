use serde::{Deserialize, Serialize};

use bevy::prelude::*;

use uuid::Uuid;

use crate::TextPos;

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
pub struct SelectedTab {
    pub id: ReflectableUuid,
}

#[derive(Component)]
pub struct SelectedTabTextInput {
    pub id: ReflectableUuid,
}

#[derive(Component)]
pub struct ChangeColor {
    pub color: Color,
}

#[derive(Serialize, Deserialize, Default, Copy, Clone, Reflect, Debug, Eq, PartialEq, Hash)]
pub enum ArrowType {
    Line,
    #[default]
    Arrow,
    DoubleArrow,
    ParallelLine,
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
pub struct SaveState;

#[derive(Component)]
pub struct LoadState;

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

#[derive(Component, Default)]
pub struct PathModalTop {
    pub id: ReflectableUuid,
}

#[derive(Component, Default)]
pub struct PathModalText {
    pub id: ReflectableUuid,
    pub save: bool,
}

#[derive(Component, Default)]
pub struct PathModalTextInput {
    pub id: ReflectableUuid,
    pub save: bool,
}

#[derive(Component, Default)]
pub struct PathModalConfirm {
    pub id: ReflectableUuid,
    pub save: bool,
}

#[derive(Component, Default)]
pub struct PathModalCancel {
    pub id: ReflectableUuid,
}
