//! # Key Store Capability
use std::fmt::Debug;

use crux_core::capability::{CapabilityContext, Operation};
use crux_core::Capability;
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Errors that can be returned by the key store capability.
#[derive(Clone, Debug, Deserialize, Serialize, Error, PartialEq, Eq)]
pub enum KeyStoreError {
    /// Invalid request.
    #[error("invalid key store request {message}")]
    InvalidRequest { message: String },

    /// The response from the shell capability was invalid.
    #[error("invalid key store response {message}")]
    InvalidResponse { message: String },
}

/// An entry in the key store.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum KeyStoreEntry {
    /// No entry for the given ID and purpose.
    None,

    /// A serialized private key.
    Data(#[serde(with = "serde_bytes")] Vec<u8>),
}

impl From<Vec<u8>> for KeyStoreEntry {
    fn from(bytes: Vec<u8>) -> Self {
        KeyStoreEntry::Data(bytes)
    }
}

impl From<KeyStoreEntry> for Option<Vec<u8>> {
    fn from(entry: KeyStoreEntry) -> Option<Vec<u8>> {
        match entry {
            KeyStoreEntry::None => None,
            KeyStoreEntry::Data(bytes) => Some(bytes),
        }
    }
}

impl From<Option<Vec<u8>>> for KeyStoreEntry {
    fn from(val: Option<Vec<u8>>) -> Self {
        match val {
            None => KeyStoreEntry::None,
            Some(bytes) => KeyStoreEntry::Data(bytes),
        }
    }
}

/// Supported operations for the key store capability.
#[derive(Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum KeyStoreOperation {
    /// Get a serialized private key from the key store.
    Get { id: String, purpose: String },

    /// Set a serialized private key in the key store.
    Set {
        id: String,
        purpose: String,
        #[serde(with = "serde_bytes")]
        data: Vec<u8>,
    },

    /// Remove a serialized private key from the key store.
    Delete { id: String, purpose: String },

    /// Generate a random secret suitable for key derivation.
    GenerateSecret { length: usize },
}

impl Debug for KeyStoreOperation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            KeyStoreOperation::Get { id, purpose } => {
                f.debug_struct("Get").field("id", id).field("purpose", purpose).finish()
            }
            KeyStoreOperation::Set { id, purpose, data } => {
                let body = format!("<binary data - {} bytes>", data.len());
                f.debug_struct("Set")
                    .field("id", id)
                    .field("purpose", purpose)
                    .field("data", &body)
                    .finish()
            }
            KeyStoreOperation::Delete { id, purpose } => {
                f.debug_struct("Delete").field("id", id).field("purpose", purpose).finish()
            }
            KeyStoreOperation::GenerateSecret { length } => {
                f.debug_struct("GenerateSecret").field("length", length).finish()
            }
        }
    }
}

/// The possible responses from the key store capability.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum KeyStoreResponse {
    /// The result of a get operation.
    Retrieved { key: KeyStoreEntry },

    /// The result of a set operation.
    Set,

    /// result of a delete operation.
    Deleted,

    /// A random secret suitable for key derivation.
    GeneratedSecret { secret: Vec<u8> },
}

/// The result of an operation on the key store.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum KeyStoreResult {
    /// The operation was successful.
    Ok { response: KeyStoreResponse },

    /// The operation failed.
    Err { error: KeyStoreError },
}

impl KeyStoreResult {
    fn unwrap_get(self) -> Result<KeyStoreEntry, KeyStoreError> {
        match self {
            KeyStoreResult::Ok {
                response: KeyStoreResponse::Retrieved { key },
            } => Ok(key),
            KeyStoreResult::Err { error } => Err(error),
            _ => Err(KeyStoreError::InvalidResponse {
                message: "unexpected response for Get operation".to_string(),
            }),
        }
    }

    fn unwrap_set(self) -> Result<(), KeyStoreError> {
        match self {
            KeyStoreResult::Ok {
                response: KeyStoreResponse::Set,
            } => Ok(()),
            KeyStoreResult::Err { error } => Err(error),
            _ => Err(KeyStoreError::InvalidResponse {
                message: "unexpected response for Set operation".to_string(),
            }),
        }
    }

    fn unwrap_delete(self) -> Result<(), KeyStoreError> {
        match self {
            KeyStoreResult::Ok {
                response: KeyStoreResponse::Deleted,
            } => Ok(()),
            KeyStoreResult::Err { error } => Err(error),
            _ => Err(KeyStoreError::InvalidResponse {
                message: "unexpected response for Delete operation".to_string(),
            }),
        }
    }

    fn unwrap_generate_secret(self) -> Result<Vec<u8>, KeyStoreError> {
        match self {
            KeyStoreResult::Ok {
                response: KeyStoreResponse::GeneratedSecret { secret },
            } => Ok(secret),
            KeyStoreResult::Err { error } => Err(error),
            _ => Err(KeyStoreError::InvalidResponse {
                message: "unexpected response for GenerateSecret operation".to_string(),
            }),
        }
    }
}

impl Operation for KeyStoreOperation {
    type Output = KeyStoreResult;
}

/// Capability type for the key store.
pub struct KeyStore<Ev> {
    context: CapabilityContext<KeyStoreOperation, Ev>,
}

impl<Ev> Capability<Ev> for KeyStore<Ev> {
    type MappedSelf<MappedEv> = KeyStore<MappedEv>;
    type Operation = KeyStoreOperation;

    fn map_event<F, NewEv>(&self, f: F) -> Self::MappedSelf<NewEv>
    where
        F: Fn(NewEv) -> Ev + Send + Sync + 'static,
        Ev: 'static,
        NewEv: 'static + Send,
    {
        KeyStore::new(self.context.map_event(f))
    }

