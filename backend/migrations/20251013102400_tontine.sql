-- Enable UUID extension
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Table des utilisateurs
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    email VARCHAR(255) UNIQUE NOT NULL,
    phone VARCHAR(50) UNIQUE NOT NULL,
    full_name VARCHAR(255) NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    is_active BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Table des tontines
CREATE TABLE tontines (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name VARCHAR(255) NOT NULL,
    description TEXT,
    amount_per_member DECIMAL(15,2) NOT NULL,
    frequency VARCHAR(50) NOT NULL, -- 'weekly', 'monthly', 'daily'
    max_members INTEGER NOT NULL,
    current_round INTEGER DEFAULT 1,
    status VARCHAR(50) DEFAULT 'active', -- 'active', 'completed', 'cancelled'
    created_by UUID REFERENCES users(id),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Table des membres de tontine
CREATE TABLE tontine_members (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    tontine_id UUID REFERENCES tontines(id) ON DELETE CASCADE,
    user_id UUID REFERENCES users(id) ON DELETE CASCADE,
    join_date TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    is_active BOOLEAN DEFAULT TRUE,
    position_order INTEGER, -- Ordre de réception des fonds
    UNIQUE(tontine_id, user_id)
);

-- Table des tours de tontine
CREATE TABLE tontine_rounds (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    tontine_id UUID REFERENCES tontines(id) ON DELETE CASCADE,
    round_number INTEGER NOT NULL,
    beneficiary_user_id UUID REFERENCES users(id),
    amount DECIMAL(15,2) NOT NULL,
    round_date TIMESTAMP WITH TIME ZONE,
    status VARCHAR(50) DEFAULT 'pending', -- 'pending', 'completed', 'cancelled'
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(tontine_id, round_number)
);

-- Table des cotisations
CREATE TABLE contributions (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    tontine_round_id UUID REFERENCES tontine_rounds(id) ON DELETE CASCADE,
    member_id UUID REFERENCES tontine_members(id) ON DELETE CASCADE,
    amount DECIMAL(15,2) NOT NULL,
    payment_date TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    payment_method VARCHAR(100), -- 'cash', 'mobile_money', 'bank_transfer'
    payment_status VARCHAR(50) DEFAULT 'paid', -- 'paid', 'pending', 'failed'
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Table des transactions
CREATE TABLE transactions (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    tontine_id UUID REFERENCES tontines(id) ON DELETE CASCADE,
    from_user_id UUID REFERENCES users(id),
    to_user_id UUID REFERENCES users(id),
    amount DECIMAL(15,2) NOT NULL,
    transaction_type VARCHAR(100) NOT NULL, -- 'contribution', 'payout', 'refund'
    status VARCHAR(50) DEFAULT 'completed',
    description TEXT,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Table des transactions de paiement
CREATE TABLE payment_transactions (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    contribution_id UUID REFERENCES contributions(id),
    payment_reference VARCHAR(255) UNIQUE NOT NULL,
    provider VARCHAR(50) NOT NULL, -- 'MTNMobileMoney', 'OrangeMoney', etc.
    amount DECIMAL(15,2) NOT NULL,
    currency VARCHAR(10) DEFAULT 'XAF',
    status VARCHAR(50) DEFAULT 'pending',
    provider_reference VARCHAR(255),
    metadata JSONB,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Index pour les recherches par référence
CREATE INDEX idx_payment_transactions_reference ON payment_transactions(payment_reference);
CREATE INDEX idx_payment_transactions_contribution ON payment_transactions(contribution_id);
CREATE INDEX idx_tontine_members_tontine_id ON tontine_members(tontine_id);
CREATE INDEX idx_tontine_members_user_id ON tontine_members(user_id);
CREATE INDEX idx_tontine_rounds_tontine_id ON tontine_rounds(tontine_id);
CREATE INDEX idx_contributions_round_id ON contributions(tontine_round_id);
CREATE INDEX idx_contributions_member_id ON contributions(member_id);
CREATE INDEX idx_transactions_tontine_id ON transactions(tontine_id);

-- Fonction pour mettre à jour updated_at
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ language 'plpgsql';

-- Triggers pour mettre à jour automatiquement updated_at
CREATE TRIGGER update_users_updated_at BEFORE UPDATE ON users
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_tontines_updated_at BEFORE UPDATE ON tontines
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();