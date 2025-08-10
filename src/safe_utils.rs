use crate::error::{PayupError, Result};

/// Safely extracts an ID from an optional field
pub fn get_id_or_error(id: Option<String>, resource_type: &str) -> Result<String> {
    id.ok_or_else(|| {
        PayupError::Validation(format!("{} ID is required but was not provided", resource_type))
    })
}

/// Safely extracts a required string field
pub fn require_string(value: Option<String>, field_name: &str) -> Result<String> {
    value.ok_or_else(|| {
        PayupError::Validation(format!("{} is required but was not provided", field_name))
    })
}

/// Safely extracts a required numeric field
pub fn require_number<T: std::fmt::Display>(value: Option<T>, field_name: &str) -> Result<T> {
    value.ok_or_else(|| {
        PayupError::Validation(format!("{} is required but was not provided", field_name))
    })
}

/// Safely converts a value to string
pub trait SafeToString {
    fn safe_to_string(&self) -> String;
}

impl<T: std::fmt::Display> SafeToString for Option<T> {
    fn safe_to_string(&self) -> String {
        match self {
            Some(v) => v.to_string(),
            None => String::new(),
        }
    }
}

/// Provides safe access to optional fields with default values
pub trait SafeAccess<T> {
    fn or_default(&self) -> T
    where
        T: Default;
    
    fn or_value(&self, default: T) -> T;
}

impl<T: Clone> SafeAccess<T> for Option<T> {
    fn or_default(&self) -> T
    where
        T: Default,
    {
        self.clone().unwrap_or_default()
    }
    
    fn or_value(&self, default: T) -> T {
        self.clone().unwrap_or(default)
    }
}

/// Result extension for better error context
pub trait ResultExt<T> {
    fn with_context<F>(self, f: F) -> Result<T>
    where
        F: FnOnce() -> String;
}

impl<T, E> ResultExt<T> for std::result::Result<T, E>
where
    E: std::fmt::Display,
{
    fn with_context<F>(self, f: F) -> Result<T>
    where
        F: FnOnce() -> String,
    {
        self.map_err(|e| PayupError::GenericError(format!("{}: {}", f(), e)))
    }
}

/// Safe parameter extraction for API calls
pub struct SafeParams {
    params: Vec<(String, String)>,
}

impl SafeParams {
    pub fn new() -> Self {
        Self {
            params: Vec::new(),
        }
    }
    
    pub fn add_required<T: ToString>(mut self, key: &str, value: Option<T>, field_name: &str) -> Result<Self> {
        match value {
            Some(v) => {
                self.params.push((key.to_string(), v.to_string()));
                Ok(self)
            }
            None => Err(PayupError::Validation(format!("{} is required", field_name))),
        }
    }
    
    pub fn add_optional<T: ToString>(mut self, key: &str, value: Option<T>) -> Self {
        if let Some(v) = value {
            self.params.push((key.to_string(), v.to_string()));
        }
        self
    }
    
    pub fn build(self) -> Vec<(String, String)> {
        self.params
    }
}

/// Validates that a value meets certain criteria
pub fn validate<T, F>(value: T, validator: F, error_msg: &str) -> Result<T>
where
    F: FnOnce(&T) -> bool,
{
    if validator(&value) {
        Ok(value)
    } else {
        Err(PayupError::Validation(error_msg.to_string()))
    }
}

/// Validates string length
pub fn validate_string_length(value: &str, min: usize, max: usize, field_name: &str) -> Result<()> {
    let len = value.len();
    if len < min || len > max {
        Err(PayupError::Validation(format!(
            "{} must be between {} and {} characters, got {}",
            field_name, min, max, len
        )))
    } else {
        Ok(())
    }
}

/// Validates numeric range
pub fn validate_range<T>(value: T, min: T, max: T, field_name: &str) -> Result<T>
where
    T: PartialOrd + std::fmt::Display,
{
    if value < min || value > max {
        Err(PayupError::Validation(format!(
            "{} must be between {} and {}, got {}",
            field_name, min, max, value
        )))
    } else {
        Ok(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_get_id_or_error() {
        assert!(get_id_or_error(Some("id123".to_string()), "Customer").is_ok());
        assert!(get_id_or_error(None, "Customer").is_err());
    }
    
    #[test]
    fn test_safe_params() {
        let params = SafeParams::new()
            .add_optional("customer", Some("cust_123"))
            .add_optional("amount", Some(1000))
            .add_optional("missing", None::<String>)
            .build();
        
        assert_eq!(params.len(), 2);
        assert!(params.contains(&("customer".to_string(), "cust_123".to_string())));
        assert!(params.contains(&("amount".to_string(), "1000".to_string())));
    }
    
    #[test]
    fn test_validate_string_length() {
        assert!(validate_string_length("hello", 1, 10, "test").is_ok());
        assert!(validate_string_length("", 1, 10, "test").is_err());
        assert!(validate_string_length("very long string", 1, 5, "test").is_err());
    }
    
    #[test]
    fn test_validate_range() {
        assert!(validate_range(5, 1, 10, "test").is_ok());
        assert!(validate_range(0, 1, 10, "test").is_err());
        assert!(validate_range(11, 1, 10, "test").is_err());
    }
}