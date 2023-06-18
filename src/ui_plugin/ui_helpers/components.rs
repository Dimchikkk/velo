use crate::{ui_plugin::NodeType, utils::ReflectableUuid};
use bevy::prelude::*;
use bevy_markdown::TextSpanMetadata;

use crate::TextPos;

#[derive(Component)]
pub struct GenericButton;

#[derive(Component)]
pub struct Root;

#[derive(Component)]
pub struct Menu;

#[derive(Component, Clone)]
pub struct AddTab;

#[derive(Component)]
pub struct DeleteTab {
    pub id: ReflectableUuid,
}

#[derive(Component)]
pub struct Tooltip;

#[derive(Component, Clone)]
pub struct NewDoc;

#[derive(Component)]
pub struct ParticlesEffect;

#[derive(Component)]
pub struct DocList;

#[derive(Component, Clone)]
pub struct SaveDoc;

#[derive(Component, Clone)]
pub struct ExportToFile;

#[derive(Component)]
pub struct SetWindowProperty;

#[derive(Component, Clone)]
pub struct ImportFromFile;

#[derive(Component, Clone)]
pub struct ImportFromUrl;

#[derive(Component, Clone)]
pub struct ShareDoc;

#[derive(Component, Clone)]
pub struct ChangeTheme;

#[derive(Component)]
pub struct DeleteDoc {
    pub id: ReflectableUuid,
}

#[derive(Component)]
pub struct TabButton {
    pub id: ReflectableUuid,
}
#[derive(Component)]
pub struct TabContainer {
    pub id: ReflectableUuid,
}

#[derive(Component)]
pub struct SearchButton {
    pub id: ReflectableUuid,
}

#[derive(Component)]
pub struct SearchText {
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

#[derive(Component)]
pub struct VeloBorder {
    pub id: ReflectableUuid,
    pub node_type: NodeType,
}

#[derive(Component, Default, Debug)]
pub struct VeloNode {
    pub id: ReflectableUuid,
}

#[derive(PartialEq, Eq)]
pub enum ButtonTypes {
    AddRec,
    AddCircle,
    AddPaper,
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

#[derive(Component, Default)]
pub struct RawText {
    pub id: ReflectableUuid,
    pub last_text: String,
}

#[derive(Component, Default)]
pub struct BevyMarkdownView {
    pub id: ReflectableUuid,
    pub span_metadata: Vec<TextSpanMetadata>,
}

#[derive(Component, Copy, Clone, Debug, Default)]
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

#[derive(Component)]
pub struct InteractiveNode;

#[derive(Component)]
pub struct Background;

#[derive(Component)]
pub struct VeloShadow {
    pub id: ReflectableUuid,
}
