use bevy::prelude::Event;

use super::components::{ArrowConnect, ArrowType};
use crate::utils::ReflectableUuid;

#[derive(Event)]
pub struct RedrawArrow {
    pub id: ReflectableUuid,
}
#[derive(Event, Eq, PartialEq, Hash, Debug, Clone, Copy)]
pub struct CreateArrow {
    pub visible: bool,
    pub arrow_type: ArrowType,
    pub start: ArrowConnect,
    pub end: ArrowConnect,
}
