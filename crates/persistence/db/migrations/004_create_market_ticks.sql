CREATE TABLE market_ticks (
    symbol TEXT NOT NULL,
    price DOUBLE PRECISION NOT NULL,
    volume DOUBLE PRECISION,
    timestamp TIMESTAMPTZ NOT NULL
);

SELECT create_hypertable('market_ticks', 'timestamp');

CREATE TABLE ai_signals (
    id UUID PRIMARY KEY,
    symbol TEXT NOT NULL,
    confidence DOUBLE PRECISION NOT NULL,
    action TEXT NOT NULL,
    reason TEXT NOT NULL,
    timestamp TIMESTAMPTZ NOT NULL
);

SELECT create_hypertable('ai_signals', 'timestamp');
