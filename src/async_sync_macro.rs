/// Macro to generate both async and sync versions of API methods
/// This eliminates code duplication between sync and async implementations
#[macro_export]
macro_rules! impl_sync_async {
    (
        $(#[$meta:meta])*
        pub fn $name:ident($($arg:ident: $arg_type:ty),*) -> Result<$ret:ty> {
            $($body:tt)*
        }
    ) => {
        $(#[$meta])*
        pub async fn $name($($arg: $arg_type),*) -> Result<$ret> {
            $($body)*
        }

        paste::paste! {
            $(#[$meta])*
            pub fn [<$name _sync>]($($arg: $arg_type),*) -> Result<$ret> {
                tokio::runtime::Runtime::new()
                    .map_err(|e| crate::error::PayupError::Runtime(e.to_string()))?
                    .block_on(async move {
                        Self::$name($($arg),*).await
                    })
            }
        }
    };
}

/// Macro to implement standard CRUD operations for API resources
#[macro_export]
macro_rules! impl_crud_resource {
    ($resource:ident, $endpoint:expr) => {
        impl $resource {
            pub async fn create(auth: &Auth, params: Create<$resource>) -> Result<$resource> {
                let client = HttpClient::new(STRIPE_BASE_URL)
                    .with_bearer_auth(&auth.secret);
                
                RequestBuilder::post($endpoint)
                    .form(params.to_form_params())
                    .send(&client)
                    .await
            }

            pub async fn retrieve(auth: &Auth, id: &str) -> Result<$resource> {
                let client = HttpClient::new(STRIPE_BASE_URL)
                    .with_bearer_auth(&auth.secret);
                
                RequestBuilder::get(&format!("{}/{}", $endpoint, id))
                    .send(&client)
                    .await
            }

            pub async fn update(auth: &Auth, id: &str, params: Update<$resource>) -> Result<$resource> {
                let client = HttpClient::new(STRIPE_BASE_URL)
                    .with_bearer_auth(&auth.secret);
                
                RequestBuilder::post(&format!("{}/{}", $endpoint, id))
                    .form(params.to_form_params())
                    .send(&client)
                    .await
            }

            pub async fn delete(auth: &Auth, id: &str) -> Result<DeletedResource> {
                let client = HttpClient::new(STRIPE_BASE_URL)
                    .with_bearer_auth(&auth.secret);
                
                RequestBuilder::delete(&format!("{}/{}", $endpoint, id))
                    .send(&client)
                    .await
            }

            pub async fn list(auth: &Auth, params: ListParams) -> Result<List<$resource>> {
                let client = HttpClient::new(STRIPE_BASE_URL)
                    .with_bearer_auth(&auth.secret);
                
                RequestBuilder::get($endpoint)
                    .query_opt("limit", params.limit.map(|l| l.to_string()))
                    .query_opt("starting_after", params.starting_after)
                    .query_opt("ending_before", params.ending_before)
                    .send(&client)
                    .await
            }

            pub fn create_sync(auth: &Auth, params: Create<$resource>) -> Result<$resource> {
                tokio::runtime::Runtime::new()
                    .map_err(|e| crate::error::PayupError::Runtime(e.to_string()))?
                    .block_on(Self::create(auth, params))
            }

            pub fn retrieve_sync(auth: &Auth, id: &str) -> Result<$resource> {
                tokio::runtime::Runtime::new()
                    .map_err(|e| crate::error::PayupError::Runtime(e.to_string()))?
                    .block_on(Self::retrieve(auth, id))
            }

            pub fn update_sync(auth: &Auth, id: &str, params: Update<$resource>) -> Result<$resource> {
                tokio::runtime::Runtime::new()
                    .map_err(|e| crate::error::PayupError::Runtime(e.to_string()))?
                    .block_on(Self::update(auth, id, params))
            }

            pub fn delete_sync(auth: &Auth, id: &str) -> Result<DeletedResource> {
                tokio::runtime::Runtime::new()
                    .map_err(|e| crate::error::PayupError::Runtime(e.to_string()))?
                    .block_on(Self::delete(auth, id))
            }

            pub fn list_sync(auth: &Auth, params: ListParams) -> Result<List<$resource>> {
                tokio::runtime::Runtime::new()
                    .map_err(|e| crate::error::PayupError::Runtime(e.to_string()))?
                    .block_on(Self::list(auth, params))
            }
        }
    };
}

/// Macro to generate parameter builders with fluent API
#[macro_export]
macro_rules! impl_param_builder {
    ($name:ident {
        required: { $($req_field:ident: $req_type:ty),* $(,)? },
        optional: { $($opt_field:ident: $opt_type:ty),* $(,)? }
    }) => {
        #[derive(Debug, Clone)]
        pub struct $name {
            $(pub $req_field: $req_type,)*
            $(pub $opt_field: Option<$opt_type>,)*
        }

        impl $name {
            pub fn new($($req_field: $req_type),*) -> Self {
                Self {
                    $($req_field,)*
                    $($opt_field: None,)*
                }
            }

            $(
                pub fn $opt_field(mut self, value: $opt_type) -> Self {
                    self.$opt_field = Some(value);
                    self
                }

                paste::paste! {
                    pub fn [<with_ $opt_field>](mut self, value: $opt_type) -> Self {
                        self.$opt_field = Some(value);
                        self
                    }

                    pub fn [<maybe_ $opt_field>](mut self, value: Option<$opt_type>) -> Self {
                        self.$opt_field = value;
                        self
                    }
                }
            )*

            pub fn to_form_params(&self) -> Vec<(String, String)> {
                let mut params = vec![];
                
                $(
                    params.push((stringify!($req_field).to_string(), self.$req_field.to_string()));
                )*
                
                $(
                    if let Some(ref value) = self.$opt_field {
                        params.push((stringify!($opt_field).to_string(), value.to_string()));
                    }
                )*
                
                params
            }
        }
    };
}

/// Macro to implement retry logic with exponential backoff
#[macro_export]
macro_rules! retry_with_backoff {
    ($operation:expr, $max_attempts:expr) => {{
        use std::time::Duration;
        use tokio::time::sleep;
        
        let mut attempt = 0;
        let mut last_error = None;
        
        while attempt < $max_attempts {
            match $operation {
                Ok(result) => return Ok(result),
                Err(e) => {
                    last_error = Some(e);
                    attempt += 1;
                    
                    if attempt < $max_attempts {
                        let delay = Duration::from_millis(100 * (2_u64.pow(attempt)));
                        sleep(delay).await;
                    }
                }
            }
        }
        
        Err(last_error.unwrap())
    }};
}

/// Macro to validate required parameters
#[macro_export]
macro_rules! validate_params {
    ($($param:expr => $error_msg:expr),* $(,)?) => {
        $(
            if $param.is_none() || $param.as_ref().map(|s| s.is_empty()).unwrap_or(false) {
                return Err(crate::error::PayupError::Validation($error_msg.to_string()));
            }
        )*
    };
}

/// Macro to handle API responses with proper error mapping
#[macro_export]
macro_rules! handle_api_response {
    ($response:expr) => {{
        let status = $response.status();
        
        if status.is_success() {
            $response.json().await
                .map_err(|e| crate::error::PayupError::Deserialization(e.to_string()))
        } else {
            let error_body = $response.text().await
                .unwrap_or_else(|_| format!("HTTP error {}", status.as_u16()));
            
            match status.as_u16() {
                400 => Err(crate::error::PayupError::BadRequest(error_body)),
                401 => Err(crate::error::PayupError::Unauthorized(error_body)),
                403 => Err(crate::error::PayupError::Forbidden(error_body)),
                404 => Err(crate::error::PayupError::NotFound(error_body)),
                429 => Err(crate::error::PayupError::RateLimited(error_body)),
                500..=599 => Err(crate::error::PayupError::ServerError(error_body)),
                _ => Err(crate::error::PayupError::Http(format!("HTTP {}: {}", status, error_body))),
            }
        }
    }};
}

/// Macro to generate test fixtures
#[macro_export]
macro_rules! test_fixture {
    ($name:ident: $type:ty = $value:expr) => {
        #[cfg(test)]
        pub fn $name() -> $type {
            $value
        }
    };
}