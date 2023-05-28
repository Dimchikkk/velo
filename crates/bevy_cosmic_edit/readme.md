# bevy_cosmic_edit

This bevy plugin provides mutliline text editing for bevy, thanks to [cosmic_text](https://github.com/pop-os/cosmic-text) crate!

Emoji, ligatures, and other fancy stuff is supported!

![bevy_cosmic_edit](./bevy_cosmic_edit.png)

## Usage

Simply add the plugin to your app:

```rust
.add_plugin(CosmicEditPlugin)
```

and insert `CosmicFontConfig` resource:

```rust
.insert_resource(CosmicFontConfig {
    fonts_dir_path: None,
    load_system_fonts: false,
    monospace_family: Some("Source Code Pro".to_string()),
    sans_serif_family: Some("Source Code Pro".to_string()),
    serif_family: Some("Source Code Pro".to_string()),
    custom_font_data: Some(CustomCosmicFont {
        data: include_bytes!("../assets/fonts/SourceCodePro-Regular.ttf"),
        override_bevy_font: true,
    }),
});
```

Specify directory with fonts, or use system fonts, or use custom font data.

Then spawn cosmic edit UI node:

```rust
fn my_bevy_system(
    ...,
    mut font_system_state: ResMut<FontSystemState>,
    windows: Query<&Window, With<PrimaryWindow>>,
) {
    ...
    let primary_window: &Window = windows.single();
    let font_system = font_system_state.font_system.as_mut().unwrap();
    let cosmic_edit_meta = CosmicEditMeta {
        text: "Hello world".to_string(),
        font_size: 14.,
        line_height: 18.,
        scale_factor: primary_window.scale_factor() as f32,
        font_system,
        is_visible: true,
        initial_background: None,
        text_pos: CosmicTextPos::Center,
        initial_size: Some((180., 35.)),
    };
    let cosmic_edit = spawn_cosmic_edit(&mut commands, cosmic_edit_meta);
    // and attach cosmic_edit to any bevy UI node
    commands.entity(top).add_child(cosmic_edit);
}
```

|bevy|bevy\_cosmic_edit|
|----|---|
|0.10|0.2|

## License

MIT or Apache-2.0