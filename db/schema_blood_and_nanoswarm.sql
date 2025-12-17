CREATE TABLE IF NOT EXISTS blood_type (
    id UUID PRIMARY KEY,
    abo_group VARCHAR(2) NOT NULL,
    rh_factor VARCHAR(3) NOT NULL,
    is_universal_donor BOOLEAN NOT NULL,
    is_universal_recipient BOOLEAN NOT NULL,
    loinc_code VARCHAR(16) NOT NULL,
    snomed_ct_code VARCHAR(32) NOT NULL,
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

CREATE TABLE IF NOT EXISTS nanoswarm_node (
    node_id UUID PRIMARY KEY,
    cohort_id UUID NOT NULL,
    ops_threshold_tops NUMERIC(10,2) NOT NULL,
    qpu_mesh_dim INTEGER NOT NULL,
    thermal_limit_w NUMERIC(6,3) NOT NULL,
    supply_voltage_v NUMERIC(4,3) NOT NULL,
    max_current_a NUMERIC(5,2) NOT NULL,
    bio_compat_array DOUBLE PRECISION[] NOT NULL,
    latency_profile_ms DOUBLE PRECISION[] NOT NULL,
    compliance_level VARCHAR(64) NOT NULL,
    ai_firmware_version VARCHAR(64) NOT NULL,
    topology_matrix BYTEA NOT NULL
);

INSERT INTO nanoswarm_node (
    node_id,
    cohort_id,
    ops_threshold_tops,
    qpu_mesh_dim,
    thermal_limit_w,
    supply_voltage_v,
    max_current_a,
    bio_compat_array,
    latency_profile_ms,
    compliance_level,
    ai_firmware_version,
    topology_matrix
) VALUES (
    'ffffffff-ffff-4fff-8fff-fffffffffff1',
    'dddddddd-dddd-4ddd-8ddd-ddddddddddd4',
    1000.00,
    32,
    0.450,
    2.300,
    18.00,
    ARRAY[0.98,0.97,0.96,0.95,0.97,0.96,0.95,0.94]::DOUBLE PRECISION[],
    ARRAY[22.5,21.8,19.6,18.9,23.2,20.4,19.1,17.7]::DOUBLE PRECISION[],
    'surgical-grade',
    'ALN-QPU-FW-2.3.7',
    decode('B4E2D7A1C9F0837AD1E4B6C3F9A2D5E1B7C8D9F0A3E6B1C4D7F2A9E0C5B8D3F6','hex')
)
ON CONFLICT (node_id) DO NOTHING;
