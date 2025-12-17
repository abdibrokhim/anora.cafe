use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ProductCategory {
    Featured,
    Originals,
}

impl std::fmt::Display for ProductCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProductCategory::Featured => write!(f, "~ featured ~"),
            ProductCategory::Originals => write!(f, "~ originals ~"),
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum RoastLevel {
    Light,
    Medium,
    Dark,
}

impl std::fmt::Display for RoastLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RoastLevel::Light => write!(f, "light roast"),
            RoastLevel::Medium => write!(f, "medium roast"),
            RoastLevel::Dark => write!(f, "dark roast"),
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ProductType {
    Subscription,
    OneTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Product {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub description: String,
    pub price_cents: i32,
    pub category: ProductCategory,
    pub roast_level: Option<RoastLevel>,
    pub weight_oz: i32,
    pub bean_type: String,
    pub product_type: ProductType,
    pub highlight_color: String,
    pub region_id: String,
    pub in_stock: bool,
}

impl Product {
    pub fn price_display(&self) -> String {
        format!("${}", self.price_cents / 100)
    }

    pub fn details_line(&self) -> String {
        if let Some(roast) = &self.roast_level {
            format!("{} | {}oz | {}", roast, self.weight_oz, self.bean_type)
        } else {
            format!("{}oz", self.weight_oz)
        }
    }
}
