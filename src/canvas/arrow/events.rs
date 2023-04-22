use super::components::{ArrowConnect, ArrowType};
use crate::utils::ReflectableUuid;
pub struct RedrawArrowEvent {
    pub id: ReflectableUuid,
}
#[derive(Eq, PartialEq, Hash, Debug, Clone, Copy)]
pub struct CreateArrowEvent {
    pub arrow_type: ArrowType,
    pub start: ArrowConnect,
    pub end: ArrowConnect,
}
