#![allow(clippy::duplicate_mod)]
use bevy::prelude::*;
use bevy::render::view::RenderLayers;

use super::ui_helpers::ParticlesEffect;

use crate::components::EffectsCamera;

pub fn update_particles_effect(
    mut q_effect: Query<(&mut bevy_hanabi::EffectSpawner, &mut Transform), Without<Projection>>,
    mouse_button_input: Res<Input<MouseButton>>,
    window: Query<&Window, With<bevy::window::PrimaryWindow>>,
    mut effects_camera: Query<(&mut Camera, &GlobalTransform), With<EffectsCamera>>,
) {
    let (camera, camera_transform) = effects_camera.single_mut();
    if !camera.is_active {
        return;
    }
    // Note: On first frame where the effect spawns, EffectSpawner is spawned during
    // CoreSet::PostUpdate, so will not be available yet. Ignore for a frame if
    // so.
    let Ok((mut spawner, mut effect_transform)) = q_effect.get_single_mut() else { return; };

    if let Ok(window) = window.get_single() {
        if let Some(mouse_pos) = window.cursor_position() {
            if mouse_button_input.pressed(MouseButton::Left) {
                let ray = camera
                    .viewport_to_world(camera_transform, mouse_pos)
                    .unwrap();
                let spawning_pos = Vec3::new(ray.origin.x, ray.origin.y, 0.);

                effect_transform.translation = spawning_pos;

                // Spawn a single burst of particles
                spawner.reset();
            }
        }
    }
}

pub fn create_particles_effect(
    mut query: Query<(&Interaction, &Children), (Changed<Interaction>, With<ParticlesEffect>)>,
    mut text_style_query: Query<&mut Text, With<ParticlesEffect>>,
    mut commands: Commands,
    mut effects: ResMut<Assets<bevy_hanabi::EffectAsset>>,
    mut effects_camera: Query<&mut Camera, With<EffectsCamera>>,
    mut effects_query: Query<(&Name, Entity)>,
) {
    use bevy_hanabi::prelude::*;
    use rand::Rng;

    for (interaction, children) in &mut query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                for child in children.iter() {
                    if let Ok(mut text) = text_style_query.get_mut(*child) {
                        if effects_camera.single_mut().is_active {
                            text.sections[0].style.color = text.sections[0].style.color.with_a(0.5)
                        } else {
                            text.sections[0].style.color = text.sections[0].style.color.with_a(1.)
                        }
                    }
                }
                if effects_camera.single_mut().is_active {
                    effects_camera.single_mut().is_active = false;
                    for (name, entity) in effects_query.iter_mut() {
                        if name.as_str() == "effect:2d" {
                            commands.entity(entity).despawn_recursive();
                        }
                    }
                } else {
                    effects_camera.single_mut().is_active = true;
                    let mut gradient = Gradient::new();
                    let mut rng = rand::thread_rng();
                    gradient.add_key(
                        0.0,
                        Vec4::new(
                            rng.gen_range(0.0..1.0),
                            rng.gen_range(0.0..1.0),
                            rng.gen_range(0.0..1.0),
                            1.0,
                        ),
                    );
                    gradient.add_key(
                        1.0,
                        Vec4::new(
                            rng.gen_range(0.0..1.0),
                            rng.gen_range(0.0..1.0),
                            rng.gen_range(0.0..1.0),
                            0.0,
                        ),
                    );

                    let mut size_gradient1 = Gradient::new();
                    size_gradient1.add_key(0.0, Vec2::splat(0.008));
                    size_gradient1.add_key(0.3, Vec2::splat(0.012));
                    size_gradient1.add_key(1.0, Vec2::splat(0.0));
                    let writer = ExprWriter::new();
                    let lifetime = writer.lit(5.).uniform(writer.lit(10.)).expr();
                    let spawner = Spawner::rate(rng.gen_range(10.0..300.0).into());
                    let position_circle_modifier = SetPositionCircleModifier {
                        center: writer.lit(Vec3::ZERO).expr(),
                        axis: writer.lit(Vec3::Z).expr(),
                        radius: writer.lit(0.0001).expr(),
                        dimension: ShapeDimension::Surface,
                    };
                    let velocity_circle_modifier = SetVelocityCircleModifier {
                        center: writer.lit(Vec3::ZERO).expr(),
                        axis: writer.lit(Vec3::Z).expr(),
                        speed: writer.lit(0.05).uniform(writer.lit(0.1)).expr(),
                    };
                    let effect = effects.add(
                        EffectAsset::new(32768, spawner, writer.finish())
                            .with_name("Effect")
                            .init(position_circle_modifier)
                            .init(velocity_circle_modifier)
                            .init(SetAttributeModifier {
                                attribute: Attribute::LIFETIME,
                                value: lifetime,
                            })
                            .render(SizeOverLifetimeModifier {
                                gradient: size_gradient1,
                                screen_space_size: false,
                            })
                            .render(ColorOverLifetimeModifier { gradient }),
                    );

                    commands
                        .spawn(ParticleEffectBundle {
                            effect: ParticleEffect::new(effect),
                            ..default()
                        })
                        .insert(Name::new("effect:2d"))
                        .insert(RenderLayers::layer(2));
                }
            }
            Interaction::Hovered => {}
            Interaction::None => {}
        }
    }
}
