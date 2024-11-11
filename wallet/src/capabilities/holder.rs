//! Holder capability.
//!
//! This capability gives all of the callbacks specified by the `vercre-holder`
//! `Provider` traits. It is essentially a wrapper to a set of lower-level
//! capabilities.

use std::fmt::Debug;

use chrono::{DateTime, Utc};
use crux_core::capability::{CapabilityContext, Operation};
use crux_core::Capability;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use vercre_holder::credential::{Credential, ImageData};
use vercre_holder::provider::{Algorithm, CredentialStorer, DidResolver, Document, HolderProvider, Issuer, Signer, StateStore, Verifier};
use vercre_holder::{
    AuthorizationRequest, AuthorizationResponse, Constraints, CredentialRequest, CredentialResponse, DeferredCredentialRequest, DeferredCredentialResponse, MetadataRequest, MetadataResponse, NotificationRequest, NotificationResponse, OAuthServerRequest, OAuthServerResponse, RequestObjectResponse, ResponseRequest, ResponseResponse, TokenRequest, TokenResponse
};

/// Error response.
#[derive(Clone, Debug, Deserialize, Serialize, Error, PartialEq, Eq)]
pub enum HolderError {
    /// Invalid request.
    #[error("invalid holder provider request {message}")]
    InvalidRequest { message: String },

    /// The response from the shell capability was invalid.
    #[error("invalid holder provider response {message}")]
    InvalidResponse { message: String },
}

/// Supported operatons for the Holder capability.
#[derive(Clone, Deserialize, Serialize, PartialEq, Eq)]
pub enum HolderOperation {}

impl Debug for HolderOperation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "HolderOperation")
    }
}

/// The result of an operation on the provider.
///
/// Note: we cannot use Rust's `Result` and `Option` here because generics are
/// not supported across the FFI boundary in Crux.
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum HolderResult {
    Ok { response: HolderResponse },
    Err { error: HolderError },
}

impl HolderResult {
    // TODO: unwrap HolderResult into standard Rust Result.
}

/// Possible responses from the Holder capability.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub enum HolderResponse {}

impl Operation for HolderOperation {
    type Output = HolderResult;
}

/// The type used to implement the capability and the provider.
pub struct Holder<Ev> {
    context: CapabilityContext<HolderOperation, Ev>,
}

impl<Ev> Capability<Ev> for Holder<Ev> {
    type MappedSelf<MappedEv> = Holder<MappedEv>;
    type Operation = HolderOperation;

    fn map_event<F, NewEv>(&self, f: F) -> Self::MappedSelf<NewEv>
    where
        F: Fn(NewEv) -> Ev + Send + Sync + 'static,
        Ev: 'static,
        NewEv: 'static + Send,
    {
        Holder::new(self.context.map_event(f))
    }

    #[cfg(feature = "typegen")]
    fn register_types(generator: &mut crux_core::typegen::TypeGen) -> crux_core::typegen::Result {
        generator.register_type::<HolderResponse>()?;
        generator.register_type::<HolderError>()?;
        generator.register_type::<Self::Operation>()?;
        generator.register_type::<<Self::Operation as Operation>::Output>()?;
        Ok(())
    }
}

impl<Ev> Clone for Holder<Ev> {
    fn clone(&self) -> Self {
        Self {
            context: self.context.clone(),
        }
    }
}

impl<Ev> Holder<Ev>
where
    Ev: 'static,
{
    /// Create a new instance of the Holder capability.
    pub fn new(context: CapabilityContext<HolderOperation, Ev>) -> Self {
        Self { context }
    }
}

impl<Ev> HolderProvider for Holder<Ev> {}

impl<Ev> Issuer for Holder<Ev> {
    /// Get issuer metadata from the issuer service endpoint.
    async fn metadata(&self, _req: MetadataRequest) -> anyhow::Result<MetadataResponse> {
        todo!()
    }

    /// Get OAuth authorization configuration from the issuer's service
    /// endpoint.
    async fn oauth_server(&self, _req: OAuthServerRequest) -> anyhow::Result<OAuthServerResponse> {
        todo!()
    }

    /// Get an authorization code.
    async fn authorization(
        &self, _req: AuthorizationRequest,
    ) -> anyhow::Result<AuthorizationResponse> {
        todo!()
    }

