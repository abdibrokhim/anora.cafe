use crate::db::{DataCache, SshIdentity, SupabaseClient};
use crate::models::{Cart, Order, PaymentInfo, Product, Region, SavedAddress, ShippingAddress, Subscription};
use anyhow::Result;
use std::time::Instant;

/// Main application tabs
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Tab {
    #[default]
    Home,
    Shop,
    Account,
    Cart,
}

/// Account section tabs
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AccountSection {
    #[default]
    OrderHistory,
    Subscriptions,
    Faq,
    About,
}

/// Checkout flow steps
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CheckoutStep {
    #[default]
    Cart,
    Shipping,
    Payment,
    Confirmation,
}

/// Shipping step mode - selecting saved address or adding new one
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ShippingMode {
    #[default]
    SelectAddress,
    AddNewAddress,
}

/// Payment method options
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PaymentMethod {
    Ssh,
    Browser,
}

/// Input field being edited
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum InputField {
    #[default]
    None,
    // Shipping fields (left column)
    Name,
    Street1,
    Street2,
    City,
    // Shipping fields (right column)
    State,
    Country,
    Phone,
    PostalCode,
    // Payment fields
    PaymentName,
    PaymentEmail,
    CardNumber,
    ExpiryMonth,
    ExpiryYear,
    Cvv,
}

impl InputField {
    pub fn shipping_fields() -> Vec<Self> {
        // Order: left column top-to-bottom, then right column top-to-bottom
        vec![
            Self::Name,
            Self::Street1,
            Self::Street2,
            Self::City,
            Self::State,
            Self::Country,
            Self::Phone,
            Self::PostalCode,
        ]
    }

    pub fn payment_fields() -> Vec<Self> {
        vec![
            Self::PaymentName,
            Self::PaymentEmail,
            Self::CardNumber,
            Self::ExpiryMonth,
            Self::ExpiryYear,
            Self::Cvv,
        ]
    }

    pub fn next_shipping(&self) -> Self {
        let fields = Self::shipping_fields();
        let current_idx = fields.iter().position(|f| f == self).unwrap_or(0);
        fields.get(current_idx + 1).copied().unwrap_or(fields[0])
    }

    pub fn next_payment(&self) -> Self {
        let fields = Self::payment_fields();
        let current_idx = fields.iter().position(|f| f == self).unwrap_or(0);
        fields.get(current_idx + 1).copied().unwrap_or(fields[0])
    }
}

/// Loading state for async operations
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LoadingState {
    #[default]
    Idle,
    Loading,
    Error,
}

/// Main application state
pub struct App {
    pub running: bool,
    pub current_tab: Tab,
    pub region: Region,
    pub regions: Vec<Region>,
    pub products: Vec<Product>,
    pub cart: Cart,
    pub orders: Vec<Order>,
    pub subscriptions: Vec<Subscription>,

    // UI state
    pub selected_product_index: usize,
    pub product_quantity: i32,
    pub account_section: AccountSection,
    pub checkout_step: CheckoutStep,
    pub cart_item_index: usize,
    pub payment_option_index: usize,
    pub payment_method: Option<PaymentMethod>,

    // Form data
    pub shipping_address: ShippingAddress,
    pub saved_addresses: Vec<SavedAddress>,
    pub shipping_mode: ShippingMode,
    pub address_select_index: usize,
    pub payment_info: PaymentInfo,
    pub active_input: InputField,

    // Notification message (for errors)
    pub notification: Option<String>,

    // Loading state
    pub loading: LoadingState,

    // Splash screen state
    pub show_splash: bool,
    pub splash_start: Instant,

    // User identity (SSH key fingerprint)
    pub identity: SshIdentity,

    // Database client and cache
    pub db: SupabaseClient,
    pub cache: DataCache,
}

impl App {
    pub fn new() -> Self {
        let db = SupabaseClient::default();
        let cache = DataCache::new();
        let identity = SshIdentity::get_or_create();
        // Start with a default region, will be updated when regions are loaded
        let region = Region::default();

        Self {
            running: true,
            current_tab: Tab::Home,
            region,
            regions: Vec::new(),
            products: Vec::new(),
            cart: Cart::new(),
            orders: Vec::new(),
            subscriptions: Vec::new(),
            selected_product_index: 0,
            product_quantity: 1,
            account_section: AccountSection::OrderHistory,
            checkout_step: CheckoutStep::Cart,
            cart_item_index: 0,
            payment_option_index: 0,
            payment_method: None,
            shipping_address: ShippingAddress::default(),
            saved_addresses: Vec::new(),
            shipping_mode: ShippingMode::SelectAddress,
            address_select_index: 0,
            payment_info: PaymentInfo::default(),
            active_input: InputField::None,
            notification: None,
            loading: LoadingState::Idle,
            show_splash: true,
            splash_start: Instant::now(),
            identity,
            db,
            cache,
        }
    }