    #[cfg(feature = "typegen")]
    fn register_types(generator: &mut crux_core::typegen::TypeGen) -> crux_core::typegen::Result {
        generator.register_type::<KeyStoreResponse>()?;
        generator.register_type::<KeyStoreError>()?;
        generator.register_type::<KeyStoreEntry>()?;
        generator.register_type::<Self::Operation>()?;
        generator.register_type::<<Self::Operation as Operation>::Output>()?;
        Ok(())
    }
}

impl<Ev> Clone for KeyStore<Ev> {
    fn clone(&self) -> Self {
        Self {
            context: self.context.clone(),
        }
    }
}

impl<Ev> KeyStore<Ev>
where
    Ev: 'static,
{
    /// Create a new key store capability.
    pub fn new(context: CapabilityContext<KeyStoreOperation, Ev>) -> Self {
        Self { context }
    }

    /// Get a serialized private key from the key store and send an update event
    /// to the application.
    pub fn get<F>(
        &self, id: impl Into<String> + Send + 'static, purpose: impl Into<String> + Send + 'static,
        make_event: F,
    ) where
        F: FnOnce(Result<KeyStoreEntry, KeyStoreError>) -> Ev + Send + Sync + 'static,
    {
        self.context.spawn({
            let context = self.context.clone();
            async move {
                let response = get(&context, id, purpose).await;
                context.update_app(make_event(response))
            }
        });
    }

    /// Get a serialized private key from the key store.
    pub async fn get_async(
        &self, id: impl Into<String>, purpose: impl Into<String>,
    ) -> Result<KeyStoreEntry, KeyStoreError> {
        get(&self.context, id, purpose).await
    }

    /// Store a serialized private key in the key store and send an update event
    /// to the application.
    pub fn set<F>(
        &self, id: impl Into<String> + Send + 'static, purpose: impl Into<String> + Send + 'static,
        data: impl Serialize + Send + 'static, make_event: F,
    ) where
        F: FnOnce(Result<(), KeyStoreError>) -> Ev + Send + Sync + 'static,
    {
        self.context.spawn({
            let context = self.context.clone();
            async move {
                let response = set(&context, id, purpose, data).await;
                context.update_app(make_event(response))
            }
        });
    }

    /// Store a serialized private key in the key store.
    pub async fn set_async(
        &self, id: impl Into<String>, purpose: impl Into<String>, data: impl Serialize,
    ) -> Result<(), KeyStoreError> {
        set(&self.context, id, purpose, data).await
    }

    /// Remove a serialized private key from the key store and send an update
    /// event to the application.
    pub fn delete<F>(
        &self, id: impl Into<String> + Send + 'static, purpose: impl Into<String> + Send + 'static,
        make_event: F,
    ) where
        F: FnOnce(Result<(), KeyStoreError>) -> Ev + Send + Sync + 'static,
    {
        self.context.spawn({
            let context: CapabilityContext<KeyStoreOperation, Ev> = self.context.clone();
            async move {
                let response = delete(&context, id, purpose).await;
                context.update_app(make_event(response))
            }
        });
    }

    /// Remove a serialized private key from the key store.
    pub async fn delete_async(
        &self, id: impl Into<String>, purpose: impl Into<String>,
    ) -> Result<(), KeyStoreError> {
        delete(&self.context, id, purpose).await
    }

    /// Generate a random secret suitable for key derivation and send an update
    /// event to the application.
    pub fn generate_secret<F>(&self, length: usize, make_event: F)
    where
        F: FnOnce(Result<Vec<u8>, KeyStoreError>) -> Ev + Send + Sync + 'static,
    {
        self.context.spawn({
            let context: CapabilityContext<KeyStoreOperation, Ev> = self.context.clone();
            async move {
                let response = generate_secret(&context, length).await;
                context.update_app(make_event(response))
            }
        });
    }

    /// Generate a random secret suitable for key derivation.
    pub async fn generate_secret_async(&self, length: usize) -> Result<Vec<u8>, KeyStoreError> {
        generate_secret(&self.context, length).await
    }
}

async fn get<Ev: 'static>(
    context: &CapabilityContext<KeyStoreOperation, Ev>, id: impl Into<String>,
    purpose: impl Into<String>,
) -> Result<KeyStoreEntry, KeyStoreError> {
    context
        .request_from_shell(KeyStoreOperation::Get {
            id: id.into(),
            purpose: purpose.into(),
        })
        .await
        .unwrap_get()
}

async fn set<Ev: 'static>(
    context: &CapabilityContext<KeyStoreOperation, Ev>, id: impl Into<String>,
    purpose: impl Into<String>, data: impl Serialize,
) -> Result<(), KeyStoreError> {
    let bytes = serde_json::to_vec(&data).map_err(|e| KeyStoreError::InvalidRequest {
        message: format!("failed to serialize key: {}", e),
    })?;
    context
        .request_from_shell(KeyStoreOperation::Set {
            id: id.into(),
            purpose: purpose.into(),
            data: bytes,
        })
        .await
        .unwrap_set()
}

async fn delete<Ev: 'static>(
    context: &CapabilityContext<KeyStoreOperation, Ev>, id: impl Into<String>,
    purpose: impl Into<String>,
) -> Result<(), KeyStoreError> {
    context
        .request_from_shell(KeyStoreOperation::Delete {
            id: id.into(),
            purpose: purpose.into(),
        })
        .await
        .unwrap_delete()
}

async fn generate_secret<Ev: 'static>(
    context: &CapabilityContext<KeyStoreOperation, Ev>, length: usize,
) -> Result<Vec<u8>, KeyStoreError> {
    context
        .request_from_shell(KeyStoreOperation::GenerateSecret { length })
        .await
        .unwrap_generate_secret()
}