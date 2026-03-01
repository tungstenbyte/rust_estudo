-- Tabela do domínio segundominio (execute no banco se ainda não existir)
CREATE TABLE IF NOT EXISTS segundominio (
    id BIGSERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
