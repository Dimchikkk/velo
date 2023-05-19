# bevy_pkv

[![crates.io](https://img.shields.io/crates/v/bevy_pkv.svg)](https://crates.io/crates/bevy_pkv)
![MIT/Apache 2.0](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)
[![docs.rs](https://img.shields.io/docsrs/bevy_pkv)](https://docs.rs/bevy_pkv)
[![ci](https://github.com/johanhelsing/bevy_pkv/actions/workflows/ci.yml/badge.svg)](https://github.com/johanhelsing/bevy_pkv/actions/workflows/ci.yml)

`bevy_pkv` is a cross-platform persistent key value store for rust apps.

Use it for storing things like settings, save games etc.

Currently, the Bevy dependency is optional, so it may be used in other games/apps as well.

## Usage with Bevy

Add a store resource to your app

```rust no_run
# #[cfg(feature = "bevy")] { // ignore this line
use bevy::prelude::*;
use bevy_pkv::PkvStore;

fn main() {
App::new()
    .add_plugins(DefaultPlugins)
    .insert_resource(PkvStore::new("FooCompany", "BarGame"))
    // ...insert systems etc.
    .run();
}
# }
```

This will create or load a store in the appropriate location for your system, and make it available to bevy systems:

```rust ignore
fn setup(mut pkv: ResMut<PkvStore>) {
    if let Ok(username) = pkv.get::<String>("username") {
        info!("Welcome back {username}");
    } else {
        pkv.set_string("username", "alice")
            .expect("failed to store username");

        // alternatively, using the slightly less efficient generic api:
        pkv.set("username", &"alice".to_string())
            .expect("failed to store username");
    }
}
```

Using your own types implementing `serde::Serialize` and `Deserialize`:

```rust ignore
#[derive(Serialize, Deserialize)]
struct User {
    name: String,
}

fn setup(mut pkv: ResMut<PkvStore>) {
    if let Ok(user) = pkv.get::<User>("user") {
        info!("Welcome back {}", user.name);
    } else {
        let user = User {
            name: "bob".to_string(),
        };
        pkv.set("user", &user).expect("failed to store user");
    }
}
```

See the [examples](./examples) for further usage

## Usage without Bevy

Disable the default features when adding the dependency:

```toml
bevy_pkv = {version = 0.7, default-features = false}
```

## Implementation details

### Native

`sled` and `rmp_serde` (MessagePack) is used for storage. It's creating a sled db in `bevy_pkv` in the appropriate application data directory for your system.

Alternatively, disable default-features and enable the `rocksdb` feature to use a RocksDB-based implementation.

### Wasm

`Window.localStorage` and `serde_json` is used for storage. Perhaps IndexedDb and something else would have been a better choice, but its API is complicated, and I wanted a simple implementation and a simple synchronous API.

## Bevy version support

The `main` branch targets the latest bevy release.

I intend to support the `main` branch of Bevy in the `bevy-main` branch.

|bevy|bevy\_pkv|
|----|---|
|0.10|0.7, main|
|0.9 |0.6|
|0.8 |0.5|
|0.7 |0.2, 0.3, 0.4|
|0.6 |0.1|

## License

MIT or Apache-2.0
