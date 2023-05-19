use crate::{Location, StoreImpl};
use serde::{de::DeserializeOwned, Serialize};

#[derive(Debug)]
pub struct RocksDBStore {
    db: rocksdb::DB,
}

pub use RocksDBStore as InnerStore;

/// Errors that can occur during `PkvStore::get`
#[derive(thiserror::Error, Debug)]
pub enum GetError {
    /// An internal error from the rocksdb crate
    #[error("Rocksdb error")]
    Rocksdb(#[from] rocksdb::Error),
    /// Error when deserializing the value
    #[error("MessagePack deserialization error")]
    MessagePack(#[from] rmp_serde::decode::Error),
    /// The value for the given key was not found
    #[error("No value found for the given key")]
    NotFound,
}

/// Errors that can occur during `PkvStore::set`
#[derive(thiserror::Error, Debug)]
pub enum SetError {
    /// An internal error from the rocksdb crate
    #[error("Rocksdb error")]
    Rocksdb(#[from] rocksdb::Error),
    /// Error when serializing the value
    #[error("MessagePack serialization error")]
    MessagePack(#[from] rmp_serde::encode::Error),
}

impl RocksDBStore {
    pub(crate) fn new(location: Location) -> Self {
        let mut options = rocksdb::Options::default();
        options.set_error_if_exists(false);
        options.create_if_missing(true);
        options.create_missing_column_families(true);

        let db_path = location.get_path().join("bevy_rocksdb_pkv");
        let db = rocksdb::DB::open(&options, db_path).expect("Failed to init key value store");
        Self { db }
    }
}

impl StoreImpl for RocksDBStore {
    type GetError = GetError;
    type SetError = SetError;

    /// Serialize and store the value
    fn set<T: Serialize>(&mut self, key: &str, value: &T) -> Result<(), Self::SetError> {
        let mut serializer = rmp_serde::Serializer::new(Vec::new()).with_struct_map();
        value.serialize(&mut serializer)?;
        self.db.put(key, serializer.into_inner())?;

        Ok(())
    }

    /// More or less the same as set::<String>, but can take a &str
    fn set_string(&mut self, key: &str, value: &str) -> Result<(), Self::SetError> {
        let bytes = rmp_serde::to_vec(value)?;
        self.db.put(key, bytes)?;

        Ok(())
    }

    /// Get the value for the given key
    /// returns Err(GetError::NotFound) if the key does not exist in the key value store.
    fn get<T: DeserializeOwned>(&self, key: &str) -> Result<T, Self::GetError> {
        let bytes = self.db.get(key)?.ok_or(Self::GetError::NotFound)?;
        let value = rmp_serde::from_slice(&bytes)?;
        Ok(value)
    }

    /// Clear all keys and their values
    /// The RocksDB adapter uses an iterator to achieve this, unlike sled
    fn clear(&mut self) -> Result<(), Self::SetError> {
        let kv_iter = self.db.iterator(rocksdb::IteratorMode::Start);

        for kv in kv_iter {
            let (key, _) = kv?;
            self.db.delete(key)?;
        }

        Ok(())
    }
}