    /// Get an access token.
    async fn token(&self, _req: TokenRequest) -> anyhow::Result<TokenResponse> {
        todo!()
    }

    /// Get a credential.
    async fn credential(&self, _req: CredentialRequest) -> anyhow::Result<CredentialResponse> {
        todo!()
    }

    /// Get a deferred credential.
    async fn deferred(
        &self, _req: DeferredCredentialRequest,
    ) -> anyhow::Result<DeferredCredentialResponse> {
        todo!()
    }

    /// Get a base64 encoded form of the credential logo.
    async fn image(&self, _image_url: &str) -> anyhow::Result<ImageData> {
        todo!()
    }

    /// Notify the issuer of issuance progress.
    async fn notification(
        &self, _req: NotificationRequest,
    ) -> anyhow::Result<NotificationResponse> {
        todo!()
    }
}

impl<Ev> Verifier for Holder<Ev> {
    /// Get a request object. If an error is returned, the wallet will cancel
    /// the presentation flow.
    async fn request_object(
        &self, _req: &str,
    ) -> anyhow::Result<RequestObjectResponse> {
        todo!()
    }

    /// Send the presentation to the verifier.
    async fn present(
        &self, _uri: Option<&str>, _presentation: &ResponseRequest,
    ) -> anyhow::Result<ResponseResponse> {
        todo!()
    }
}

impl<Ev> CredentialStorer for Holder<Ev> {
    /// Save a `Credential` to the store. Overwrite any existing credential with
    /// the same ID. Create a new credential if one with the same ID does
    /// not exist.
    async fn save(&self, _credential: &Credential) -> anyhow::Result<()> {
        todo!()
    }

    /// Retrieve a `Credential` from the store with the given ID. Return None if
    /// no credential with the ID exists.
    async fn load(&self, _id: &str) -> anyhow::Result<Option<Credential>> {
        todo!()
    }

    /// Find the credentials that match the the provided filter. If `filter` is
    /// None, return all credentials in the store.
    async fn find(
        &self, _filter: Option<Constraints>,
    ) -> anyhow::Result<Vec<Credential>> {
        todo!()
    }

    /// Remove the credential with the given ID from the store. Return an error
    /// if the credential does not exist.
    async fn remove(&self, _id: &str) -> anyhow::Result<()> {
        todo!()
    }
}

impl<Ev> StateStore for Holder<Ev> {
    /// Store state using the provided key. The expiry parameter indicates
    /// when data can be expunged from the state store.
    async fn put(
        &self, _key: &str, _state: impl Serialize + Send, _expiry: DateTime<Utc>,
    ) -> anyhow::Result<()> {
        todo!()
    }

    /// Retrieve data using the provided key.
    async fn get<T: DeserializeOwned>(&self, _key: &str) -> anyhow::Result<T> {
        todo!()
    }

    /// Remove data using the key provided.
    async fn purge(&self, _key: &str) -> anyhow::Result<()> {
        todo!()
    }
}

impl<Ev> Signer for Holder<Ev> {
    /// Sign is a convenience method for infallible Signer implementations.
    async fn sign(&self, msg: &[u8]) -> Vec<u8> {
        self.try_sign(msg).await.expect("should sign")
    }

    /// `TrySign` is the fallible version of Sign.
    async fn try_sign(&self, _msg: &[u8]) -> anyhow::Result<Vec<u8>> {
        todo!()
    }

    /// The public key of the key pair used in signing. The possibility of key
    /// rotation mean this key should only be referenced at the point of signing.
    async fn public_key(&self) -> anyhow::Result<Vec<u8>> {
        todo!()
    }

    /// Signature algorithm used by the signer.
    fn algorithm(&self) -> Algorithm {
        todo!()
    }

    /// The verification method the verifier should use to verify the signer's
    /// signature. This is typically a DID URL + # + verification key ID.
    fn verification_method(&self) -> String {
        todo!()
    }
}

impl<Ev> DidResolver for Holder<Ev> {
    /// Resolve the DID URL to a DID Document.
    ///
    /// # Errors
    ///
    /// Returns an error if the DID URL cannot be resolved.
    async fn resolve(&self, _url: &str) -> anyhow::Result<Document> {
        todo!()
    }
}
