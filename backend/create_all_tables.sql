-- ==========================================
-- 完整建表脚本 - 在 DBeaver 中运行
-- 此脚本可安全地多次运行，只创建不存在的对象
-- ==========================================

-- Step 1: 创建RFC文档表
CREATE TABLE IF NOT EXISTS rfcs (
    id SERIAL PRIMARY KEY,
    rfc_number INTEGER UNIQUE NOT NULL,
    title TEXT NOT NULL,
    original_text TEXT,
    parsed_structure JSONB,
    status VARCHAR(50) NOT NULL DEFAULT 'draft',
    abstract TEXT,
    authors TEXT[],
    publish_date DATE,
    obsoletes INTEGER[],
    obsoleted_by INTEGER[],
    updates INTEGER[],
    updated_by INTEGER[],
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Step 2: 创建翻译内容表
CREATE TABLE IF NOT EXISTS translations (
    id SERIAL PRIMARY KEY,
    rfc_id INTEGER NOT NULL REFERENCES rfcs(id) ON DELETE CASCADE,
    section_id VARCHAR(50) NOT NULL,
    original_text TEXT NOT NULL,
    translated_text TEXT,
    reviewed BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    UNIQUE(rfc_id, section_id)
);

-- Step 3: 创建翻译任务表
CREATE TABLE IF NOT EXISTS translation_tasks (
    id UUID PRIMARY KEY,
    rfc_id INTEGER NOT NULL REFERENCES rfcs(id) ON DELETE CASCADE,
    status VARCHAR(50) NOT NULL DEFAULT 'pending',
    progress INTEGER DEFAULT 0,
    error_message TEXT,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Step 4: 创建术语库表
CREATE TABLE IF NOT EXISTS glossary (
    id SERIAL PRIMARY KEY,
    en_term VARCHAR(255) UNIQUE NOT NULL,
    zh_term VARCHAR(255) NOT NULL,
    context TEXT,
    frequency INTEGER DEFAULT 1,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Step 5: 创建索引（如果不存在）
CREATE INDEX IF NOT EXISTS idx_rfcs_rfc_number ON rfcs(rfc_number);
CREATE INDEX IF NOT EXISTS idx_rfcs_status ON rfcs(status);
CREATE INDEX IF NOT EXISTS idx_rfcs_title_gin ON rfcs USING gin(to_tsvector('english', title));
CREATE INDEX IF NOT EXISTS idx_translations_rfc_id ON translations(rfc_id);
CREATE INDEX IF NOT EXISTS idx_translations_section_id ON translations(section_id);
CREATE INDEX IF NOT EXISTS idx_translation_tasks_rfc_id ON translation_tasks(rfc_id);
CREATE INDEX IF NOT EXISTS idx_translation_tasks_status ON translation_tasks(status);
CREATE INDEX IF NOT EXISTS idx_glossary_en_term ON glossary(en_term);

-- Step 6: 创建更新时间戳触发器
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ language 'plpgsql';

-- 创建触发器（删除旧的再创建）
DROP TRIGGER IF EXISTS update_rfcs_updated_at ON rfcs;
CREATE TRIGGER update_rfcs_updated_at BEFORE UPDATE ON rfcs
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

DROP TRIGGER IF EXISTS update_translations_updated_at ON translations;
CREATE TRIGGER update_translations_updated_at BEFORE UPDATE ON translations
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

DROP TRIGGER IF EXISTS update_translation_tasks_updated_at ON translation_tasks;
CREATE TRIGGER update_translation_tasks_updated_at BEFORE UPDATE ON translation_tasks
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

DROP TRIGGER IF EXISTS update_glossary_updated_at ON glossary;
CREATE TRIGGER update_glossary_updated_at BEFORE UPDATE ON glossary
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- Step 7: 创建迁移记录表并标记迁移已完成
CREATE TABLE IF NOT EXISTS _sqlx_migrations (
    version BIGINT PRIMARY KEY,
    description TEXT NOT NULL,
    installed_on TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    success BOOLEAN NOT NULL,
    checksum BYTEA NOT NULL,
    execution_time BIGINT NOT NULL
);

-- 标记两个迁移已完成（这样 sqlx 不会再次运行它们）
INSERT INTO _sqlx_migrations (version, description, success, checksum, execution_time) VALUES
    (20260131000001, 'initial_schema', true, '\x00'::bytea, 0),
    (20260131000002, 'add_tags_remove_users', true, '\x00'::bytea, 0)
ON CONFLICT (version) DO NOTHING;

-- 完成！
SELECT 'Database initialized successfully!' as message;
