use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
#[derive(Clone, Reflect, Default, Debug, Copy, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[reflect_value]
pub struct ReflectableUuid(pub Uuid);

impl ReflectableUuid {
    pub fn generate() -> Self {
        let uuid = uuid::Uuid::new_v4();
        Self(uuid)
    }
}
