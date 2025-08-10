use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Default)]
pub struct ParameterBuilder {
    params: HashMap<String, String>,
}

impl ParameterBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.params.insert(key.into(), value.into());
        self
    }

    pub fn add_opt(self, key: impl Into<String>, value: Option<impl Into<String>>) -> Self {
        if let Some(v) = value {
            self.add(key, v)
        } else {
            self
        }
    }

    pub fn add_bool(self, key: impl Into<String>, value: bool) -> Self {
        self.add(key, value.to_string())
    }

    pub fn add_number<T: ToString>(self, key: impl Into<String>, value: T) -> Self {
        self.add(key, value.to_string())
    }

    pub fn add_opt_number<T: ToString>(self, key: impl Into<String>, value: Option<T>) -> Self {
        if let Some(v) = value {
            self.add_number(key, v)
        } else {
            self
        }
    }

    pub fn add_list<T: ToString>(self, key: impl Into<String>, values: Vec<T>) -> Self {
        let joined = values.iter()
            .map(|v| v.to_string())
            .collect::<Vec<_>>()
            .join(",");
        self.add(key, joined)
    }

    pub fn add_metadata(mut self, metadata: HashMap<String, String>) -> Self {
        for (k, v) in metadata {
            self.params.insert(format!("metadata[{}]", k), v);
        }
        self
    }

    pub fn build(self) -> Vec<(String, String)> {
        self.params.into_iter().collect()
    }

    pub fn build_refs(&self) -> Vec<(&str, &str)> {
        self.params.iter()
            .map(|(k, v)| (k.as_str(), v.as_str()))
            .collect()
    }

    pub fn to_query_string(&self) -> String {
        self.params.iter()
            .map(|(k, v)| format!("{}={}", k, urlencoding::encode(v)))
            .collect::<Vec<_>>()
            .join("&")
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageRequest {
    pub limit: Option<usize>,
    pub starting_after: Option<String>,
    pub ending_before: Option<String>,
}

impl PageRequest {
    pub fn new() -> Self {
        Self {
            limit: None,
            starting_after: None,
            ending_before: None,
        }
    }

    pub fn limit(mut self, limit: usize) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn starting_after(mut self, cursor: impl Into<String>) -> Self {
        self.starting_after = Some(cursor.into());
        self.ending_before = None;
        self
    }

    pub fn ending_before(mut self, cursor: impl Into<String>) -> Self {
        self.ending_before = Some(cursor.into());
        self.starting_after = None;
        self
    }

    pub fn to_params(&self) -> ParameterBuilder {
        ParameterBuilder::new()
            .add_opt_number("limit", self.limit)
            .add_opt("starting_after", self.starting_after.clone())
            .add_opt("ending_before", self.ending_before.clone())
    }
}

#[macro_export]
macro_rules! impl_builder {
    ($struct_name:ident { $($field:ident: $field_type:ty),* $(,)? }) => {
        #[derive(Debug, Clone, Default)]
        pub struct $struct_name {
            $(pub $field: Option<$field_type>,)*
        }

        impl $struct_name {
            pub fn new() -> Self {
                Self::default()
            }

            $(
                pub fn $field(mut self, value: $field_type) -> Self {
                    self.$field = Some(value);
                    self
                }

                paste::paste! {
                    pub fn [<with_ $field>](mut self, value: $field_type) -> Self {
                        self.$field = Some(value);
                        self
                    }
                }
            )*

            pub fn build(self) -> Self {
                self
            }
        }
    };
}

#[macro_export]
macro_rules! async_handler {
    ($async_fn:expr, $sync_fn:expr) => {
        {
            use tokio::runtime::Runtime;
            
            if let Ok(handle) = tokio::runtime::Handle::try_current() {
                handle.block_on($async_fn)
            } else {
                let rt = Runtime::new().map_err(|e| {
                    crate::error::PayupError::Runtime(format!("Failed to create runtime: {}", e))
                })?;
                rt.block_on($async_fn)
            }
        }
    };
}

#[macro_export]
macro_rules! ensure {
    ($cond:expr, $err:expr) => {
        if !$cond {
            return Err($err);
        }
    };
}

#[macro_export]
macro_rules! ensure_some {
    ($opt:expr, $err:expr) => {
        match $opt {
            Some(val) => val,
            None => return Err($err),
        }
    };
}

pub trait Validate {
    type Error;
    
    fn validate(&self) -> Result<(), Self::Error>;
}

pub trait ToParams {
    fn to_params(&self) -> ParameterBuilder;
}

pub trait FromResponse: Sized {
    type Response;
    type Error;
    
    fn from_response(response: Self::Response) -> Result<Self, Self::Error>;
}