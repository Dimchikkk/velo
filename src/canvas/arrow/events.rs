use super::components::{ArrowConnect, ArrowType};
use crate::utils::ReflectableUuid;
pub struct RedrawArrow {
    pub id: ReflectableUuid,
}
#[derive(Eq, PartialEq, Hash, Debug, Clone, Copy)]
pub struct CreateArrow {
    pub arrow_type: ArrowType,
    pub start: ArrowConnect,
    pub end: ArrowConnect,
}
