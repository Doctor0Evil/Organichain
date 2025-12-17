-- Pharmaceutical-grade schema, PostgreSQL 17 + pgcrypto + pg_trgm + pgvector
-- Scope: blood typing, Bio-Aug biomech features, nanoswarm nodes, ALN policy hooks

CREATE EXTENSION IF NOT EXISTS pgcrypto;
CREATE EXTENSION IF NOT EXISTS pg_trgm;
CREATE EXTENSION IF NOT EXISTS vector;

-- 1. Blood typing master (matches your INSERT set, with clinical constraints)

CREATE TABLE IF NOT EXISTS blood_type (
    id                  UUID PRIMARY KEY,
    abo_group           CHAR(2)        NOT NULL CHECK (abo_group IN ('O','A','B','AB')),
    rh_factor           CHAR(3)        NOT NULL CHECK (rh_factor IN ('POS','NEG')),
    is_universal_donor  BOOLEAN        NOT NULL,
    is_universal_recipient BOOLEAN     NOT NULL,
    loinc_code          VARCHAR(32)    NOT NULL,
    snomed_ct_code      VARCHAR(32)    NOT NULL,
    created_at          TIMESTAMPTZ    NOT NULL DEFAULT NOW(),
    UNIQUE (abo_group, rh_factor)
);

INSERT INTO blood_type (id, abo_group, rh_factor, is_universal_donor, is_universal_recipient, loinc_code, snomed_ct_code)
VALUES
('11111111-1111-4111-8111-111111111111','O','NEG', TRUE,  FALSE,'77397-8','278148006'),
('22222222-2222-4222-8222-222222222222','O','POS', FALSE, FALSE,'77397-8','278147001'),
('33333333-3333-4333-8333-333333333333','A','NEG', FALSE, FALSE,'77397-8','278152006'),
('44444444-4444-4444-8444-444444444444','A','POS', FALSE, FALSE,'77397-8','278149003'),
('55555555-5555-4555-8555-555555555555','B','NEG', FALSE, FALSE,'77397-8','278153001'),
('66666666-6666-4666-8666-666666666666','B','POS', FALSE, FALSE,'77397-8','278150003'),
('77777777-7777-4777-8777-777777777777','AB','NEG',FALSE, FALSE,'77397-8','278154007'),
('88888888-8888-4888-8888-888888888888','AB','POS',FALSE, TRUE, '77397-8','278151004')
ON CONFLICT (abo_group, rh_factor) DO NOTHING;

-- 2. Clinical subject registry (de‑identified, Bio-Aug compatible)

