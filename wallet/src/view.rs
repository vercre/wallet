//! # View Model

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ViewModel {
    pub text: String,
    pub confirmed: bool,
}