    /// Check if splash screen duration has elapsed (5 seconds)
    pub fn check_splash_timeout(&mut self) {
        if self.show_splash && self.splash_start.elapsed().as_secs() >= 5 {
            self.show_splash = false;
        }
    }

    /// Skip splash screen immediately
    pub fn skip_splash(&mut self) {
        self.show_splash = false;
    }

    /// Load regions from Supabase (with caching)
    pub async fn load_regions(&mut self) -> Result<()> {
        // Check cache first
        if let Some(regions) = self.cache.get_regions() {
            self.regions = regions;
            if self.region.id.is_empty() && !self.regions.is_empty() {
                self.region = self.regions[0].clone();
            }
            return Ok(());
        }

        // Fetch from Supabase
        self.loading = LoadingState::Loading;
        match self.db.get_regions().await {
            Ok(regions) if !regions.is_empty() => {
                self.cache.set_regions(regions.clone());
                self.regions = regions;
                if self.region.id.is_empty() && !self.regions.is_empty() {
                    self.region = self.regions[0].clone();
                }
                self.loading = LoadingState::Idle;
            }
            Ok(_) => {
                // No regions in database, use a sensible default
                self.regions = vec![Region::default()];
                self.region = Region::default();
                self.loading = LoadingState::Idle;
            }
            Err(e) => {
                self.loading = LoadingState::Error;
                self.notification = Some(format!("Failed to load regions: {}", e));
                // Use default region on error
                self.regions = vec![Region::default()];
                self.region = Region::default();
            }
        }
        Ok(())
    }

    /// Load products for the current region (with caching)
    pub async fn load_products(&mut self) -> Result<()> {
        // Check cache first
        if let Some(products) = self.cache.get_products(&self.region.id) {
            self.products = products;
            return Ok(());
        }

        // Fetch from Supabase
        self.loading = LoadingState::Loading;
        match self.db.get_products(Some(&self.region.id)).await {
            Ok(products) => {
                self.cache.set_products(&self.region.id, products.clone());
                self.products = products;
                self.loading = LoadingState::Idle;
            }
            Err(e) => {
                self.loading = LoadingState::Error;
                self.notification = Some(format!("Failed to load products: {}", e));
                self.products = Vec::new();
            }
        }
        Ok(())
    }

    /// Load saved addresses from Supabase
    pub async fn load_saved_addresses(&mut self) -> Result<()> {
        match self.db.get_saved_addresses(&self.identity.fingerprint).await {
            Ok(addresses) => {
                self.saved_addresses = addresses;
            }
            Err(_) => {
                // Silently fail - addresses are optional
                self.saved_addresses = Vec::new();
            }
        }
        Ok(())
    }

    /// Save current address to Supabase
    pub async fn save_address_to_db(&mut self) -> Result<()> {
        if !self.shipping_address.is_complete() || self.saved_addresses.len() >= 3 {
            return Ok(());
        }

        // Check if address already exists
        let exists = self.saved_addresses.iter().any(|a| {
            a.street_1 == self.shipping_address.street_1
                && a.city == self.shipping_address.city
                && a.postal_code == self.shipping_address.postal_code
        });

        if exists {
            return Ok(());
        }

        let saved_address = SavedAddress::from_shipping(&self.shipping_address, &self.identity.fingerprint);
        
        match self.db.save_address(&saved_address).await {
            Ok(created) => {
                self.saved_addresses.insert(0, created);
                // Keep only 3 addresses
                if self.saved_addresses.len() > 3 {
                    self.saved_addresses.truncate(3);
                }
            }
            Err(_) => {
                // Silently fail - continue with checkout
            }
        }
        Ok(())
    }

