use crate::Value;
use anyhow::{ensure, Context, Result};
use bincode::{deserialize_from, serialize_into};
use log::info;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs::File,
    path::{Path, PathBuf},
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum StorageError {
    #[error("Storage file is open")]
    StorageIsOpen,

    #[error("Storage file is not open")]
    StorageIsClosed,

    #[error("Storage has no key {0}")]
    StorageMissingKey(String),

    #[error("Could not deserialize Storage")]
    Deserialize,

    #[error("Could not Serialize Storage")]
    Serialize,
}

#[derive(Debug)]
pub struct Storage {
    name: String,
    data: HashMap<String, Value>,
}

impl Storage {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_owned(),
            data: HashMap::new(),
        }
    }
}

#[derive(Debug)]
pub struct StoragePool {
    path: PathBuf,
    files: HashMap<String, Storage>,
}

impl StoragePool {
    pub fn new<P>(path: P) -> Self
    where
        P: AsRef<Path>,
    {
        Self {
            path: path.as_ref().into(),
            files: HashMap::new(),
        }
    }

    pub fn open(&mut self, name: &str) -> Result<()> {
        ensure!(!self.files.contains_key(name), StorageError::StorageIsOpen);

        let storage = Storage::new(name);

        self.files.insert(name.to_owned(), storage);

        let storage_path = self.path.join(name);
        info!("Opened storage at {}", storage_path.display());

        Ok(())
    }

    pub fn close(&mut self, name: &str) -> Result<()> {
        ensure!(self.files.contains_key(name), StorageError::StorageIsClosed);

        self.files.remove(name);

        let storage_path = self.path.join(name);
        info!("Closed storage at {}", storage_path.display());

        Ok(())
    }

    pub fn read(&mut self, name: &str) -> Result<()> {
        ensure!(self.files.contains_key(name), StorageError::StorageIsClosed);

        let storage_path = self.path.join(name);
        let file = File::open(&storage_path)?;
        let data = deserialize_from(file).context(StorageError::Deserialize)?;

        self.files
            .get_mut(name)
            .context(StorageError::StorageIsClosed)?
            .data = data;

        info!("Read storage at {}", storage_path.display());

        Ok(())
    }

    pub fn write(&self, name: &str) -> Result<()> {
        ensure!(self.files.contains_key(name), StorageError::StorageIsClosed);

        let storage_path = self.path.join(name);
        let file = File::create(&storage_path)?;
        let data = &self
            .files
            .get(name)
            .context(StorageError::StorageIsClosed)?
            .data;

        serialize_into(file, data).context(StorageError::Serialize)?;

        info!("Wrote storage at {}", storage_path.display());

        Ok(())
    }

    pub fn get(&self, name: &str, key: &str) -> Result<&Value> {
        ensure!(self.files.contains_key(name), StorageError::StorageIsClosed);

        let data = &self.files.get(name).unwrap().data;

        if let Some(value) = data.get(key) {
            info!("Read storage {} key {}", name, key);

            Ok(value)
        } else {
            Err(StorageError::StorageMissingKey(key.to_owned()))?
        }
    }

    pub fn set(&mut self, name: &str, key: &str, value: &Value) -> Result<()> {
        todo!()
    }

    pub fn erase(&mut self, name: &str, key: &str) -> Result<()> {
        todo!()
    }

    pub fn exists(&self, name: &str, key: &str) -> Result<bool> {
        todo!()
    }

    pub fn get_files(&self) -> Vec<&str> {
        self.files.keys().map(|key| key.as_str()).collect()
    }
}
