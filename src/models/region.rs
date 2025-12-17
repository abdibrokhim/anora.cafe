use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Region {
    pub id: String,
    pub name: String,
    pub code: String,
    pub flag: String,
    pub currency: String,
    pub free_shipping_threshold: i32,
}

impl Default for Region {
    fn default() -> Self {
        Self {
            id: "global".to_string(),
            name: "Global".to_string(),
            code: "Global".to_string(),
            flag: "🌎".to_string(),
            currency: "USD".to_string(),
            free_shipping_threshold: 40,
        }
    }
}