    /// Delete a saved address from Supabase
    pub async fn delete_address_from_db(&mut self, index: usize) -> Result<()> {
        if index >= self.saved_addresses.len() {
            return Ok(());
        }

        let address = &self.saved_addresses[index];
        if let Some(id) = address.id {
            let _ = self.db.delete_address(&id).await;
        }
        self.saved_addresses.remove(index);
        
        // Adjust selection index if needed
        if self.address_select_index >= self.saved_addresses.len() + 1 && self.address_select_index > 0 {
            self.address_select_index -= 1;
        }
        Ok(())
    }

    /// Initial data load (regions + products + saved addresses)
    pub async fn load_initial_data(&mut self) -> Result<()> {
        self.load_regions().await?;
        self.load_products().await?;
        self.load_saved_addresses().await?;
        Ok(())
    }

    /// Change region and reload products
    pub async fn change_region(&mut self, region: Region) {
        self.region = region;
        let _ = self.load_products().await;
        self.selected_product_index = 0;
    }

    /// Add current product to cart
    pub fn add_to_cart(&mut self) {
        if let Some(product) = self.products.get(self.selected_product_index) {
            self.cart.add_item(product.clone(), self.product_quantity);
            self.product_quantity = 1; // Reset quantity
        }
    }

    /// Process current input character
    pub fn handle_input_char(&mut self, c: char) {
        // Clear notification when user starts typing
        self.notification = None;

        match self.active_input {
            InputField::None => {}
            InputField::Name => self.shipping_address.name.push(c),
            InputField::Street1 => self.shipping_address.street_1.push(c),
            InputField::Street2 => self.shipping_address.street_2.push(c),
            InputField::City => self.shipping_address.city.push(c),
            InputField::State => self.shipping_address.state.push(c),
            InputField::Country => self.shipping_address.country.push(c),
            InputField::Phone => self.shipping_address.phone.push(c),
            InputField::PostalCode => self.shipping_address.postal_code.push(c),
            InputField::PaymentName => self.payment_info.name.push(c),
            InputField::PaymentEmail => self.payment_info.email.push(c),
            InputField::CardNumber => {
                if c.is_ascii_digit() && self.payment_info.card_number.len() < 16 {
                    self.payment_info.card_number.push(c);
                }
            }
            InputField::ExpiryMonth => {
                if c.is_ascii_digit() && self.payment_info.expiry_month.len() < 2 {
                    self.payment_info.expiry_month.push(c);
                }
            }
            InputField::ExpiryYear => {
                if c.is_ascii_digit() && self.payment_info.expiry_year.len() < 4 {
                    self.payment_info.expiry_year.push(c);
                }
            }
            InputField::Cvv => {
                if c.is_ascii_digit() && self.payment_info.cvv.len() < 3 {
                    self.payment_info.cvv.push(c);
                }
            }
        }
    }

    /// Handle backspace in input
    pub fn handle_input_backspace(&mut self) {
        match self.active_input {
            InputField::None => {}
            InputField::Name => {
                self.shipping_address.name.pop();
            }
            InputField::Street1 => {
                self.shipping_address.street_1.pop();
            }
            InputField::Street2 => {
                self.shipping_address.street_2.pop();
            }
            InputField::City => {
                self.shipping_address.city.pop();
            }
            InputField::State => {
                self.shipping_address.state.pop();
            }
            InputField::Country => {
                self.shipping_address.country.pop();
            }
            InputField::Phone => {
                self.shipping_address.phone.pop();
            }
            InputField::PostalCode => {
                self.shipping_address.postal_code.pop();
            }
            InputField::PaymentName => {
                self.payment_info.name.pop();
            }
            InputField::PaymentEmail => {
                self.payment_info.email.pop();
            }
            InputField::CardNumber => {
                self.payment_info.card_number.pop();
            }
            InputField::ExpiryMonth => {
                self.payment_info.expiry_month.pop();
            }
            InputField::ExpiryYear => {
                self.payment_info.expiry_year.pop();
            }
            InputField::Cvv => {
                self.payment_info.cvv.pop();
            }
        }
    }

    /// Move to next input field
    pub fn next_input_field(&mut self) {
        // Clear notification when navigating fields
        self.notification = None;

        match self.checkout_step {
            CheckoutStep::Shipping => {
                self.active_input = self.active_input.next_shipping();
            }
            CheckoutStep::Payment if self.payment_method == Some(PaymentMethod::Ssh) => {
                self.active_input = self.active_input.next_payment();
            }
            _ => {}
        }
    }

