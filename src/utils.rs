use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
#[derive(Clone, Reflect, Default, Debug, Copy, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[reflect_value]
pub struct ReflectableUuid(pub Uuid);
