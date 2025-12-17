#![allow(dead_code)]

use super::{CartItem, ShippingAddress};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum OrderStatus {
    Pending,
    Processing,
    Shipped,
    Delivered,
    Cancelled,
}

impl std::fmt::Display for OrderStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OrderStatus::Pending => write!(f, "pending"),
            OrderStatus::Processing => write!(f, "processing"),
            OrderStatus::Shipped => write!(f, "shipped"),
            OrderStatus::Delivered => write!(f, "delivered"),
            OrderStatus::Cancelled => write!(f, "cancelled"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Order {
    pub id: Uuid,
    pub user_id: Uuid,
    pub items: Vec<CartItem>,
    pub shipping_address: ShippingAddress,
    pub subtotal_cents: i32,
    pub shipping_cents: i32,
    pub total_cents: i32,
    pub status: OrderStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Order {
    pub fn total_display(&self) -> String {
        format!("${:.2}", self.total_cents as f64 / 100.0)
    }

    pub fn subtotal_display(&self) -> String {
        format!("${:.2}", self.subtotal_cents as f64 / 100.0)
    }

    pub fn shipping_display(&self) -> String {
        format!("${:.2}", self.shipping_cents as f64 / 100.0)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Subscription {
    pub id: Uuid,
    pub user_id: Uuid,
    pub product_id: Uuid,
    pub product_name: String,
    pub status: SubscriptionStatus,
    pub next_delivery: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum SubscriptionStatus {
    Active,
    Paused,
    Cancelled,
}

impl std::fmt::Display for SubscriptionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SubscriptionStatus::Active => write!(f, "active"),
            SubscriptionStatus::Paused => write!(f, "paused"),
            SubscriptionStatus::Cancelled => write!(f, "cancelled"),
        }
    }
}