    /// Navigate products
    pub fn next_product(&mut self) {
        if !self.products.is_empty() {
            self.selected_product_index = (self.selected_product_index + 1) % self.products.len();
            self.product_quantity = 1;
        }
    }

    pub fn prev_product(&mut self) {
        if !self.products.is_empty() {
            self.selected_product_index = self
                .selected_product_index
                .checked_sub(1)
                .unwrap_or(self.products.len() - 1);
            self.product_quantity = 1;
        }
    }

    /// Navigate account sections
    pub fn next_account_section(&mut self) {
        self.account_section = match self.account_section {
            AccountSection::OrderHistory => AccountSection::Subscriptions,
            AccountSection::Subscriptions => AccountSection::Faq,
            AccountSection::Faq => AccountSection::About,
            AccountSection::About => AccountSection::OrderHistory,
        };
    }

    pub fn prev_account_section(&mut self) {
        self.account_section = match self.account_section {
            AccountSection::OrderHistory => AccountSection::About,
            AccountSection::Subscriptions => AccountSection::OrderHistory,
            AccountSection::Faq => AccountSection::Subscriptions,
            AccountSection::About => AccountSection::Faq,
        };
    }

    /// Navigate cart items
    pub fn next_cart_item(&mut self) {
        if !self.cart.items.is_empty() {
            self.cart_item_index = (self.cart_item_index + 1) % self.cart.items.len();
        }
    }

    pub fn prev_cart_item(&mut self) {
        if !self.cart.items.is_empty() {
            self.cart_item_index = self
                .cart_item_index
                .checked_sub(1)
                .unwrap_or(self.cart.items.len() - 1);
        }
    }

    /// Navigate payment options
    pub fn next_payment_option(&mut self) {
        self.payment_option_index = (self.payment_option_index + 1) % 2;
    }

    pub fn prev_payment_option(&mut self) {
        self.payment_option_index = self.payment_option_index.checked_sub(1).unwrap_or(1);
    }

    /// Navigate address selection (saved addresses + "add new address" option)
    pub fn next_address_option(&mut self) {
        // Total options = saved addresses count + 1 (for "add new address")
        let total = self.saved_addresses.len() + 1;
        self.address_select_index = (self.address_select_index + 1) % total;
    }

    pub fn prev_address_option(&mut self) {
        let total = self.saved_addresses.len() + 1;
        self.address_select_index = self.address_select_index.checked_sub(1).unwrap_or(total - 1);
    }

    /// Select the current address option
    pub fn select_address_option(&mut self) {
        if self.address_select_index < self.saved_addresses.len() {
            // Selected a saved address - convert to ShippingAddress
            self.shipping_address = self.saved_addresses[self.address_select_index].to_shipping();
            // Proceed to payment
            self.active_input = InputField::None;
            self.checkout_step = CheckoutStep::Payment;
        } else {
            // Selected "add new address"
            self.shipping_mode = ShippingMode::AddNewAddress;
            self.shipping_address = ShippingAddress::default();
            self.active_input = InputField::Name;
        }
    }

    /// Remove the selected saved address (async for DB deletion)
    pub async fn remove_selected_address(&mut self) {
        if self.address_select_index < self.saved_addresses.len() {
            let _ = self.delete_address_from_db(self.address_select_index).await;
        }
    }

    /// Check if we're in address selection mode
    #[allow(dead_code)]
    pub fn is_selecting_address(&self) -> bool {
        self.checkout_step == CheckoutStep::Shipping && self.shipping_mode == ShippingMode::SelectAddress
    }

    /// Cycle to next region instantly
    pub async fn cycle_region(&mut self) {
        if self.regions.is_empty() {
            return;
        }
        let current_idx = self
            .regions
            .iter()
            .position(|r| r.id == self.region.id)
            .unwrap_or(0);
        let next_idx = (current_idx + 1) % self.regions.len();
        if let Some(region) = self.regions.get(next_idx) {
            self.change_region(region.clone()).await;
        }
    }

