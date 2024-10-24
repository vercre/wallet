//! # Application configuration.
//!
//! Some hard-coded configuration values for the wallet app. In a real app,
//! each installed instance would need its own configuration.

/// Get the client ID for the wallet app. In practice this should be a unique
/// device ID that has been registered with the issuer.
#[allow(dead_code)]
pub fn client_id() -> String {
    "96bfb9cb-0513-7d64-5532-bed74c48f9ab".to_string()
}
