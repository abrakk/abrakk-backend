-- Educational kits table

CREATE TABLE IF NOT EXISTS kits (
    id                   UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    title                TEXT NOT NULL,
    description          TEXT NOT NULL,
    subject              subject_area NOT NULL,
    difficulty           difficulty_level NOT NULL,
    age_min              INTEGER NOT NULL CHECK (age_min >= 0),
    age_max              INTEGER NOT NULL CHECK (age_max <= 18),
    language             TEXT NOT NULL DEFAULT 'en',
    learning_objectives  TEXT[] NOT NULL DEFAULT '{}',
    materials_required   TEXT[] NOT NULL DEFAULT '{}',
    author_id            UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    is_published         BOOLEAN NOT NULL DEFAULT false,
    download_count       BIGINT NOT NULL DEFAULT 0,
    created_at           TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at           TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    CONSTRAINT kits_age_range CHECK (age_min <= age_max)
);

-- Indices
CREATE INDEX idx_kits_author_id   ON kits (author_id);
CREATE INDEX idx_kits_subject     ON kits (subject);
CREATE INDEX idx_kits_difficulty  ON kits (difficulty);
CREATE INDEX idx_kits_language    ON kits (language);
CREATE INDEX idx_kits_is_published ON kits (is_published);
CREATE INDEX idx_kits_age_range   ON kits (age_min, age_max);

-- Full-text search index on title and description
CREATE INDEX idx_kits_fts ON kits
    USING GIN (to_tsvector('english', title || ' ' || description));

COMMENT ON TABLE kits IS 'Educational learning kits — the core resource on the platform';
