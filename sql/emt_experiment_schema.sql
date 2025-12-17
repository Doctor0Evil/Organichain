CREATE TABLE emt_envelope (
  envelopeid           BIGINT PRIMARY KEY,
  featureid            BIGINT NOT NULL,
  useridhash           BIGINT NOT NULL,
  timestampms          BIGINT NOT NULL,
  power_mw             REAL NOT NULL,
  eff_mass_kg          REAL NOT NULL,
  tissue_depth_mm      REAL NOT NULL,
  energy_density_mjmm3 REAL NOT NULL,
  duty_cycle_pct       REAL NOT NULL,
  actuation_freq_hz    REAL NOT NULL,
  dose_1s_mjmm3        REAL NOT NULL,
  dose_60s_mjmm3       REAL NOT NULL,
  dose_1h_mjmm3        REAL NOT NULL,
  limit_energy_mjmm3   REAL NOT NULL,
  limit_duty_pct       REAL NOT NULL,
  limit_freq_hz        REAL NOT NULL,
  limit_temp_delta_c   REAL NOT NULL,
  thermal_risk01       REAL NOT NULL,
  mechanical_risk01    REAL NOT NULL,
  neuro_risk01         REAL NOT NULL,
  policyprofileid      INTEGER NOT NULL,
  violation_flag       SMALLINT NOT NULL
);

CREATE TABLE emt_bench_sample (
  sampleid         BIGSERIAL PRIMARY KEY,
  envelopeid       BIGINT NOT NULL REFERENCES emt_envelope(envelopeid),
  node_id          BIGINT NOT NULL,
  timestampms      BIGINT NOT NULL,
  measured_temp_c  REAL NOT NULL,
  baseline_temp_c  REAL NOT NULL,
  measured_force_n REAL NOT NULL,
  supply_voltage_v REAL NOT NULL,
  supply_current_ma REAL NOT NULL
);
