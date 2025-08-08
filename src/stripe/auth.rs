use serde::{Deserialize, Serialize};

/// Stores the Stripe API client + secret.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Auth {
    pub client: String,
    pub secret: String,
}

impl Auth {
    pub fn new(client: String, secret: String) -> Self {
        Auth { client, secret }
    }
}