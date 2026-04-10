-- ═══════════════════════════════════════════════════════════════════════════════
-- SENTIENT CORE - PostgreSQL Initialization Script
-- ═══════════════════════════════════════════════════════════════════════════════
-- Bu script docker-compose ile PostgreSQL container başlatıldığında otomatik çalışır.

-- Sentinel veritabanı
CREATE DATABASE sentient;

-- Agent state tablosu
CREATE TABLE IF NOT EXISTS agent_state (
    id SERIAL PRIMARY KEY,
    agent_id VARCHAR(255) NOT NULL UNIQUE,
    state JSONB NOT NULL DEFAULT '{}',
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Memory tablosu
CREATE TABLE IF NOT EXISTS memory (
    id SERIAL PRIMARY KEY,
    session_id VARCHAR(255) NOT NULL,
    role VARCHAR(50) NOT NULL,
    content TEXT NOT NULL,
    embedding VECTOR(1536),
    metadata JSONB DEFAULT '{}',
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Sessions tablosu
CREATE TABLE IF NOT EXISTS sessions (
    id SERIAL PRIMARY KEY,
    session_id VARCHAR(255) NOT NULL UNIQUE,
    user_id VARCHAR(255),
    status VARCHAR(50) DEFAULT 'active',
    metadata JSONB DEFAULT '{}',
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Skills tablosu
CREATE TABLE IF NOT EXISTS skills (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL UNIQUE,
    description TEXT,
    schema JSONB NOT NULL,
    enabled BOOLEAN DEFAULT true,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Audit log tablosu
CREATE TABLE IF NOT EXISTS audit_log (
    id SERIAL PRIMARY KEY,
    action VARCHAR(100) NOT NULL,
    entity_type VARCHAR(100),
    entity_id VARCHAR(255),
    user_id VARCHAR(255),
    details JSONB DEFAULT '{}',
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- İndeksler
CREATE INDEX IF NOT EXISTS idx_agent_state_agent_id ON agent_state(agent_id);
CREATE INDEX IF NOT EXISTS idx_memory_session_id ON memory(session_id);
CREATE INDEX IF NOT EXISTS idx_sessions_user_id ON sessions(user_id);
CREATE INDEX IF NOT EXISTS idx_audit_log_created_at ON audit_log(created_at);

-- Varsayılan yetenekler
INSERT INTO skills (name, description, schema) VALUES
    ('code_generation', 'Kod üretme ve düzenleme', '{"type": "object", "properties": {"language": {"type": "string"}, "task": {"type": "string"}}}'),
    ('web_search', 'Web araması yapma', '{"type": "object", "properties": {"query": {"type": "string"}}}'),
    ('file_operations', 'Dosya okuma ve yazma', '{"type": "object", "properties": {"path": {"type": "string"}, "operation": {"type": "string"}}}')
ON CONFLICT (name) DO NOTHING;

-- ═══════════════════════════════════════════════════════════════════════════════
-- pgvector uzantısı (eğer yüklü ise)
-- ═══════════════════════════════════════════════════════════════════════════════
CREATE EXTENSION IF NOT EXISTS vector;

-- Vector indeksi
CREATE INDEX IF NOT EXISTS idx_memory_embedding ON memory 
USING ivfflat (embedding vector_cosine_ops)
WITH (lists = 100);

-- Başarı mesajı
DO $$
BEGIN
    RAISE NOTICE 'SENTIENT Core database initialized successfully';
END $$;
