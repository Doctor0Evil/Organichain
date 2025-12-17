CREATE TABLE integration_node (
    node_id            BYTEA PRIMARY KEY,
    ops_threshold_tops NUMERIC(10,3) NOT NULL CHECK (ops_threshold_tops >= 0),
    topology_rows      SMALLINT      NOT NULL CHECK (topology_rows > 0),
    topology_cols      SMALLINT      NOT NULL CHECK (topology_cols > 0),
    compliance_level   SMALLINT      NOT NULL CHECK (compliance_level BETWEEN 0 AND 2),
    ai_firmware_version INTEGER      NOT NULL,
    node_class         SMALLINT      NOT NULL CHECK (node_class BETWEEN 1 AND 5)
);

CREATE TABLE bioaug_mt6883_binding (
    binding_id           BIGSERIAL PRIMARY KEY,
    useridhash           BYTEA      NOT NULL,
    featureid            BIGINT     NOT NULL,
    mt6883_plugin_name   TEXT       NOT NULL,
    mt6883_plugin_namehash BYTEA    NOT NULL,
    node_id              BYTEA      NOT NULL REFERENCES integration_node(node_id),
    rights_profilehash   BYTEA      NOT NULL,
    clinical_profilehash BYTEA      NOT NULL,
    CONSTRAINT chk_plugin_namehash_len CHECK (octet_length(mt6883_plugin_namehash) = 4)
);

CREATE TABLE nano_bci_envelope (
    featureid        BIGINT PRIMARY KEY,
    stim_type        SMALLINT NOT NULL CHECK (stim_type BETWEEN 1 AND 5),
    resonancefreqhz  NUMERIC(8,2) NOT NULL CHECK (resonancefreqhz BETWEEN 1 AND 5000),
    amplitudemt      NUMERIC(6,2) NOT NULL CHECK (amplitudemt <= 10.00),
    dutycycle        NUMERIC(4,3) NOT NULL CHECK (dutycycle BETWEEN 0 AND 1),
    energyintegral   NUMERIC(10,4) NOT NULL
);
