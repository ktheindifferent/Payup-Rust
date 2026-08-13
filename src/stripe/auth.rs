use serde::{Deserialize, Serialize};

/// Stores the Stripe API client + secret.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Auth {
    pub client: String,
    pub secret: String,
}

#[cfg(test)]
mod tests {
    use super::Auth;

    #[test]
    fn api_key_supports_legacy_and_secret_key_layouts() {
        assert_eq!(Auth::new("sk_legacy".into(), String::new()).api_key(), "sk_legacy");
        assert_eq!(
            Auth::new("pk_public".into(), "sk_secret".into()).api_key(),
            "sk_secret"
        );
    }
}

impl Auth {
    pub fn new(client: String, secret: String) -> Self {
        Auth { client, secret }
    }

    /// Return the Stripe API key while preserving compatibility with both legacy
    /// `Auth::new(api_key, "")` and `Auth::new(public_key, secret_key)` callers.
    pub fn api_key(&self) -> &str {
        if self.secret.is_empty() {
            &self.client
        } else {
            &self.secret
        }
    }
}
