#![allow(dead_code)]

use crate::models::{Order, Product, Region, SavedAddress, Subscription};
use anyhow::{anyhow, Result};
use reqwest::Client;
use std::env;

/// Supabase client for database operations
pub struct SupabaseClient {
    client: Client,
    base_url: String,
    api_key: String,
}

impl SupabaseClient {
    /// Create a new Supabase client from environment variables
    pub fn new() -> Result<Self> {
        let base_url = env::var("SUPABASE_URL")
            .unwrap_or_else(|_| "".to_string());
        let api_key = env::var("SUPABASE_ANON_KEY")
            .unwrap_or_else(|_| "".to_string());

        Ok(Self {
            client: Client::new(),
            base_url,
            api_key,
        })
    }

    /// Create client with explicit credentials
    pub fn with_credentials(base_url: String, api_key: String) -> Self {
        Self {
            client: Client::new(),
            base_url,
            api_key,
        }
    }

    fn rest_url(&self, table: &str) -> String {
        format!("{}/rest/v1/{}", self.base_url, table)
    }

    /// Fetch all products (optionally filtered by region)
    pub async fn get_products(&self, region_id: Option<&str>) -> Result<Vec<Product>> {
        let url = if let Some(region) = region_id {
            format!(
                "{}?region_id=eq.{}&in_stock=eq.true&order=category.asc,name.asc",
                self.rest_url("products"),
                region
            )
        } else {
            format!(
                "{}?in_stock=eq.true&order=category.asc,name.asc",
                self.rest_url("products")
            )
        };

        let response = self
            .client
            .get(&url)
            .header("apikey", &self.api_key)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await?;

        if response.status().is_success() {
            let products: Vec<Product> = response.json().await?;
            Ok(products)
        } else {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            Err(anyhow!("Failed to fetch products: {} - {}", status, body))
        }
    }

    /// Fetch all available regions
    pub async fn get_regions(&self) -> Result<Vec<Region>> {
        let url = format!("{}?order=name.asc", self.rest_url("regions"));

        let response = self
            .client
            .get(&url)
            .header("apikey", &self.api_key)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await?;

        if response.status().is_success() {
            let regions: Vec<Region> = response.json().await?;
            Ok(regions)
        } else {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            Err(anyhow!("Failed to fetch regions: {} - {}", status, body))
        }
    }

    /// Fetch orders for a user
    pub async fn get_orders(&self, user_id: &str) -> Result<Vec<Order>> {
        let url = format!(
            "{}?user_id=eq.{}&order=created_at.desc",
            self.rest_url("orders"),
            user_id
        );

        let response = self
            .client
            .get(&url)
            .header("apikey", &self.api_key)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await?;

        if response.status().is_success() {
            let orders: Vec<Order> = response.json().await?;
            Ok(orders)
        } else {
            Ok(Vec::new())
        }
    }

    /// Fetch subscriptions for a user
    pub async fn get_subscriptions(&self, user_id: &str) -> Result<Vec<Subscription>> {
        let url = format!(
            "{}?user_id=eq.{}&order=created_at.desc",
            self.rest_url("subscriptions"),
            user_id
        );

        let response = self
            .client
            .get(&url)
            .header("apikey", &self.api_key)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await?;

        if response.status().is_success() {
            let subscriptions: Vec<Subscription> = response.json().await?;
            Ok(subscriptions)
        } else {
            Ok(Vec::new())
        }
    }

    /// Create a new order
    pub async fn create_order(&self, order: &Order) -> Result<Order> {
        let url = self.rest_url("orders");

        let response = self
            .client
            .post(&url)
            .header("apikey", &self.api_key)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .header("Prefer", "return=representation")
            .json(order)
            .send()
            .await?;

        if response.status().is_success() {
            let created: Vec<Order> = response.json().await?;
            Ok(created.into_iter().next().unwrap_or_else(|| order.clone()))
        } else {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            Err(anyhow!("Failed to create order: {} - {}", status, body))
        }
    }

    /// Create a new subscription
    pub async fn create_subscription(&self, subscription: &Subscription) -> Result<Subscription> {
        let url = self.rest_url("subscriptions");

        let response = self
            .client
            .post(&url)
            .header("apikey", &self.api_key)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .header("Prefer", "return=representation")
            .json(subscription)
            .send()
            .await?;

        if response.status().is_success() {
            let created: Vec<Subscription> = response.json().await?;
            Ok(created
                .into_iter()
                .next()
                .unwrap_or_else(|| subscription.clone()))
        } else {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            Err(anyhow!(
                "Failed to create subscription: {} - {}",
                status,
                body
            ))
        }
    }

    /// Health check
    pub async fn health_check(&self) -> Result<bool> {
        let url = format!("{}/rest/v1/", self.base_url);

        let response = self
            .client
            .get(&url)
            .header("apikey", &self.api_key)
            .send()
            .await?;

        Ok(response.status().is_success())
    }

    /// Fetch saved addresses for a user (by SSH fingerprint)
    pub async fn get_saved_addresses(&self, user_fingerprint: &str) -> Result<Vec<SavedAddress>> {
        let url = format!(
            "{}?user_fingerprint=eq.{}&order=created_at.desc&limit=3",
            self.rest_url("saved_addresses"),
            user_fingerprint
        );

        let response = self
            .client
            .get(&url)
            .header("apikey", &self.api_key)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await?;

        if response.status().is_success() {
            let addresses: Vec<SavedAddress> = response.json().await?;
            Ok(addresses)
        } else {
            // Return empty list if table doesn't exist or other error
            Ok(Vec::new())
        }
    }

    /// Save a new address for a user
    pub async fn save_address(&self, address: &SavedAddress) -> Result<SavedAddress> {
        let url = self.rest_url("saved_addresses");

        let response = self
            .client
            .post(&url)
            .header("apikey", &self.api_key)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .header("Prefer", "return=representation")
            .json(address)
            .send()
            .await?;

        if response.status().is_success() {
            let created: Vec<SavedAddress> = response.json().await?;
            Ok(created.into_iter().next().unwrap_or_else(|| address.clone()))
        } else {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            Err(anyhow!("Failed to save address: {} - {}", status, body))
        }
    }

    /// Delete a saved address by ID
    pub async fn delete_address(&self, address_id: &uuid::Uuid) -> Result<()> {
        let url = format!(
            "{}?id=eq.{}",
            self.rest_url("saved_addresses"),
            address_id
        );

        let response = self
            .client
            .delete(&url)
            .header("apikey", &self.api_key)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await?;

        if response.status().is_success() {
            Ok(())
        } else {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            Err(anyhow!("Failed to delete address: {} - {}", status, body))
        }
    }
}

impl Default for SupabaseClient {
    fn default() -> Self {
        Self::new().unwrap_or_else(|_| Self {
            client: Client::new(),
            base_url: String::new(),
            api_key: String::new(),
        })
    }
}
