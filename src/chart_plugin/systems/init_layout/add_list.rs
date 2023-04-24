use std::collections::{HashMap, VecDeque};

use bevy::{
    a11y::{
        accesskit::{NodeBuilder, Role},
        AccessibilityNode,
    },
    prelude::*,
};
use bevy_pkv::PkvStore;
use uuid::Uuid;

use super::ui_helpers::ScrollingList;
use crate::chart_plugin::ui_helpers::{add_list_item, add_tab, DocList};
use crate::components::{Doc, Tab};
use crate::resources::{AppState, LoadRequest};
use crate::utils::ReflectableUuid;

pub fn add_list(
    bottom_panel: Entity,
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    state: &mut ResMut<AppState>,
    pkv: &mut ResMut<PkvStore>,
) -> Entity {
    if let Ok(last_saved) = pkv.get::<ReflectableUuid>("last_saved") {
        state.current_document = Some(last_saved);
        commands.insert_resource(LoadRequest {
            doc_id: Some(last_saved),
            drop_last_checkpoint: false,
        });
    }

    let top = commands
        .spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                align_self: AlignSelf::Stretch,
                size: Size::new(Val::Percent(80.), Val::Percent(80.)),
                overflow: Overflow::Hidden,
                ..default()
            },
            background_color: Color::rgb(212.0 / 255.0, 225.0 / 255.0, 87.0 / 255.0).into(),
            ..default()
        })
        .id();
    let node = commands
        .spawn((
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    max_size: Size::UNDEFINED,
                    align_items: AlignItems::Center,
                    ..default()
                },
                ..default()
            },
            DocList,
            ScrollingList::default(),
            AccessibilityNode(NodeBuilder::new(Role::List)),
        ))
        .id();

    if let Ok(names) = pkv.get::<HashMap<ReflectableUuid, String>>("names") {
        let mut keys: Vec<_> = names.keys().collect();
        keys.sort_by_key(|k| names.get(k).unwrap().to_lowercase());

        for key in keys {
            let name = names.get(key).unwrap();
            let button = add_list_item(commands, asset_server, *key, name.clone());
            commands.entity(node).add_child(button);
        }
    } else {
        let tab_id = ReflectableUuid(Uuid::new_v4());
        let tab_name: String = "Tab 1".to_string();
        let tabs = vec![Tab {
            id: tab_id,
            name: tab_name.clone(),
            checkpoints: VecDeque::new(),
            is_active: true,
        }];
        let doc_id = ReflectableUuid(Uuid::new_v4());
        let name = "Untitled".to_string();
        state.docs.insert(
            doc_id,
            Doc {
                id: doc_id,
                name: name.clone(),
                tabs,
                tags: vec![],
            },
        );
        let button = add_list_item(commands, asset_server, doc_id, name);
        state.current_document = Some(doc_id);
        let tab_view = add_tab(commands, asset_server, tab_name, tab_id);
        commands.entity(bottom_panel).add_child(tab_view);
        commands.entity(node).add_child(button);
    }
    commands.entity(top).add_child(node);
    top
}
