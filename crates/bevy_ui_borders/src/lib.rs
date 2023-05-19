use bevy::prelude::*;
use bevy::render::Extract;
use bevy::ui::ExtractedUiNode;
use bevy::ui::ExtractedUiNodes;
use bevy::ui::FocusPolicy;
use bevy::ui::RenderUiSystem;
use bevy::ui::UiStack;

/// The color of a UI node's border.
#[derive(Component, Copy, Clone, Default, Debug, Deref, DerefMut, Reflect)]
#[reflect(Component)]
pub struct BorderColor(pub Color);

impl From<Color> for BorderColor {
    fn from(color: Color) -> Self {
        Self(color)
    }
}

/// Outline around the UI node's border that doesn't occupy any space in the UI layout.
#[derive(Component, Copy, Clone, Default, Debug, Reflect)]
#[reflect(Component)]
pub struct Outline {
    pub color: Color,
    pub thickness: UiRect,
}

impl Outline {
    pub fn new(color: Color, thickness: UiRect) -> Self {
        Self { color, thickness }
    }

    pub fn all(color: Color, thickness: Val) -> Self {
        Self {
            color,
            thickness: UiRect::all(thickness),
        }
    }
}

/// The basic UI node but with a Border and Outline
///
/// Useful as a container for a variety of child nodes.
#[derive(Bundle, Clone, Debug)]
pub struct BorderedNodeBundle {
    /// Describes the logical size of the node
    pub node: Node,
    /// Describes the style including flexbox settings
    pub style: Style,
    /// The background color, which serves as a "fill" for this node
    pub background_color: BackgroundColor,
    /// Whether this node should block interaction with lower nodes
    pub focus_policy: FocusPolicy,
    /// The transform of the node
    ///
    /// This field is automatically managed by the UI layout system.
    /// To alter the position of the `nodebundle`, use the properties of the [`Style`] component.
    pub transform: Transform,
    /// The global transform of the node
    ///
    /// This field is automatically managed by the UI layout system.
    /// To alter the position of the `NodeBundle`, use the properties of the [`Style`] component.
    pub global_transform: GlobalTransform,
    /// Describes the visibility properties of the node
    pub visibility: Visibility,
    /// Algorithmically-computed indication of whether an entity is visible and should be extracted for rendering
    pub computed_visibility: ComputedVisibility,
    /// Indicates the depth at which the node should appear in the UI
    pub z_index: ZIndex,
    /// The color of the node's border.
    pub border_color: BorderColor,
    /// The thickness and color of the outline
    pub outline: Outline,
}

impl Default for BorderedNodeBundle {
    fn default() -> Self {
        BorderedNodeBundle {
            // Transparent background
            background_color: Color::NONE.into(),
            node: Default::default(),
            style: Default::default(),
            focus_policy: Default::default(),
            transform: Default::default(),
            global_transform: Default::default(),
            visibility: Default::default(),
            computed_visibility: Default::default(),
            z_index: Default::default(),
            border_color: Color::WHITE.into(),
            outline: Default::default(),
        }
    }
}

/// Add a border bundle to a ui node to draw its border
#[derive(Bundle, Copy, Clone, Default)]
pub struct BorderBundle {
    /// The color of the node's border.
    pub border_color: BorderColor,
    /// The color and thickness of the node's outline
    pub outline: Outline,
}

/// Percentage thickness of all border edges is calculated based on the width of the parent node.
fn resolve_thickness(value: Val, parent_width: f32) -> f32 {
    match value {
        Val::Auto => 0.,

        Val::Px(px) => px.max(0.),
        Val::Percent(percent) => (parent_width * percent / 100.).max(0.),
        _=>0.
    }
}

const fn edge_rects(min: Vec2, max: Vec2, inner_min: Vec2, inner_max: Vec2) -> [Rect; 4] {
    [
        // Left
        Rect {
            min,
            max: Vec2::new(inner_min.x, max.y),
        },
        // Right
        Rect {
            min: Vec2::new(inner_max.x, min.y),
            max,
        },
        // Top
        Rect {
            min: Vec2::new(inner_min.x, min.y),
            max: Vec2::new(inner_max.x, inner_min.y),
        },
        // Bottom
        Rect {
            min: Vec2::new(inner_min.x, inner_max.y),
            max: Vec2::new(inner_max.x, max.y),
        },
    ]
}

