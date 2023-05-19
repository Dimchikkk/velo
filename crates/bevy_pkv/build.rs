use cfg_aliases::cfg_aliases;

fn main() {
    cfg_aliases! {
        wasm: { target_arch = "wasm32" },
        rocksdb_backend: { all(feature = "rocksdb", not(wasm)) },
        sled_backend: { all(feature = "sled", not(wasm)) }
    }
}
