use crate::utils::ReflectableUuid;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
#[derive(Component)]
pub struct MainCamera;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Tab {
    pub is_active: bool,
    pub id: ReflectableUuid,
    pub name: String,
    pub checkpoints: VecDeque<String>,
}

#[derive(Default, Serialize, Deserialize, Clone, Debug)]
pub struct Doc {
    pub tabs: Vec<Tab>,
    pub id: ReflectableUuid,
    pub name: String,
    pub tags: Vec<String>,
}
