use std::collections::{HashMap, VecDeque};

use bevy::{
    a11y::{
        accesskit::{NodeBuilder, Role},
        AccessibilityNode,
    },
    prelude::*,
};
use bevy_pkv::PkvStore;

use super::ui_helpers::ScrollingList;
use crate::chart_plugin::ui_helpers::{add_list_item, DocList};
use crate::components::{Doc, Tab};
use crate::resources::{AppState, LoadDocRequest};
use crate::utils::ReflectableUuid;

pub fn add_list(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    state: &mut ResMut<AppState>,
    pkv: &mut ResMut<PkvStore>,
) -> Entity {
    if let Ok(last_saved) = pkv.get::<ReflectableUuid>("last_saved") {
        state.current_document = Some(last_saved);
        commands.insert_resource(LoadDocRequest { doc_id: last_saved });
    }

    let top = commands
        .spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                align_self: AlignSelf::Stretch,
                width:Val::Percent(80.),height: Val::Percent(80.),
                //overflow: Overflow::Hidden,
                ..default()
            },
            background_color: Color::rgb(158., 158., 158.).into(),
            ..default()
        })
        .id();
    let node = commands
        .spawn((
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    // max_size: Size::UNDEFINED,
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
            let button = add_list_item(
                commands,
                None,
                asset_server,
                *key,
                name.clone(),
                state.current_document == Some(*key),
            );
            commands.entity(node).add_child(button);
        }
    } else {
        let tab_id = ReflectableUuid::generate();
        let tab_name: String = "Tab 1".to_string();
        let tabs = vec![Tab {
            id: tab_id,
            name: tab_name,
            checkpoints: VecDeque::new(),
            is_active: true,
        }];
        let doc_id = ReflectableUuid::generate();
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
        let button = add_list_item(commands, None, asset_server, doc_id, name, true);
        state.current_document = Some(doc_id);
        commands.entity(node).add_child(button);
        commands.insert_resource(LoadDocRequest { doc_id });
    }
    commands.entity(top).add_child(node);
    top
}
