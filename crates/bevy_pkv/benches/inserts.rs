use bevy_pkv::PkvStore;

fn insert_values(store: &mut PkvStore, pairs: &Vec<(String, String)>) {
    for (key, value) in pairs {
        store.set::<String>(key, value).unwrap();
    }
}

#[cfg(not(target_arch = "wasm32"))]
mod native {
    use super::insert_values;
    use bevy_pkv::PkvStore;
    use criterion::{criterion_group, BatchSize, Criterion};

    fn insert_benchmark(c: &mut Criterion) {
        c.bench_function("insert 100", |b| {
            b.iter_batched(
                || {
                    let store = PkvStore::new("BevyPkv", "InsertBench");
                    let values = (0..100).map(|i| (i.to_string(), i.to_string())).collect();
                    // todo: clear the store here
                    (store, values)
                },
                |(mut store, pairs)| insert_values(&mut store, &pairs),
                BatchSize::PerIteration,
            )
        });
    }

    criterion_group!(benches, insert_benchmark);
}

#[cfg(target_arch = "wasm32")]
mod wasm {
    use super::insert_values;
    use bevy_pkv::PkvStore;
    use easybench_wasm::bench_env;
    use web_sys::console;

    // Easy bench clones the environment, but store doesn't implement clone
    struct HackStore(PkvStore);

    impl Clone for HackStore {
        fn clone(&self) -> Self {
            let store = PkvStore::new("BevyPkv", "InsertBench");
            // todo: clear store as well
            Self(store)
        }
    }

    pub fn main() {
        console::log_1(&"Hi".to_string().into());
        {
            let values: Vec<(String, String)> =
                (0..100).map(|i| (i.to_string(), i.to_string())).collect();
            let env = (HackStore(PkvStore::new("BevyPkv", "InsertBench")), values);
            console::log_1(
                &format!(
                    "insert 100: {}",
                    bench_env(env, |(store, values)| insert_values(&mut store.0, values)),
                )
                .into(),
            );
        }
        {
            let values: Vec<(String, String)> =
                (0..1000).map(|i| (i.to_string(), i.to_string())).collect();
            let env = (HackStore(PkvStore::new("BevyPkv", "InsertBench")), values);
            console::log_1(
                &format!(
                    "insert 1000: {}",
                    bench_env(env, |(store, values)| insert_values(&mut store.0, values)),
                )
                .into(),
            );
        }
    }
}

#[cfg(target_arch = "wasm32")]
fn main() {
    wasm::main()
}

#[cfg(not(target_arch = "wasm32"))]
criterion::criterion_main!(native::benches);
