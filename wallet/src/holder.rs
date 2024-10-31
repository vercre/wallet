//! Mock-up of a crate that uses a provider model for callbacks.
// use std::future::Future;
// use std::collections::HashMap;

// use chrono::{DateTime, Duration, Utc};
// use serde::de::DeserializeOwned;
// use serde::Serialize;

// // Sample request for metadata
// pub struct MetadataRequest {
//     pub credential_issuer: String,
// }

// // Sample response for metadata. A real one is an elaborate struct.
// #[derive(Serialize)]
// pub struct MetadataResponse {
//     pub credential_issuer: String,
//     pub credential_schema: String,
// }

// // Sample credential. A real one would is an elaborate struct.
// pub struct Credential {
//     pub issuer: String,
//     pub subject: String,
//     pub claims: HashMap<String, String>,
// }

// // Sample provider trait
// pub trait Provider {
//     // Get me some metadata. Probably using a HTTP request.
//     fn metadata(&self, req: MetadataRequest) -> impl Future<Output = anyhow::Result<MetadataResponse>> + Send;

//     // Save some state, probably in a KV store, so I can remember things between steps in the flow.
//     fn put_state(&self, key: &str, state: impl Serialize + Send, expiry: DateTime<Utc>) -> impl Future<Output = anyhow::Result<()>> + Send;

//     // Get some state I asked you to save earlier.
//     fn get_state<T: DeserializeOwned>(&self, key: &str) -> impl Future<Output = anyhow::Result<T>> + Send;

//     // Store a credential in your persistent storage so you can present it sometime in the future.
//     fn save(&self, credential: Credential) -> impl Future<Output = anyhow::Result<()>> + Send;
// }

// // Sample crate function that uses the provider.
// pub async fn do_stuff(provider: impl Provider, issuer: &str) -> anyhow::Result<()> {
//     // Get some metadata
//     let metadata = provider.metadata(MetadataRequest { credential_issuer: issuer.into() }).await?;

//     // Save some state
//     provider.put_state("key", "value", Utc::now() + Duration::days(1)).await?;

//     let mut claims = HashMap::new();
//     claims.insert("name".into(), "Alice".into());
//     claims.insert("age".into(), "30".into());

//     // Save a credential
//     provider.save(Credential {
//         issuer: "issuer".into(),
//         subject: "example".to_string(),
//         claims,
//     }).await?;

//     Ok(())
// }