    /// Proceed to next checkout step (async for DB operations)
    pub async fn next_checkout_step(&mut self) {
        // Clear any previous notification
        self.notification = None;

        self.checkout_step = match self.checkout_step {
            CheckoutStep::Cart if !self.cart.is_empty() => {
                // Reset shipping mode to selection
                self.shipping_mode = ShippingMode::SelectAddress;
                self.address_select_index = 0;
                self.active_input = InputField::None;
                CheckoutStep::Shipping
            }
            CheckoutStep::Shipping => {
                match self.shipping_mode {
                    ShippingMode::SelectAddress => {
                        // This case is handled by select_address_option
                        return;
                    }
                    ShippingMode::AddNewAddress => {
                        // Validate shipping fields
                        if let Some(empty_field) = self.get_empty_shipping_field() {
                            self.notification = Some(format!("{} can't be empty", empty_field));
                            return;
                        }
                        // Save the address to Supabase
                        let _ = self.save_address_to_db().await;
                        self.active_input = InputField::None;
                        CheckoutStep::Payment
                    }
                }
            }
            CheckoutStep::Payment => {
                if self.payment_method == Some(PaymentMethod::Ssh) {
                    // Validate payment fields
                    if let Some(empty_field) = self.get_empty_payment_field() {
                        self.notification = Some(format!("{} can't be empty", empty_field));
                        return;
                    }
                    CheckoutStep::Confirmation
                } else if self.payment_method == Some(PaymentMethod::Browser) {
                    CheckoutStep::Confirmation
                } else {
                    CheckoutStep::Payment
                }
            }
            CheckoutStep::Confirmation => {
                // Order placed - reset
                self.cart.clear();
                self.checkout_step = CheckoutStep::Cart;
                self.current_tab = Tab::Home;
                CheckoutStep::Cart
            }
            _ => self.checkout_step,
        };
    }

    /// Get the first empty shipping field name, if any
    fn get_empty_shipping_field(&self) -> Option<&'static str> {
        if self.shipping_address.name.is_empty() {
            return Some("name");
        }
        if self.shipping_address.street_1.is_empty() {
            return Some("street");
        }
        if self.shipping_address.city.is_empty() {
            return Some("city");
        }
        if self.shipping_address.country.is_empty() {
            return Some("country");
        }
        if self.shipping_address.phone.is_empty() {
            return Some("phone");
        }
        if self.shipping_address.postal_code.is_empty() {
            return Some("postal code");
        }
        None
    }

    /// Get the first empty payment field name, if any
    fn get_empty_payment_field(&self) -> Option<&'static str> {
        if self.payment_info.name.is_empty() {
            return Some("name");
        }
        if self.payment_info.email.is_empty() {
            return Some("email");
        }
        if self.payment_info.card_number.is_empty() {
            return Some("card number");
        }
        if self.payment_info.expiry_month.is_empty() {
            return Some("expiry month");
        }
        if self.payment_info.expiry_year.is_empty() {
            return Some("expiry year");
        }
        if self.payment_info.cvv.is_empty() {
            return Some("cvv");
        }
        None
    }

    /// Clear the notification
    #[allow(dead_code)]
    pub fn clear_notification(&mut self) {
        self.notification = None;
    }

    /// Go back in checkout flow
    pub fn prev_checkout_step(&mut self) {
        // Clear notification when going back
        self.notification = None;

        self.checkout_step = match self.checkout_step {
            CheckoutStep::Cart => {
                self.current_tab = Tab::Shop;
                CheckoutStep::Cart
            }
            CheckoutStep::Shipping => {
                match self.shipping_mode {
                    ShippingMode::AddNewAddress => {
                        // Go back to address selection
                        self.shipping_mode = ShippingMode::SelectAddress;
                        self.active_input = InputField::None;
                        CheckoutStep::Shipping
                    }
                    ShippingMode::SelectAddress => {
                        // Go back to cart
                        self.active_input = InputField::None;
                        CheckoutStep::Cart
                    }
                }
            }
            CheckoutStep::Payment => {
                self.payment_method = None;
                self.shipping_mode = ShippingMode::SelectAddress;
                self.active_input = InputField::None;
                CheckoutStep::Shipping
            }
            CheckoutStep::Confirmation => {
                self.active_input = if self.payment_method == Some(PaymentMethod::Ssh) {
                    InputField::PaymentName
                } else {
                    InputField::None
                };
                CheckoutStep::Payment
            }
        };
    }

    /// Select payment method
    pub fn select_payment_method(&mut self) {
        self.payment_method = match self.payment_option_index {
            0 => {
                self.active_input = InputField::PaymentName;
                Some(PaymentMethod::Ssh)
            }
            _ => {
                self.active_input = InputField::None;
                Some(PaymentMethod::Browser)
            }
        };
    }

    pub fn quit(&mut self) {
        self.running = false;
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}
