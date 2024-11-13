//! Provider of callbacks for `vercre-holder`.
//!
//! Implementation of the `vercre-holder` `Provider` traits. Uses capabilities
//! where necessary to provide the underlying connectivity and storage.

use anyhow::anyhow;
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
    http: crux_http::Http<Ev>,
    kv: crux_kv::KeyValue<Ev>,
    store: Store<Ev>,
}

impl<Ev> Clone for Provider<Ev> {
    fn clone(&self) -> Self {
        Self {
            http: self.http.clone(),
            kv: self.kv.clone(),
            store: self.store.clone(),
        }
    }
}

impl<Ev> Provider<Ev> {
    pub fn new(http: crux_http::Http<Ev>, kv: crux_kv::KeyValue<Ev>, store: Store<Ev>) -> Self {
        Self { http, kv, store }
    }
}

impl<Ev> HolderProvider for Provider<Ev> where Ev: 'static {}

impl<Ev> Issuer for Provider<Ev>
where
    Ev: 'static,
{
    /// Get issuer metadata from the issuer service endpoint.
    async fn metadata(&self, req: MetadataRequest) -> anyhow::Result<MetadataResponse> {
        let url = format!("{}/.well-known/openid-credential-issuer", req.credential_issuer);
        let req_bytes = serde_json::to_vec(&req)?;
        let mut response = self.http.post(url).body_bytes(req_bytes).send_async().await?;
        let res_bytes = response.body_bytes().await?;
        let metadata: MetadataResponse = serde_json::from_slice(&res_bytes)?;
        Ok(metadata)
    }

    /// Get OAuth authorization configuration from the issuer's service
    /// endpoint.
    async fn oauth_server(&self, _req: OAuthServerRequest) -> anyhow::Result<OAuthServerResponse> {
        unimplemented!()
    }

    /// Get an authorization code.
    async fn authorization(
        &self, req: AuthorizationRequest,
    ) -> anyhow::Result<AuthorizationResponse> {
        match req {
            AuthorizationRequest::Uri(_) => {
                unimplemented!();
            }
            AuthorizationRequest::Object(request) => {
                let url = format!("{}/authorize", request.credential_issuer);
                let mut response = self.http.get(url).query(&request)?.send_async().await?;
                let res_bytes = response.body_bytes().await?;
                let authorization: AuthorizationResponse = serde_json::from_slice(&res_bytes)?;
                Ok(authorization)
            }
        }
    }

    /// Get an access token.
    async fn token(&self, req: TokenRequest) -> anyhow::Result<TokenResponse> {
        let url = format!("{}/token", req.credential_issuer);
        let req_bytes = serde_json::to_vec(&req)?;
        let mut response = self.http.post(url).body_bytes(req_bytes).send_async().await?;
        let res_bytes = response.body_bytes().await?;
        let token: TokenResponse = serde_json::from_slice(&res_bytes)?;
        Ok(token)
    }

    /// Get a credential.
    async fn credential(&self, req: CredentialRequest) -> anyhow::Result<CredentialResponse> {
        let url = format!("{}/credential", req.credential_issuer);
        let req_bytes = serde_json::to_vec(&req)?;
        let mut response = self.http.post(url).body_bytes(req_bytes).send_async().await?;
        let res_bytes = response.body_bytes().await?;
        let credential: CredentialResponse = serde_json::from_slice(&res_bytes)?;
        Ok(credential)
    }

    /// Get a deferred credential.
    async fn deferred(
        &self, req: DeferredCredentialRequest,
    ) -> anyhow::Result<DeferredCredentialResponse> {
        let url = format!("{}/deferred_credential", req.credential_issuer);
        let req_bytes = serde_json::to_vec(&req)?;
        let mut response = self.http.post(url).body_bytes(req_bytes).send_async().await?;
        let res_bytes = response.body_bytes().await?;
        let deferred: DeferredCredentialResponse = serde_json::from_slice(&res_bytes)?;
        Ok(deferred)
    }

    /// Get a base64 encoded form of the credential logo.
    async fn image(&self, image_url: &str) -> anyhow::Result<ImageData> {
        let mut response = self.http.get(image_url).send_async().await?;
        let data = response.body_string().await?;
        let media_type = response.content_type().map_or("".into(), |ct| ct.to_string());
        Ok(ImageData { data, media_type })
    }

    /// Notify the issuer of issuance progress.
    async fn notification(
        &self, _req: NotificationRequest,
    ) -> anyhow::Result<NotificationResponse> {
        unimplemented!()
    }
}

impl<Ev> Verifier for Provider<Ev>
where Ev: 'static
{
    /// Get a request object. If an error is returned, the wallet will cancel
    /// the presentation flow.
    async fn request_object(&self, req: &str) -> anyhow::Result<RequestObjectResponse> {
        let mut response = self.http.get(req).send_async().await?;
        let res_bytes = response.body_bytes().await?;
        let request: RequestObjectResponse = serde_json::from_slice(&res_bytes)?;
        Ok(request)
    }

    /// Send the presentation to the verifier.
    async fn present(
        &self, uri: Option<&str>, presentation: &ResponseRequest,
    ) -> anyhow::Result<ResponseResponse> {
        let Some(url) = uri else {
            return Err(anyhow!("no presentation URI"));
        };
        let req_bytes = serde_json::to_vec(presentation)?;
        let mut response = self.http.post(url).body_bytes(req_bytes).send_async().await?;
        let res_bytes = response.body_bytes().await?;
        let response: ResponseResponse = serde_json::from_slice(&res_bytes)?;
        Ok(response)
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
                        Err(e) => return Err(e),
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

impl<Ev> StateStore for Provider<Ev>
where Ev: 'static,
{
    /// Store state using the provided key. The expiry parameter indicates
    /// when data can be expunged from the state store.
    async fn put(
        &self, key: &str, state: impl Serialize + Send, _expiry: DateTime<Utc>,
    ) -> anyhow::Result<()> {
        let data = serde_json::to_vec(&state)?;
        self.kv.set_async(key.into(), data).await?;
        Ok(())
    }

    /// Retrieve data using the provided key.
    async fn get<T: DeserializeOwned>(&self, key: &str) -> anyhow::Result<T> {
        let data = self.kv.get_async(key.into()).await?;
        match data {
            Some(data) => {
                let state: T = serde_json::from_slice(&data)?;
                Ok(state)
            }
            None => Err(anyhow!("no data found")),
        }
    }

    /// Remove data using the key provided.
    async fn purge(&self, key: &str) -> anyhow::Result<()> {
        self.kv.delete_async(key.into()).await?;
        Ok(())
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
