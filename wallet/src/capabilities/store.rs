//! # Verifiable Credential Store
use std::fmt::Debug;

use crux_core::capability::{CapabilityContext, Operation};
use crux_core::Capability;
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Errors that can be returned by the Store capability.
#[derive(Clone, Debug, Deserialize, Serialize, Error, PartialEq, Eq)]
pub enum StoreError {
    /// Invalid request.
    #[error("invalid store request {message}")]
    InvalidRequest { message: String },

    /// The response from the shell capability was invalid.
    #[error("invalid store response {message}")]
    InvalidResponse { message: String },
}

/// An entry in the data store; a serialized credential or flow state.
/// 
/// `StoreEntry::None` is used to represent a missing entry in the store rather
/// than using an `Option` which is not supported across the FFI boundary in
/// Crux.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum StoreEntry {
    /// No entry for a given ID.
    None,

    /// A serialized credential or flow state.
    Data(#[serde(with = "serde_bytes")] Vec<u8>),
}

impl From<Vec<u8>> for StoreEntry {
    fn from(bytes: Vec<u8>) -> Self {
        StoreEntry::Data(bytes)
    }
}

impl From<StoreEntry> for Option<Vec<u8>> {
    fn from(entry: StoreEntry) -> Option<Vec<u8>> {
        match entry {
            StoreEntry::None => None,
            StoreEntry::Data(bytes) => Some(bytes),
        }
    }
}

impl From<Option<Vec<u8>>> for StoreEntry {
    fn from(val: Option<Vec<u8>>) -> Self {
        match val {
            None => StoreEntry::None,
            Some(bytes) => StoreEntry::Data(bytes),
        }
    }
}

/// Supported operations for the Store capability.
#[derive(Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum StoreOperation {
    /// Save a serialized object to the store. Insert a new entry if the ID does
    /// not exist, otherwise update the existing entry.
    Save {
        catalog: String,
        id: String,
        #[serde(with = "serde_bytes")]
        data: Vec<u8>
    },

    /// Get all serialized objects from the store.
    List { catalog: String },

    /// Remove the credential with the given ID from the store.
    Delete { catalog: String, id: String },
}

impl Debug for StoreOperation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StoreOperation::Save { catalog, id, data } => {
                let body = format!("<binary data - {} bytes>", data.len());
                f.debug_struct("Save")
                    .field("catalog", catalog)
                    .field("id", id)
                    .field("data", &body)
                    .finish()
            }
            StoreOperation::List { catalog } => {
                f.debug_struct("List").field("catalog", catalog).finish()
            }
            StoreOperation::Delete { catalog, id } => {
                f.debug_struct("Delete").field("catalog", catalog).field("id", id).finish()
            }
        }
    }
}

/// The result of an operation on the store.
///
/// Note: we cannot use Rust's `Result` and `Option` here because generics are
/// not supported across the FFI boundary in Crux.
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum StoreResult {
    Ok { response: StoreResponse },
    Err { error: StoreError },
}

impl StoreResult {
    fn unwrap_save(self) -> Result<(), StoreError> {
        match self {
            StoreResult::Ok { response } => match response {
                StoreResponse::Saved => Ok(()),
                _ => {
                    panic!("unexpected response for Save operation: {:?}", response);
                }
            },
            StoreResult::Err { error } => Err(error.clone()),
        }
    }

    fn unwrap_list(self) -> Result<Vec<StoreEntry>, StoreError> {
        match self {
            StoreResult::Ok { response } => match response {
                StoreResponse::List { entries } => Ok(entries),
                _ => {
                    panic!("unexpected response for List operation: {:?}", response);
                }
            },
            StoreResult::Err { error } => Err(error.clone()),
        }
    }

    fn unwrap_delete(self) -> Result<(), StoreError> {
        match self {
            StoreResult::Ok { response } => match response {
                StoreResponse::Deleted => Ok(()),
                _ => {
                    panic!("unexpected response for Delete operation: {:?}", response);
                }
            },
            StoreResult::Err { error } => Err(error.clone()),
        }
    }
}

/// The possible responses from the Store capability.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum StoreResponse {
    /// The result of a successful save operation.
    Saved,

    /// The result of a successful list operation.
    List { entries: Vec<StoreEntry> },

    /// The result of a successful delete operation.
    Deleted,
}

impl Operation for StoreOperation {
    type Output = StoreResult;
}

