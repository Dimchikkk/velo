# Velo 🚵 
![Rust](https://img.shields.io/badge/Rust-lang-000000.svg?style=flat&logo=rust)[![codecov](https://codecov.io/gh/StaffEngineer/velo/branch/main/graph/badge.svg?token=QGEKLM6ZDF)](https://codecov.io/gh/StaffEngineer/velo)

![alt text](velo.png "Velo")

## Demo

This app is primarily designed for native desktop platforms, and its
WebAssembly (wasm) target has a limited feature set. wasm target is best
suited for quick document sharing and editing, currently only landscape
mode is supported (tested on Chrome):

  [<https://staffengineer.github.io/velo?document=https://gist.githubusercontent.com/StaffEngineer/ccd3062ad10af32fba7e189f209509b2/raw/99345e1d2e5d66c0a269635d39f34cc5d6bdf253/velo.json>](https://staffengineer.github.io/velo?document=https://gist.githubusercontent.com/StaffEngineer/ccd3062ad10af32fba7e189f209509b2/raw/99345e1d2e5d66c0a269635d39f34cc5d6bdf253/velo.json)

## Inspiration

At work, I frequently rely on lucid.app to brainstorm ideas with my
colleagues or by myself. Typically, I share my ideas by sending either
the diagrams themselves or screenshots of them. While I tend to stick
with simple features like rectangles and arrows, I\'ve been
contemplating the idea of creating a similar tool in Rust. Not only
would it allow me to learn the language, but it would also be an
enjoyable project to work on.

## What\'s implemented:
-   support rectangle/circle nodes
-   add/remove node
-   node resizing
-   node repositioning
-   wrapped text inside nodes
-   paste screenshot from clipboard [native target only 🖥️] 
-   connect nodes with arrows
-   make app snapshot in memory and load from it (MacOs: Command + s\[l\])
-   save app state to db and load from it
-   change background color of nodes
-   move node to front/back
-   positioning text inside node
-   multiple documents/tabs support
-   load app state from url
-   ability to create sharable url of the document using \"Share
    Document\" button (**.velo.toml** should be created in user home
    directory containing GitHub access token with \"gist\" scope) [native target only 🖥️]:

   ```toml
   github_access_token = "<github_access_token>"
   ```

- initial markdown support
  - italic/bold text style
  - links
  - syntax highlighting
  - headings
  - inline code
  - ordered/unordered lists
- particles effect [native target only 🖥️]
- filter documents by text in nodes (fuzzy search) [native target only 🖥️]

## Run

Native:

```sh
cargo r 
```

Wasm:

```sh
cargo r --target wasm32-unknown-unknown
```

To create app bundle with icon (tested only on MacOS):

```sh
cargo install cargo-bundle
cargo bundle
```

## Pre-commit actions

```sh
cargo fmt
cargo clippy -- -A clippy::type_complexity -A clippy::too_many_arguments
```

## Basic usage

- click on rectangle icon to create rectangle node
- double-click to select node
- start typing to add text to selected node
- resize node by dragging its corners
- click on canvas to deselect node
- move node by dragging it (only unselected node can be dragged to allow mouse text selection for selected nodes)
- click on arrow connection icon to connect nodes, arrow connection nodes are placed on each side of node
- for native target there is search box that allows to filter documents by text in nodes (fuzzy search)
- for wasm target you can use url query parameter `?document=<url>` to load document from url
- click save icon to save document to database on native platform or to localhost on wasm target

## License
All code in this repository dual-licensed under either:

MIT License or http://opensource.org/licenses/MIT
Apache License, Version 2.0 or http://www.apache.org/licenses/LICENSE-2.0
Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

## Contributing
Contributions are always welcome! Please adhere to this project\'s code
of conduct. If you have questions or suggestions feel free to share on [velo discord server](https://discord.gg/PqChPzWV).

❤️
