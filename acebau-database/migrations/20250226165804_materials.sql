-- Add migration script here
CREATE TABLE IF NOT EXISTS materials (
    id UUID PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    supplier VARCHAR(255) NOT NULL,
    unit_price DECIMAL(10, 2) NOT NULL,
    bulk_price DECIMAL(10, 2) NOT NULL,
    vat SMALLINT NOT NULL,
    link VARCHAR(255) NOT NULL
);

CREATE INDEX IF NOT EXISTS materials_id ON materials(id);
