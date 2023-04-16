use crate::utils::ReflectableUuid;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};
#[derive(
    Component, Copy, Clone, Debug, Eq, PartialEq, Hash, Reflect, Default, Serialize, Deserialize,
)]
#[reflect(Component)]
pub struct ArrowMeta {
    pub arrow_type: ArrowType,
    pub start: ArrowConnect,
    pub end: ArrowConnect,
}
#[derive(
    Component, Copy, Clone, Debug, Eq, PartialEq, Hash, Reflect, Default, Serialize, Deserialize,
)]
#[reflect(Component)]
pub struct ArrowConnect {
    pub id: ReflectableUuid,
    pub pos: ArrowConnectPos,
}

#[derive(Component)]
pub struct ArrowMode {
    pub arrow_type: ArrowType,
}
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Reflect, Default, Serialize, Deserialize)]
pub enum ArrowConnectPos {
    #[default]
    Top,
    Bottom,
    Left,
    Right,
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
