use crate::error::{PayupError, Result};
use reqwest::header::{HeaderValue, RETRY_AFTER};
use serde::de::DeserializeOwned;
use serde::Deserialize;
use std::fmt;
use uuid::Uuid;

/// Stripe's header for safely retrying POST mutations.
pub const IDEMPOTENCY_KEY_HEADER: &str = "Idempotency-Key";

/// A validated Stripe idempotency key.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct IdempotencyKey(String);

impl IdempotencyKey {
    /// Validate a caller-supplied key. Stripe accepts keys up to 255 characters.
    pub fn new(value: impl Into<String>) -> Result<Self> {
        let value = value.into();
        if value.is_empty() {
            return Err(PayupError::ValidationError(
                "Stripe idempotency key must not be empty".to_string(),
            ));
        }
        if value.len() > 255 {
            return Err(PayupError::ValidationError(
                "Stripe idempotency key must be at most 255 bytes".to_string(),
            ));
        }
        HeaderValue::from_str(&value).map_err(|_| {
            PayupError::ValidationError(
                "Stripe idempotency key contains invalid header characters".to_string(),
            )
        })?;
        Ok(Self(value))
    }

    /// Generate a random key suitable for one logical mutation and all of its retries.
    pub fn generate() -> Self {
        Self(Uuid::new_v4().to_string())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Default for IdempotencyKey {
    fn default() -> Self {
        Self::generate()
    }
}

impl fmt::Display for IdempotencyKey {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

pub(crate) fn idempotent_request(
    request: reqwest::RequestBuilder,
    key: &IdempotencyKey,
) -> reqwest::RequestBuilder {
    request.header(IDEMPOTENCY_KEY_HEADER, key.as_str())
}

pub(crate) fn idempotent_blocking_request(
    request: reqwest::blocking::RequestBuilder,
    key: &IdempotencyKey,
) -> reqwest::blocking::RequestBuilder {
    request.header(IDEMPOTENCY_KEY_HEADER, key.as_str())
}

#[derive(Debug, Deserialize)]
struct StripeErrorEnvelope {
    error: StripeErrorBody,
}

#[derive(Debug, Deserialize)]
struct StripeErrorBody {
    #[serde(rename = "type")]
    error_type: Option<String>,
    code: Option<String>,
    decline_code: Option<String>,
    message: Option<String>,
    param: Option<String>,
}

fn bounded_error_body(body: &str) -> String {
    const MAX_ERROR_BODY_BYTES: usize = 4096;
    if body.len() <= MAX_ERROR_BODY_BYTES {
        return body.to_string();
    }
    let mut end = MAX_ERROR_BODY_BYTES;
    while !body.is_char_boundary(end) {
        end -= 1;
    }
    format!("{}...", &body[..end])
}

pub(crate) fn stripe_error_from_parts(
    status: u16,
    request_id: Option<String>,
    retry_after: Option<u64>,
    body: &str,
) -> PayupError {
    let parsed = serde_json::from_str::<StripeErrorEnvelope>(body).ok();
    let error = parsed.as_ref().map(|envelope| &envelope.error);
    PayupError::StripeApiError {
        status,
        error_type: error
            .and_then(|error| error.error_type.clone())
            .unwrap_or_else(|| "http_error".to_string()),
        code: error.and_then(|error| error.code.clone()),
        decline_code: error.and_then(|error| error.decline_code.clone()),
        message: error
            .and_then(|error| error.message.clone())
            .unwrap_or_else(|| bounded_error_body(body)),
        param: error.and_then(|error| error.param.clone()),
        request_id,
        retry_after,
    }
}

fn retry_after(headers: &reqwest::header::HeaderMap) -> Option<u64> {
    headers
        .get(RETRY_AFTER)
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.parse().ok())
}

fn request_id(headers: &reqwest::header::HeaderMap) -> Option<String> {
    headers
        .get("request-id")
        .and_then(|value| value.to_str().ok())
        .map(str::to_string)
}

pub(crate) async fn parse_async_response<T>(response: reqwest::Response) -> Result<T>
where
    T: DeserializeOwned,
{
    let status = response.status();
    let request_id = request_id(response.headers());
    let retry_after = retry_after(response.headers());
    let body = response.text().await.map_err(PayupError::from)?;
    if !status.is_success() {
        return Err(stripe_error_from_parts(
            status.as_u16(),
            request_id,
            retry_after,
            &body,
        ));
    }
    serde_json::from_str(&body).map_err(PayupError::from)
}

pub(crate) fn parse_blocking_response<T>(response: reqwest::blocking::Response) -> Result<T>
where
    T: DeserializeOwned,
{
    let status = response.status();
    let request_id = request_id(response.headers());
    let retry_after = retry_after(response.headers());
    let body = response.text().map_err(PayupError::from)?;
    if !status.is_success() {
        return Err(stripe_error_from_parts(
            status.as_u16(),
            request_id,
            retry_after,
            &body,
        ));
    }
    serde_json::from_str(&body).map_err(PayupError::from)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validates_idempotency_keys() {
        assert!(IdempotencyKey::new("logical-operation-1").is_ok());
        assert!(IdempotencyKey::new("").is_err());
        assert!(IdempotencyKey::new("x".repeat(256)).is_err());
        assert!(IdempotencyKey::new("contains\nnewline").is_err());
    }

    #[test]
    fn attaches_idempotency_header() {
        let key = IdempotencyKey::new("stable-key").expect("valid key");
        let request = idempotent_request(reqwest::Client::new().post("https://example.com"), &key)
            .build()
            .expect("request");
        assert_eq!(
            request.headers().get(IDEMPOTENCY_KEY_HEADER),
            Some(&HeaderValue::from_static("stable-key"))
        );
    }

    #[test]
    fn parses_structured_stripe_errors() {
        let error = stripe_error_from_parts(
            402,
            Some("req_123".to_string()),
            None,
            r#"{"error":{"type":"card_error","code":"card_declined","decline_code":"do_not_honor","message":"The card was declined.","param":"payment_method"}}"#,
        );
        match error {
            PayupError::StripeApiError {
                status,
                error_type,
                code,
                decline_code,
                request_id,
                ..
            } => {
                assert_eq!(status, 402);
                assert_eq!(error_type, "card_error");
                assert_eq!(code.as_deref(), Some("card_declined"));
                assert_eq!(decline_code.as_deref(), Some("do_not_honor"));
                assert_eq!(request_id.as_deref(), Some("req_123"));
            }
            other => panic!("unexpected error: {other:?}"),
        }
    }
}
