#![allow(dead_code)]

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct ShippingAddress {
    pub name: String,
    pub street_1: String,
    pub street_2: String,
    pub city: String,
    pub state: String,
    pub country: String,
    pub phone: String,
    pub postal_code: String,
}

impl ShippingAddress {
    pub fn is_complete(&self) -> bool {
        !self.name.is_empty()
            && !self.street_1.is_empty()
            && !self.city.is_empty()
            && !self.country.is_empty()
            && !self.postal_code.is_empty()
    }

    /// Get a one-line display of the address
    pub fn display_line(&self) -> String {
        let mut parts = vec![];
        if !self.street_1.is_empty() {
            parts.push(self.street_1.clone());
        }
        if !self.city.is_empty() {
            parts.push(self.city.clone());
        }
        if !self.state.is_empty() {
            parts.push(self.state.clone());
        }
        if !self.country.is_empty() {
            parts.push(self.country.clone());
        }
        if !self.postal_code.is_empty() {
            parts.push(self.postal_code.clone());
        }
        parts.join(", ")
    }
}

/// Saved address for Supabase storage (includes user identification)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SavedAddress {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Uuid>,
    pub user_fingerprint: String,
    pub name: String,
    pub street_1: String,
    pub street_2: String,
    pub city: String,
    pub state: String,
    pub country: String,
    pub phone: String,
    pub postal_code: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<DateTime<Utc>>,
}

impl SavedAddress {
    /// Create a new saved address from a ShippingAddress
    pub fn from_shipping(address: &ShippingAddress, user_fingerprint: &str) -> Self {
        Self {
            id: None,
            user_fingerprint: user_fingerprint.to_string(),
            name: address.name.clone(),
            street_1: address.street_1.clone(),
            street_2: address.street_2.clone(),
            city: address.city.clone(),
            state: address.state.clone(),
            country: address.country.clone(),
            phone: address.phone.clone(),
            postal_code: address.postal_code.clone(),
            created_at: None,
        }
    }

    /// Convert to ShippingAddress for local use
    pub fn to_shipping(&self) -> ShippingAddress {
        ShippingAddress {
            name: self.name.clone(),
            street_1: self.street_1.clone(),
            street_2: self.street_2.clone(),
            city: self.city.clone(),
            state: self.state.clone(),
            country: self.country.clone(),
            phone: self.phone.clone(),
            postal_code: self.postal_code.clone(),
        }
    }

    /// Get a one-line display of the address
    pub fn display_line(&self) -> String {
        let mut parts = vec![];
        if !self.street_1.is_empty() {
            parts.push(self.street_1.clone());
        }
        if !self.city.is_empty() {
            parts.push(self.city.clone());
        }
        if !self.state.is_empty() {
            parts.push(self.state.clone());
        }
        if !self.country.is_empty() {
            parts.push(self.country.clone());
        }
        if !self.postal_code.is_empty() {
            parts.push(self.postal_code.clone());
        }
        parts.join(", ")
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PaymentInfo {
    pub name: String,
    pub email: String,
    pub card_number: String,
    pub expiry_month: String,
    pub expiry_year: String,
    pub cvv: String,
}

impl PaymentInfo {
    pub fn is_complete(&self) -> bool {
        !self.name.is_empty()
            && !self.email.is_empty()
            && !self.card_number.is_empty()
            && !self.expiry_month.is_empty()
            && !self.expiry_year.is_empty()
            && !self.cvv.is_empty()
    }

    pub fn masked_card(&self) -> String {
        if self.card_number.len() >= 4 {
            format!("**** **** **** {}", &self.card_number[self.card_number.len() - 4..])
        } else {
            "****".to_string()
        }
    }
}
