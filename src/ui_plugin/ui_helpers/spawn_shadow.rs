use bevy_smud::prelude::SdfAssets;
use bevy_smud::{Frame, ShapeBundle};

use bevy::prelude::*;

use crate::utils::ReflectableUuid;

use super::VeloShadow;

pub fn spawn_shadow(
    commands: &mut Commands,
    shaders: &mut ResMut<Assets<Shader>>,
    width: f32,
    height: f32,
    shadow_color: Color,
    id: ReflectableUuid,
) -> Entity {
    let fill = shaders.add_fill_body(format!(
        "
        let size = {:.2};
        let power = 12.0;
        var a = (size - d) / size;
        a = clamp(a, 0.0, 1.0);
        a = pow(a, power);
        return vec4<f32>(color.rgb, a * color.a);
    ",
        width
    ));
    let sdf_expr = format!(
        "sd_box(p, vec2<f32>({:.2}, {:.2}))",
        0.7 * (width / 2.),
        0.7 * (height / 2.),
    );
    let sdf = shaders.add_sdf_expr(sdf_expr);
    let translation = Vec3::new(-0.025 * width, -0.10 * height, 0.09);

    let shadow = commands
        .spawn((
            ShapeBundle {
                transform: Transform {
                    translation,
                    ..default()
                },
                shape: bevy_smud::SmudShape {
                    color: shadow_color,
                    sdf,
                    fill,
                    frame: Frame::Quad(width),
                },
                ..default()
            },
            VeloShadow { id },
        ))
        .id();
    shadow
}
