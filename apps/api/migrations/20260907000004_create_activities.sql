-- Activities table — individual tasks within a kit

CREATE TABLE IF NOT EXISTS activities (
    id                UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    kit_id            UUID NOT NULL REFERENCES kits(id) ON DELETE CASCADE,
    title             TEXT NOT NULL,
    description       TEXT NOT NULL,
    instructions      TEXT NOT NULL,
    duration_minutes  INTEGER NOT NULL CHECK (duration_minutes > 0),
    order_index       INTEGER NOT NULL DEFAULT 0,
    created_at        TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at        TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_activities_kit_id      ON activities (kit_id);
CREATE INDEX idx_activities_order_index ON activities (kit_id, order_index);

COMMENT ON TABLE activities IS 'Individual learning activities within a kit';
