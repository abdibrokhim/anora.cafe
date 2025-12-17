use std::collections::HashMap;
use std::time::{Duration, Instant};

/// Cache entry with data and expiration time
struct CacheEntry<T> {
    data: T,
    expires_at: Instant,
}

/// Simple in-memory cache with TTL
pub struct Cache<T> {
    entries: HashMap<String, CacheEntry<T>>,
    ttl: Duration,
}

impl<T: Clone> Cache<T> {
    /// Create a new cache with the given TTL in seconds
    pub fn new(ttl_secs: u64) -> Self {
        Self {
            entries: HashMap::new(),
            ttl: Duration::from_secs(ttl_secs),
        }
    }

    /// Get a cached value if it exists and hasn't expired
    pub fn get(&self, key: &str) -> Option<T> {
        self.entries.get(key).and_then(|entry| {
            if Instant::now() < entry.expires_at {
                Some(entry.data.clone())
            } else {
                None
            }
        })
    }

    /// Store a value in the cache
    pub fn set(&mut self, key: String, data: T) {
        let entry = CacheEntry {
            data,
            expires_at: Instant::now() + self.ttl,
        };
        self.entries.insert(key, entry);
    }

    /// Check if cache has a valid (non-expired) entry
    #[allow(dead_code)]
    pub fn has(&self, key: &str) -> bool {
        self.entries.get(key).map_or(false, |entry| {
            Instant::now() < entry.expires_at
        })
    }

    /// Remove an entry from the cache
    #[allow(dead_code)]
    pub fn invalidate(&mut self, key: &str) {
        self.entries.remove(key);
    }

    /// Clear all entries from the cache
    #[allow(dead_code)]
    pub fn clear(&mut self) {
        self.entries.clear();
    }
}

/// Data cache for the application - caches products and regions
pub struct DataCache {
    pub products: Cache<Vec<crate::models::Product>>,
    pub regions: Cache<Vec<crate::models::Region>>,
}

impl DataCache {
    /// Create a new data cache with default TTLs
    /// Products: 5 minutes, Regions: 30 minutes
    pub fn new() -> Self {
        Self {
            products: Cache::new(300),  // 5 minutes
            regions: Cache::new(1800),  // 30 minutes
        }
    }

    /// Get products for a region from cache
    pub fn get_products(&self, region_id: &str) -> Option<Vec<crate::models::Product>> {
        self.products.get(&format!("products:{}", region_id))
    }

    /// Cache products for a region
    pub fn set_products(&mut self, region_id: &str, products: Vec<crate::models::Product>) {
        self.products.set(format!("products:{}", region_id), products);
    }

    /// Get regions from cache
    pub fn get_regions(&self) -> Option<Vec<crate::models::Region>> {
        self.regions.get("regions")
    }

    /// Cache regions
    pub fn set_regions(&mut self, regions: Vec<crate::models::Region>) {
        self.regions.set("regions".to_string(), regions);
    }
}

impl Default for DataCache {
    fn default() -> Self {
        Self::new()
    }
}