CREATE TABLE IF NOT EXISTS subject_registry (
    subject_id          UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    subject_hash        BYTEA           NOT NULL UNIQUE, -- 64‑bit hash from AUUser.useridhash
    cohort_id           INTEGER         NOT NULL,
    birth_year          SMALLINT        NOT NULL CHECK (birth_year BETWEEN 1900 AND 2100),
    sex_code            SMALLINT        NOT NULL CHECK (sex_code BETWEEN 0 AND 3),
    region_code_iso3166 SMALLINT        NOT NULL,
    augmentation_level  SMALLINT        NOT NULL CHECK (augmentation_level BETWEEN 0 AND 3),
    blood_type_id       UUID            NOT NULL REFERENCES blood_type(id),
    created_at          TIMESTAMPTZ     NOT NULL DEFAULT NOW(),
    updated_at          TIMESTAMPTZ     NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_subject_registry_hash
    ON subject_registry USING HASH (subject_hash);

-- 3. Biomechanical feature layer (aligned to AU.BioAug.SecureBiomech.v1 BiomechFeature)[file:2]

CREATE TYPE feature_class AS ENUM (
    'MUSCULO_SKELETAL',
    'NEURO_INTERFACE',
    'CARDIOVASCULAR',
    'SENSORY_AUGMENT',
    'ENDOCRINE_MOD',
    'METABOLIC_SUPPORT',
    'MIXED_SYSTEM'
);

CREATE TYPE control_mode AS ENUM (
    'SENSING_ONLY',
    'BOUNDED_ACTUATION',
    'FULL_BIDIRECTIONAL'
);

CREATE TYPE safety_tier AS ENUM (
    'TIER0_PASSIVE',
    'TIER1_CONSTRAINED',
    'TIER2_CRITICAL'
);

CREATE TABLE IF NOT EXISTS biomech_feature (
    feature_id              BIGSERIAL PRIMARY KEY,
    subject_id              UUID        NOT NULL REFERENCES subject_registry(subject_id),
    feature_class           feature_class NOT NULL,
    control_mode            control_mode  NOT NULL,
    safety_tier             safety_tier   NOT NULL,
    connector_code          SMALLINT    NOT NULL,
    channels_total          SMALLINT    NOT NULL CHECK (channels_total > 0),
    sense_channels          SMALLINT    NOT NULL CHECK (sense_channels >= 0),
    actuate_channels        SMALLINT    NOT NULL CHECK (actuate_channels >= 0),
    sense_voltage_max_mv    SMALLINT    NOT NULL CHECK (sense_voltage_max_mv BETWEEN 0 AND 5000),
    actuate_voltage_max_mv  SMALLINT    NOT NULL CHECK (actuate_voltage_max_mv BETWEEN 0 AND 20000),
    isolation_min_megaohm   REAL        NOT NULL CHECK (isolation_min_megaohm > 0.0),
    sample_rate_hz          REAL        NOT NULL CHECK (sample_rate_hz > 0.0),
    latency_budget_ms       REAL        NOT NULL CHECK (latency_budget_ms > 0.0),
    jitter_max_us           REAL        NOT NULL CHECK (jitter_max_us > 0.0),
    body_region_code        SMALLINT    NOT NULL,
    tissue_depth_mm         REAL        NOT NULL CHECK (tissue_depth_mm >= 0.0),
    biocompat_score_01      REAL        NOT NULL CHECK (biocompat_score_01 BETWEEN 0.0 AND 1.0),
    aln_semantic_ptr        BIGINT      NOT NULL DEFAULT 0,
    created_at              TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at              TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_biomech_feature_subject
    ON biomech_feature (subject_id);

CREATE INDEX IF NOT EXISTS idx_biomech_feature_class_safety
    ON biomech_feature (feature_class, safety_tier);

-- 4. BCI / AI linkage (aligned to BCILink)[file:2]

CREATE TYPE ai_link_mode AS ENUM (
    'NO_AI',
    'CLOUD_ASSISTED',
    'EDGE_MODEL',
    'ON_DEVICE_NEUROMORPHIC'
);

CREATE TYPE neuromorphic_class AS ENUM (
    'NONE',
    'ANALOG_SPIKING_ARRAY',
    'DIGITAL_SNN',
    'MIXED_SIGNAL'
);

CREATE TABLE IF NOT EXISTS bci_link (
    link_id             BIGSERIAL PRIMARY KEY,
    feature_id          BIGINT      NOT NULL REFERENCES biomech_feature(feature_id),
    subject_id          UUID        NOT NULL REFERENCES subject_registry(subject_id),
    ai_link_mode        ai_link_mode       NOT NULL,
    neuromorphic_class  neuromorphic_class NOT NULL,
    model_version_hash  BYTEA       NOT NULL,      -- 32‑bit hash
    max_inference_hz    REAL        NOT NULL CHECK (max_inference_hz > 0.0),
    max_payload_bytes   INTEGER     NOT NULL CHECK (max_payload_bytes > 0),
    max_actuation_delta REAL        NOT NULL CHECK (max_actuation_delta BETWEEN 0.0 AND 1.0),
    has_human_in_loop   BOOLEAN     NOT NULL,
    has_explicit_consent BOOLEAN    NOT NULL,
    created_at          TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_bci_link_feature_mode
    ON bci_link (feature_id, ai_link_mode);

-- 5. Security and safety snapshots (aligned to SecuritySnapshot)[file:2][web:8]

CREATE TYPE security_state AS ENUM (
    'SECURE',
    'DEGRADED',
    'LOCKED_DOWN',
    'QUARANTINED'
);

CREATE TYPE threat_flag AS ENUM (
    'NONE',
    'SUSPECTED_MALWARE',
    'NETWORK_ANOMALY',
    'POLICY_VIOLATION',
    'INTEGRITY_BREACH'
);

CREATE TABLE IF NOT EXISTS security_snapshot (
    snapshot_id          BIGSERIAL PRIMARY KEY,
    feature_id           BIGINT      NOT NULL REFERENCES biomech_feature(feature_id),
    subject_id           UUID        NOT NULL REFERENCES subject_registry(subject_id),
    timestamp_ms         BIGINT      NOT NULL,
    security_state       security_state NOT NULL,
    threat_flag          threat_flag     NOT NULL,
    pqc_enabled          BOOLEAN     NOT NULL,
    hw_root_of_trust     BOOLEAN     NOT NULL,
    secure_boot          BOOLEAN     NOT NULL,
    isolation_enforced   BOOLEAN     NOT NULL,
    anomaly_score_01     REAL        NOT NULL CHECK (anomaly_score_01 BETWEEN 0.0 AND 1.0),
    attack_surface_score REAL        NOT NULL CHECK (attack_surface_score >= 0.0),
    last_update_delta_ms INTEGER     NOT NULL CHECK (last_update_delta_ms >= 0),
    created_at           TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_security_snapshot_feature_time
    ON security_snapshot (feature_id, timestamp_ms DESC);

CREATE INDEX IF NOT EXISTS idx_security_snapshot_state_flag
    ON security_snapshot (security_state, threat_flag);

-- 6. Rights and audit layer (aligned to RightsAssertion + AuditEvent)[file:2][web:8]

CREATE TYPE rights_flag AS ENUM (
    'RIGHT_EXISTENCE',
    'RIGHT_PRIVACY',
    'RIGHT_CONSENT',
    'RIGHT_SELF_OWNERSHIP',
    'RIGHT_META_COGNITIVE',
    'RIGHT_AUDIT'
);

CREATE TABLE IF NOT EXISTS rights_assertion (
    assertion_id     BIGSERIAL PRIMARY KEY,
    subject_id       UUID        NOT NULL REFERENCES subject_registry(subject_id),
    feature_id       BIGINT      NOT NULL REFERENCES biomech_feature(feature_id),
    timestamp_ms     BIGINT      NOT NULL,
    rights_flag      rights_flag NOT NULL,
    granted          BOOLEAN     NOT NULL,
    revoked          BOOLEAN     NOT NULL,
    created_at       TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_rights_assertion_subject_flag
    ON rights_assertion (subject_id, rights_flag);

CREATE TABLE IF NOT EXISTS audit_event (
    event_id         BIGSERIAL PRIMARY KEY,
    subject_id       UUID        NOT NULL REFERENCES subject_registry(subject_id),
    feature_id       BIGINT      NOT NULL REFERENCES biomech_feature(feature_id),
    timestamp_ms     BIGINT      NOT NULL,
    event_code       INTEGER     NOT NULL,
    severity_0_5     SMALLINT    NOT NULL CHECK (severity_0_5 BETWEEN 0 AND 5),
    config_hash      BYTEA       NOT NULL,
    sw_bundle_hash   BYTEA       NOT NULL,
    detail_offset    BIGINT      NOT NULL,    -- pointer into ALN metadata file
    created_at       TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_audit_event_feature_time
    ON audit_event (feature_id, timestamp_ms DESC);

-- 7. Nanoswarm node registry and clinical binding (ops_threshold etc.)

CREATE TABLE IF NOT EXISTS nano_node (
    node_id             UUID PRIMARY KEY,
    subject_id          UUID        NOT NULL REFERENCES subject_registry(subject_id),
    ops_threshold_tops  NUMERIC(12,2) NOT NULL CHECK (ops_threshold_tops > 0),
    qpu_topology_matrix DOUBLE PRECISION[] NOT NULL,  -- ALN-aligned mesh/fabric topology[file:1]
    compliance_level    TEXT        NOT NULL,        -- e.g. 'surgical-grade'
    ai_firmware_version TEXT        NOT NULL,
    thermal_ceiling_w   REAL        NOT NULL CHECK (thermal_ceiling_w > 0.0),
    power_floor_w       REAL        NOT NULL CHECK (power_floor_w >= 0.0),
    duty_cycle_max_pct  REAL        NOT NULL CHECK (duty_cycle_max_pct BETWEEN 0.0 AND 100.0),
    reliability_vector  DOUBLE PRECISION[] NOT NULL, -- per‑axis reliability
    latency_vector_ms   DOUBLE PRECISION[] NOT NULL,
    firmware_hash       BYTEA       NOT NULL,
    created_at          TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at          TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Example node instance derived from your values

INSERT INTO nano_node (
    node_id,
    subject_id,
    ops_threshold_tops,
    qpu_topology_matrix,
    compliance_level,
    ai_firmware_version,
    thermal_ceiling_w,
    power_floor_w,
    duty_cycle_max_pct,
    reliability_vector,
    latency_vector_ms,
    firmware_hash
) VALUES (
    'ffffffff-ffff-4fff-8fff-fffffffffff1',
    'dddddddd-dddd-4ddd-8ddd-ddddddddddd4',
    1000.00,
    ARRAY[[1.0,0.0],[0.0,1.0]]::DOUBLE PRECISION[],
    'surgical-grade',
    'ALN-QPU-BIOMECH-1.4.2',
    0.450,
    2.300,
    18.00,
    ARRAY[0.98,0.97,0.96,0.95,0.97,0.96,0.95,0.94]::DOUBLE PRECISION[],
    ARRAY[22.5,21.8,19.6,18.9,23.2,20.4,19.1,17.7]::DOUBLE PRECISION[],
    decode('B4E2D7A1C9F0837AD1E4B6C3F9A2D5E1B7C8D9F0A3E6B1C4D7F2A9E0C5B8D3F6','hex')
);

-- 8. IEC 62304 / ISO 14971 traceability hooks for neuromodulation safety[web:3][web:8][web:10]

CREATE TABLE IF NOT EXISTS safety_profile (
    profile_id          TEXT PRIMARY KEY, -- e.g. 'BioAugClinical'
    iec_62304_class     CHAR(1) NOT NULL CHECK (iec_62304_class IN ('A','B','C')),
    iso_14971_profile   TEXT    NOT NULL,
    created_at          TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS hazard_registry (
    hazard_id           TEXT PRIMARY KEY,
    profile_id          TEXT NOT NULL REFERENCES safety_profile(profile_id),
    description         TEXT NOT NULL,
    severity_code       TEXT NOT NULL,
    probability_code    TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS control_registry (
    control_id          TEXT PRIMARY KEY,
    hazard_id           TEXT NOT NULL REFERENCES hazard_registry(hazard_id),
    control_type        TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS policy_binding (
    policy_id           TEXT PRIMARY KEY,
    hazard_id           TEXT NOT NULL REFERENCES hazard_registry(hazard_id),
    control_id          TEXT NOT NULL REFERENCES control_registry(control_id),
    backend_module      TEXT NOT NULL,
    backend_test_id     TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS trace_link (
    trace_id            BIGSERIAL PRIMARY KEY,
    hazard_id           TEXT NOT NULL REFERENCES hazard_registry(hazard_id),
    control_id          TEXT NOT NULL REFERENCES control_registry(control_id),
    policy_id           TEXT NOT NULL REFERENCES policy_binding(policy_id),
    backend_module      TEXT NOT NULL,
    created_at          TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