/// The type used to implement the capability.
pub struct Store<Ev> {
    context: CapabilityContext<StoreOperation, Ev>,
}

impl<Ev> Capability<Ev> for Store<Ev> {
    type MappedSelf<MappedEv> = Store<MappedEv>;
    type Operation = StoreOperation;

    fn map_event<F, NewEv>(&self, f: F) -> Self::MappedSelf<NewEv>
    where
        F: Fn(NewEv) -> Ev + Send + Sync + 'static,
        Ev: 'static,
        NewEv: 'static + Send,
    {
        Store::new(self.context.map_event(f))
    }

    #[cfg(feature = "typegen")]
    fn register_types(generator: &mut crux_core::typegen::TypeGen) -> crux_core::typegen::Result {
        generator.register_type::<StoreResponse>()?;
        generator.register_type::<StoreError>()?;
        generator.register_type::<StoreEntry>()?;
        generator.register_type::<Self::Operation>()?;
        generator.register_type::<<Self::Operation as Operation>::Output>()?;
        Ok(())
    }
}

impl<Ev> Clone for Store<Ev> {
    fn clone(&self) -> Self {
        Self {
            context: self.context.clone(),
        }
    }
}

impl<Ev> Store<Ev>
where
    Ev: 'static,
{
    /// Create a new instance of the Store capability.
    pub fn new(context: CapabilityContext<StoreOperation, Ev>) -> Self {
        Self { context }
    }

    /// Save a serialized credential to the store.
    pub fn save<F>(&self, catalog: String, id: String, data: Vec<u8>, make_event: F)
    where
        F: FnOnce(Result<(), StoreError>) -> Ev + Send + Sync + 'static,
    {
        let context = self.context.clone();
        self.context.spawn({
            async move {
                let response = save(&context, catalog, id, data).await;
                context.update_app(make_event(response))
            }
        });
    }

    /// Save a serialized credential to the store while in an async context.
    ///
    /// This can be used in a higher-order capability such as a `Provider` for
    /// `vercre-holder`.
    pub async fn save_async(&self, catalog: String, id: String, data: Vec<u8>) -> Result<(), StoreError> {
        save(&self.context, catalog, id, data).await
    }

    /// Get all serialized credentials from the store.
    pub fn list<F>(&self, catalog: String, make_event: F)
    where
        F: FnOnce(Result<Vec<StoreEntry>, StoreError>) -> Ev + Send + Sync + 'static,
    {
        self.context.spawn({
            let context = self.context.clone();
            async move {
                let response = list(&context, catalog).await;
                context.update_app(make_event(response))
            }
        });
    }

    /// Get all serialized credentials from the store while in an async context.
    ///
    /// This can be used in a higher-order capability such as a `Provider` for
    /// `vercre-holder`.
    pub async fn list_async(&self, catalog: String) -> Result<Vec<StoreEntry>, StoreError> {
        list(&self.context, catalog).await
    }

    /// Remove the credential with the given ID from the store.
    pub fn delete<F>(&self, catalog: String, id: String, make_event: F)
    where
        F: FnOnce(Result<(), StoreError>) -> Ev + Send + Sync + 'static,
    {
        self.context.spawn({
            let context = self.context.clone();
            async move {
                let response = delete(&context, catalog, id).await;
                context.update_app(make_event(response))
            }
        });
    }

    /// Remove the credential with the given ID from the store while in an async
    /// context.
    ///
    /// This can be used in a higher-order capability such as a `Provider` for
    /// `vercre-holder`.
    pub async fn delete_async(&self, catalog: String, id: String) -> Result<(), StoreError> {
        delete(&self.context, catalog, id).await
    }
}

async fn save<Ev: 'static>(
    context: &CapabilityContext<StoreOperation, Ev>, catalog: String, id: String, data: Vec<u8>,
) -> Result<(), StoreError> {
    context.request_from_shell(StoreOperation::Save { catalog, id, data }).await.unwrap_save()
}

async fn list<Ev: 'static>(
    context: &CapabilityContext<StoreOperation, Ev>, catalog: String,
) -> Result<Vec<StoreEntry>, StoreError> {
    context.request_from_shell(StoreOperation::List{ catalog }).await.unwrap_list()
}

async fn delete<Ev: 'static>(
    context: &CapabilityContext<StoreOperation, Ev>, catalog: String, id: String,
) -> Result<(), StoreError> {
    context.request_from_shell(StoreOperation::Delete { catalog, id }).await.unwrap_delete()
}
