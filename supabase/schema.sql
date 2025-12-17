-- ANORA Labs Database Schema for Supabase
-- Run this in your Supabase SQL Editor to set up the database

-- Enable UUID extension
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- ============================================
-- REGIONS TABLE
-- ============================================
CREATE TABLE IF NOT EXISTS regions (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    code TEXT NOT NULL UNIQUE,
    flag TEXT NOT NULL DEFAULT '🌎',
    currency TEXT NOT NULL DEFAULT 'USD',
    free_shipping_threshold INTEGER NOT NULL DEFAULT 40,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Insert default regions
INSERT INTO regions (id, name, code, flag, currency, free_shipping_threshold) VALUES
    ('uz', 'Uzbekistan', 'UZ', '🇺🇿', 'USD', 40),
    ('global', 'Global', 'GLOBAL', '🌎', 'USD', 40)
ON CONFLICT (code) DO NOTHING;

-- ============================================
-- PRODUCTS TABLE
-- ============================================
CREATE TYPE product_category AS ENUM ('featured', 'originals');
CREATE TYPE roast_level AS ENUM ('light', 'medium', 'dark');
CREATE TYPE product_type AS ENUM ('subscription', 'one_time');

CREATE TABLE IF NOT EXISTS products (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name TEXT NOT NULL,
    slug TEXT NOT NULL UNIQUE,
    description TEXT NOT NULL,
    price_cents INTEGER NOT NULL,
    category product_category NOT NULL DEFAULT 'originals',
    roast_level roast_level,
    weight_oz INTEGER NOT NULL DEFAULT 12,
    bean_type TEXT NOT NULL DEFAULT 'whole beans',
    product_type product_type NOT NULL DEFAULT 'one_time',
    highlight_color TEXT NOT NULL DEFAULT '#ff24bd',
    region_id TEXT NOT NULL REFERENCES regions(id) ON DELETE CASCADE,
    in_stock BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Create index for faster region-based queries
CREATE INDEX IF NOT EXISTS idx_products_region ON products(region_id);
CREATE INDEX IF NOT EXISTS idx_products_category ON products(category);

-- Insert sample products
INSERT INTO products (name, slug, description, price_cents, category, roast_level, weight_oz, bean_type, product_type, highlight_color, region_id) VALUES
    (
        'cron',
        'cron',
        'Subscribe to Cron, the official ANORA Labs membership. Each month you''ll receive a scheduled delivery with a special flavor-of-the-month blend. You''ll also get receive additional gifts, exclusive offers, and invites to ANORA Labs events.',
        3000,
        'featured',
        NULL,
        12,
        'whole beans',
        'subscription',
        '#00a2c2',
        'uz'
    ),
    (
        '[object Object]',
        'object-object',
        'The interpolation of Caturra and Castillo varietals from Las Cochitas creates this refreshing citrusy and complex coffee.',
        2200,
        'originals',
        'light',
        12,
        'whole beans',
        'one_time',
        '#ffcd29',
        'uz'
    ),
    (
        'segfault',
        'segfault',
        'A bold and intense coffee that will crash your morning routine in the best way possible. Dark chocolate and smoky undertones.',
        2200,
        'originals',
        'medium',
        12,
        'whole beans',
        'one_time',
        '#0d99ff',
        'uz'
    ),
    (
        'dark mode',
        'dark-mode',
        'For those who prefer their coffee like their IDE theme. Deep, rich, and satisfying with hints of caramel.',
        2200,
        'originals',
        'dark',
        12,
        'whole beans',
        'one_time',
        '#14ae5c',
        'uz'
    ),
    (
        '404',
        '404',
        'A flavorful decaf coffee processed in the mountain waters of Brazil to create a dark chocolatey blend.',
        2200,
        'originals',
        'dark',
        12,
        'whole beans',
        'one_time',
        '#ab5998',
        'uz'
    )
ON CONFLICT (slug) DO NOTHING;

-- ============================================
-- USERS TABLE
-- ============================================
CREATE TABLE IF NOT EXISTS users (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    email TEXT NOT NULL UNIQUE,
    name TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- ============================================
-- ORDERS TABLE
-- ============================================
CREATE TYPE order_status AS ENUM ('pending', 'processing', 'shipped', 'delivered', 'cancelled');

CREATE TABLE IF NOT EXISTS orders (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID REFERENCES users(id) ON DELETE SET NULL,
    subtotal_cents INTEGER NOT NULL,
    shipping_cents INTEGER NOT NULL DEFAULT 0,
    total_cents INTEGER NOT NULL,
    status order_status NOT NULL DEFAULT 'pending',
    
    -- Shipping address (denormalized for historical accuracy)
    shipping_name TEXT NOT NULL,
    shipping_street TEXT NOT NULL,
    shipping_city TEXT NOT NULL,
    shipping_country TEXT NOT NULL,
    shipping_postal_code TEXT NOT NULL,
    shipping_phone TEXT,
    
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_orders_user ON orders(user_id);
CREATE INDEX IF NOT EXISTS idx_orders_status ON orders(status);

-- ============================================
-- ORDER ITEMS TABLE
-- ============================================
CREATE TABLE IF NOT EXISTS order_items (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    order_id UUID NOT NULL REFERENCES orders(id) ON DELETE CASCADE,
    product_id UUID NOT NULL REFERENCES products(id) ON DELETE SET NULL,
    product_name TEXT NOT NULL,  -- Denormalized for historical accuracy
    product_price_cents INTEGER NOT NULL,
    quantity INTEGER NOT NULL DEFAULT 1,
    total_cents INTEGER NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_order_items_order ON order_items(order_id);

-- ============================================
-- CART ITEMS TABLE (for persistent carts)
-- ============================================
CREATE TABLE IF NOT EXISTS cart_items (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    product_id UUID NOT NULL REFERENCES products(id) ON DELETE CASCADE,
    quantity INTEGER NOT NULL DEFAULT 1,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE(user_id, product_id)
);

CREATE INDEX IF NOT EXISTS idx_cart_items_user ON cart_items(user_id);

-- ============================================
-- SUBSCRIPTIONS TABLE
-- ============================================
CREATE TYPE subscription_status AS ENUM ('active', 'paused', 'cancelled');

CREATE TABLE IF NOT EXISTS subscriptions (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    product_id UUID NOT NULL REFERENCES products(id) ON DELETE CASCADE,
    status subscription_status NOT NULL DEFAULT 'active',
    next_delivery TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_subscriptions_user ON subscriptions(user_id);
CREATE INDEX IF NOT EXISTS idx_subscriptions_status ON subscriptions(status);

-- ============================================
-- ROW LEVEL SECURITY (RLS) POLICIES
-- ============================================

-- Enable RLS on all tables
ALTER TABLE regions ENABLE ROW LEVEL SECURITY;
ALTER TABLE products ENABLE ROW LEVEL SECURITY;
ALTER TABLE users ENABLE ROW LEVEL SECURITY;
ALTER TABLE orders ENABLE ROW LEVEL SECURITY;
ALTER TABLE order_items ENABLE ROW LEVEL SECURITY;
ALTER TABLE cart_items ENABLE ROW LEVEL SECURITY;
ALTER TABLE subscriptions ENABLE ROW LEVEL SECURITY;

-- Public read access for regions and products
CREATE POLICY "Public read access for regions" ON regions
    FOR SELECT USING (true);

CREATE POLICY "Public read access for products" ON products
    FOR SELECT USING (true);

-- Users can read their own data
CREATE POLICY "Users can read own profile" ON users
    FOR SELECT USING (auth.uid() = id);

CREATE POLICY "Users can update own profile" ON users
    FOR UPDATE USING (auth.uid() = id);

-- Orders policies
CREATE POLICY "Users can read own orders" ON orders
    FOR SELECT USING (auth.uid() = user_id);

CREATE POLICY "Users can create own orders" ON orders
    FOR INSERT WITH CHECK (auth.uid() = user_id);

-- Order items policies
CREATE POLICY "Users can read own order items" ON order_items
    FOR SELECT USING (
        EXISTS (
            SELECT 1 FROM orders 
            WHERE orders.id = order_items.order_id 
            AND orders.user_id = auth.uid()
        )
    );

-- Cart items policies
CREATE POLICY "Users can manage own cart" ON cart_items
    FOR ALL USING (auth.uid() = user_id);

-- Subscriptions policies
CREATE POLICY "Users can read own subscriptions" ON subscriptions
    FOR SELECT USING (auth.uid() = user_id);

CREATE POLICY "Users can create own subscriptions" ON subscriptions
    FOR INSERT WITH CHECK (auth.uid() = user_id);

CREATE POLICY "Users can update own subscriptions" ON subscriptions
    FOR UPDATE USING (auth.uid() = user_id);

-- ============================================
-- FUNCTIONS AND TRIGGERS
-- ============================================

-- Function to update updated_at timestamp
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ language 'plpgsql';

-- Apply trigger to tables with updated_at
CREATE TRIGGER update_regions_updated_at
    BEFORE UPDATE ON regions
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_products_updated_at
    BEFORE UPDATE ON products
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_users_updated_at
    BEFORE UPDATE ON users
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_orders_updated_at
    BEFORE UPDATE ON orders
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_cart_items_updated_at
    BEFORE UPDATE ON cart_items
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_subscriptions_updated_at
    BEFORE UPDATE ON subscriptions
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- ============================================
-- VIEWS
-- ============================================

-- View for products with region info
CREATE OR REPLACE VIEW products_with_region AS
SELECT 
    p.*,
    r.name as region_name,
    r.code as region_code,
    r.flag as region_flag
FROM products p
JOIN regions r ON p.region_id = r.id
WHERE p.in_stock = true;

-- View for order summaries
CREATE OR REPLACE VIEW order_summaries AS
SELECT 
    o.id,
    o.user_id,
    o.status,
    o.total_cents,
    o.created_at,
    COUNT(oi.id) as item_count,
    STRING_AGG(oi.product_name, ', ') as products
FROM orders o
LEFT JOIN order_items oi ON o.id = oi.order_id
GROUP BY o.id;