#[allow(clippy::type_complexity)]
fn extract_uinode_borders(
    mut extracted_uinodes: ResMut<ExtractedUiNodes>,
    ui_stack: Extract<Res<UiStack>>,
    uinode_query: Extract<
        Query<
            (
                &Node,
                &GlobalTransform,
                &Style,
                Option<&BorderColor>,
                Option<&Outline>,
                Option<&Parent>,
                &ComputedVisibility,
                Option<&CalculatedClip>,
            ),
            Without<CalculatedClip>,
        >,
    >,
    parent_node_query: Extract<Query<&Node, With<Parent>>>,
) {
    let image = bevy::render::texture::DEFAULT_IMAGE_HANDLE.typed();

    for (stack_index, entity) in ui_stack.uinodes.iter().enumerate() {
        if let Ok((
            node,
            global_transform,
            style,
            maybe_border_color,
            maybe_outline,
            parent,
            visibility,
            clip,
        )) = uinode_query.get(*entity)
        {
            if !visibility.is_visible() || node.size().x <= 0. || node.size().y <= 0. {
                continue;
            }

            // calculate border rects, ensuring that they don't overlap
            let transform = global_transform.compute_matrix();

            let mut maybe_parent_width = None;
            let get_parent_width = || {
                parent
                    .and_then(|parent| parent_node_query.get(parent.get()).ok())
                    .map(|parent_node| parent_node.size().x)
                    .unwrap_or(0.)
            };

            if let Some(border_color) =
                maybe_border_color.filter(|border_color| border_color.a() != 0.)
            {
                if border_color.a() != 0. {
                    let parent_width = get_parent_width();
                    maybe_parent_width = parent_width.into();

                    // calculate border rects, ensuring no overlap
                    let left = resolve_thickness(style.border.left, parent_width);
                    let right = resolve_thickness(style.border.right, parent_width);
                    let top = resolve_thickness(style.border.top, parent_width);
                    let bottom = resolve_thickness(style.border.bottom, parent_width);
                    let max = 0.5 * node.size();
                    let min = -max;
                    let inner_min = min + Vec2::new(left, top);
                    let inner_max = (max - Vec2::new(right, bottom)).max(inner_min);
                    let border_rects = edge_rects(min, max, inner_min, inner_max);

                    for edge in border_rects {
                        if edge.min.x < edge.max.x && edge.min.y < edge.max.y {
                            extracted_uinodes.uinodes.push(ExtractedUiNode {
                                stack_index,
                                transform: transform
                                    * Mat4::from_translation(edge.center().extend(0.)),
                                color: **border_color,
                                rect: Rect {
                                    max: edge.size(),
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

            if let Some(outline) = maybe_outline.filter(|outline| outline.color.a() != 0.) {
                let parent_width = maybe_parent_width.unwrap_or_else(get_parent_width);
                let left = resolve_thickness(outline.thickness.left, parent_width);
                let right = resolve_thickness(outline.thickness.right, parent_width);
                let top = resolve_thickness(outline.thickness.top, parent_width);
                let bottom = resolve_thickness(outline.thickness.bottom, parent_width);

                // calculate outline rects, ensuring that they don't overlap
                let half_size = 0.5 * node.size();
                let min = -Vec2::new(half_size.x + left, half_size.y + top);
                let max = Vec2::new(half_size.x + right, half_size.y + bottom);
                let inner_min = min + Vec2::new(left, top);
                let inner_max = (max - Vec2::new(right, bottom)).max(inner_min);
                let outline_rects = edge_rects(min, max, inner_min, inner_max);

                for edge in outline_rects {
                    extracted_uinodes.uinodes.push(ExtractedUiNode {
                        stack_index,
                        transform: transform * Mat4::from_translation(edge.center().extend(0.)),
                        color: outline.color,
                        rect: Rect {
                            max: edge.size(),
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
}

pub struct BordersPlugin;

impl Plugin for BordersPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<BorderColor>()
            .register_type::<Outline>();

        let render_app = match app.get_sub_app_mut(bevy::render::RenderApp) {
            Ok(render_app) => render_app,
            Err(_) => return,
        };

        render_app.add_systems(
            ExtractSchedule,
            extract_uinode_borders
                .after(RenderUiSystem::ExtractNode)
            
        );
    }
}
