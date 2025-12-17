#![allow(dead_code)]

use super::Product;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CartItem {
    pub id: Uuid,
    pub product: Product,
    pub quantity: i32,
}

impl CartItem {
    pub fn new(product: Product, quantity: i32) -> Self {
        Self {
            id: Uuid::new_v4(),
            product,
            quantity,
        }
    }

    pub fn total_cents(&self) -> i32 {
        self.product.price_cents * self.quantity
    }

    pub fn total_display(&self) -> String {
        format!("${}", self.total_cents() / 100)
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Cart {
    pub items: Vec<CartItem>,
}

impl Cart {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    pub fn add_item(&mut self, product: Product, quantity: i32) {
        // Check if product already exists in cart
        if let Some(item) = self.items.iter_mut().find(|i| i.product.id == product.id) {
            item.quantity += quantity;
        } else {
            self.items.push(CartItem::new(product, quantity));
        }
    }

    pub fn remove_item(&mut self, product_id: Uuid) {
        self.items.retain(|i| i.product.id != product_id);
    }

    pub fn update_quantity(&mut self, product_id: Uuid, quantity: i32) {
        if let Some(item) = self.items.iter_mut().find(|i| i.product.id == product_id) {
            if quantity <= 0 {
                self.remove_item(product_id);
            } else {
                item.quantity = quantity;
            }
        }
    }

    pub fn increment_item(&mut self, product_id: Uuid) {
        if let Some(item) = self.items.iter_mut().find(|i| i.product.id == product_id) {
            item.quantity += 1;
        }
    }

    pub fn decrement_item(&mut self, product_id: Uuid) {
        if let Some(item) = self.items.iter_mut().find(|i| i.product.id == product_id) {
            if item.quantity > 1 {
                item.quantity -= 1;
            } else {
                self.remove_item(product_id);
            }
        }
    }

    pub fn total_items(&self) -> i32 {
        self.items.iter().map(|i| i.quantity).sum()
    }

    pub fn subtotal_cents(&self) -> i32 {
        self.items.iter().map(|i| i.total_cents()).sum()
    }

    pub fn subtotal_display(&self) -> String {
        format!("${}", self.subtotal_cents() / 100)
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn clear(&mut self) {
        self.items.clear();
    }
}

