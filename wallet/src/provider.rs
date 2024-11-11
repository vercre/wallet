//! Provider of callbacks for `vercre-holder`.
//!
//! Implementation of the `vercre-holder` `Provider` traits. Uses capabilities
//! where necessary to provide the underlying connectivity and storage.

use chrono::{DateTime, Utc};
use serde::de::DeserializeOwned;
use serde::Serialize;
use vercre_holder::credential::{Credential, ImageData};
use vercre_holder::provider::{
    Algorithm, CredentialStorer, DidResolver, Document, HolderProvider, Issuer, Signer, StateStore,
    Verifier,
};
use vercre_holder::{
    AuthorizationRequest, AuthorizationResponse, Constraints, CredentialRequest,
    CredentialResponse, DeferredCredentialRequest, DeferredCredentialResponse, MetadataRequest,
    MetadataResponse, NotificationRequest, NotificationResponse, OAuthServerRequest,
    OAuthServerResponse, RequestObjectResponse, ResponseRequest, ResponseResponse, TokenRequest,
    TokenResponse,
};

use crate::capabilities::store::{Catalog, Store, StoreEntry};

pub struct Provider<Ev> {
    store: Store<Ev>,
}

impl<Ev> Clone for Provider<Ev> {
    fn clone(&self) -> Self {
        Self {
            store: self.store.clone(),
        }
    }
}

impl<Ev> Provider<Ev> {
    pub fn new(store: Store<Ev>) -> Self {
        Self { store }
    }
}

impl<Ev> HolderProvider for Provider<Ev> where Ev: 'static {}

impl<Ev> Issuer for Provider<Ev> {
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

impl<Ev> Verifier for Provider<Ev> {
    /// Get a request object. If an error is returned, the wallet will cancel
    /// the presentation flow.
    async fn request_object(&self, _req: &str) -> anyhow::Result<RequestObjectResponse> {
        todo!()
    }

    /// Send the presentation to the verifier.
    async fn present(
        &self, _uri: Option<&str>, _presentation: &ResponseRequest,
    ) -> anyhow::Result<ResponseResponse> {
        todo!()
    }
}

impl<Ev> CredentialStorer for Provider<Ev>
where
    Ev: 'static,
{
    /// Save a `Credential` to the store. Overwrite any existing credential with
    /// the same ID. Create a new credential if one with the same ID does
    /// not exist.
    async fn save(&self, credential: &Credential) -> anyhow::Result<()> {
        let data = serde_json::to_vec(credential)?;
        let id = credential.id.clone();
        self.store.save_async(Catalog::Credential.to_string(), id, data).await.map_err(Into::into)
    }

    /// Retrieve a `Credential` from the store with the given ID. Return None if
    /// no credential with the ID exists.
    async fn load(&self, id: &str) -> anyhow::Result<Option<Credential>> {
        let all_data = self.store.list_async(Catalog::Credential.to_string()).await?;
        for entry in all_data {
            if let StoreEntry::Data(data) = entry {
                let credential: Credential = serde_json::from_slice(&data)?;
                if credential.id == id {
                    return Ok(Some(credential));
                }
            };
        }
        Ok(None)
    }

    /// Find the credentials that match the the provided filter. If `filter` is
    /// None, return all credentials in the store.
    async fn find(&self, filter: Option<Constraints>) -> anyhow::Result<Vec<Credential>> {
        let all_data = self.store.list_async(Catalog::Credential.to_string()).await?;
        let mut credentials = Vec::new();
        for entry in all_data {
            if let StoreEntry::Data(data) = entry {
                let credential: Credential = serde_json::from_slice(&data)?;
                if let Some(filter) = &filter {
                    match filter.satisfied(&credential) {
                        Ok(true) => credentials.push(credential),
                        Ok(false) => {}
                        Err(e) => return Err(e.into()),
                    }
                } else {
                    credentials.push(credential);
                }
            }
        }
        Ok(credentials)
    }

    /// Remove the credential with the given ID from the store. Return an error
    /// if the credential does not exist.
    async fn remove(&self, id: &str) -> anyhow::Result<()> {
        self.store.delete_async(Catalog::Credential.to_string(), id).await.map_err(Into::into)
    }
}

impl<Ev> StateStore for Provider<Ev> {
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

impl<Ev> Signer for Provider<Ev> {
    /// Sign is a convenience method for infallible Signer implementations.
    async fn sign(&self, msg: &[u8]) -> Vec<u8> {
        self.try_sign(msg).await.expect("should sign")
    }

    /// `TrySign` is the fallible version of Sign.
    async fn try_sign(&self, _msg: &[u8]) -> anyhow::Result<Vec<u8>> {
        todo!()
    }

    /// The public key of the key pair used in signing. The possibility of key
    /// rotation mean this key should only be referenced at the point of
    /// signing.
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

impl<Ev> DidResolver for Provider<Ev> {
    /// Resolve the DID URL to a DID Document.
    ///
    /// # Errors
    ///
    /// Returns an error if the DID URL cannot be resolved.
    async fn resolve(&self, _url: &str) -> anyhow::Result<Document> {
        todo!()
    }
}
