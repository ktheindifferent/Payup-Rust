use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PaymentMethod {
    pub id: Option<String>,
    pub object: Option<String>,
    // Add other fields from the original struct as needed
}

impl PaymentMethod {
    pub fn new() -> Self {
        PaymentMethod {
            id: None,
            object: None,
        }
    }
    
    // Add other methods from the original implementation as needed
}