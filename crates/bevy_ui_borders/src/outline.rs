use bevy::prelude::*;
use bevy::render::Extract;
use bevy::ui::ExtractedUiNode;
use bevy::ui::ExtractedUiNodes;
use bevy::ui::FocusPolicy;
use bevy::ui::UiStack;

use crate::resolve_thickness;


/// Generates the outline geometry
#[allow(clippy::type_complexity)]
pub(crate) fn calculate_outlines(
    parent_query: Query<&Node, With<Children>>,
    mut outline_query: Query<
        (&Node, &Outline, Option<&Parent>),
        (Or<(Changed<Node>, Changed<Outline>, Changed<Parent>)>,),
    >,
) {
    for (node, outline, mut calculated_outline, parent) in outline_query.iter_mut() {
        let parent_width = parent
            .and_then(|parent| parent_query.get(parent.get()).ok())
            .map(|parent_node| parent_node.size().x)
            .unwrap_or(0.);
        let left = resolve_thickness(outline.left, parent_width);
        let right = resolve_thickness(outline.right, parent_width);
        let top = resolve_thickness(outline.top, parent_width);
        let bottom = resolve_thickness(outline.bottom, parent_width);

        // calculate outline rects, ensuring that they don't overlap
        let half_size = 0.5 * node.size();
        let min = -Vec2::new(half_size.x + left, half_size.y + top);
        let max = Vec2::new(half_size.x + right, half_size.y + bottom);
        let inner_min = min + Vec2::new(left, top);
        let inner_max = (max - Vec2::new(right, bottom)).max(inner_min);

        let outline_rects = [
            // Left outline
            Rect {
                min,
                max: Vec2::new(inner_min.x, max.y),
            },
            // Right outline
            Rect {
                min: Vec2::new(inner_max.x, min.y),
                max,
            },
            // Top outline
            Rect {
                min: Vec2::new(inner_min.x, min.y),
                max: Vec2::new(inner_max.x, inner_min.y),
            },
            // Bottom outline
            Rect {
                min: Vec2::new(inner_min.x, inner_max.y),
                max: Vec2::new(inner_max.x, max.y),
            },
        ];

        for (i, edge) in outline_rects.into_iter().enumerate() {
            calculated_outline.edges[i] = if edge.min.x < edge.max.x && edge.min.y < edge.max.y {
                Some(edge)
            } else {
                None
            };
        }
    }
}

#[allow(clippy::type_complexity)]
pub(crate) fn extract_uinode_outlines(
    mut extracted_uinodes: ResMut<ExtractedUiNodes>,
    ui_stack: Extract<Res<UiStack>>,
    uinode_query: Extract<
        Query<
            (
                &GlobalTransform,
                &Outline,
                &ComputedVisibility,
                Option<&CalculatedClip>,
            ),
            Without<CalculatedSize>,
        >,
    >,
) {
    let image = bevy::render::texture::DEFAULT_IMAGE_HANDLE.typed();

    for (stack_index, entity) in ui_stack.uinodes.iter().enumerate() {
        if let Ok((global_transform, outline, visibility, clip)) =
            uinode_query.get(*entity)
        {
            // Skip invisible nodes
            if !visibility.is_visible() || outline_color.a() == 0.0 {
                continue;
            }

            let transform = global_transform.compute_matrix();

            for &outline_rect in calculated_outline.edges.iter().flatten() {
                extracted_uinodes.uinodes.push(ExtractedUiNode {
                    stack_index,
                    transform: transform * Mat4::from_translation(outline_rect.center().extend(0.)),
                    color: **outline_color,
                    rect: Rect {
                        max: outline_rect.size(),
                        ..Default::default()
                    },
                    image: image.clone_weak(),
                    atlas_size: None,
                    clip: clip.map(|clip| clip.clip),
                    flip_x: false,
                    flip_y: false,
                });
            }
        }
    }
}
