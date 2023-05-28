# bevy_cosmic_edit

This bevy plugin provides mutliline text editing for bevy apps, thanks to [cosmic_text](https://github.com/pop-os/cosmic-text) crate!

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

Then spawn cosmic-edit UI node:

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
    // attach cosmic_edit to any Bevy UI node
    commands.entity(top).add_child(cosmic_edit);
    // `ActiveEditor` resource controls which cosmic-edit UI node is active (may be useful when window has multiple cosmic-edit nodes)
    commands.insert_resource(ActiveEditor { entity: Some(cosmic_edit) });
}
```

Query created cosmic_entity in another system:
```rust
fn my_bevy_another_system(
    ...,
    cosmic_edit_query: Query<&CosmicEditImage, With<CosmicEditImage>>,
) {
    ...
    for cosmic_edit in cosmic_edit_query.iter() {
        let text = get_cosmic_text(&cosmic_edit.editor);
        ...
    }
}
```


`get_cosmic_text` is a little helper function that returns `String` from cosmic-text `Editor`. `Editor` exposes cosmic-text API, so you can use it directly.



## Examples

For complete examples explore how the plugin is used in https://github.com/StaffEngineer/velo


## Compatibility

| bevy | bevy\_cosmic_edit |
| ---- | ----------------- |
| 0.10 | 0.2               |

## License

MIT or Apache-2.0