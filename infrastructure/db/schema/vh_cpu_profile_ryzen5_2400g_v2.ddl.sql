-- Core profile table
CREATE TABLE vh_cpu_profile (
    profile_id              VARCHAR(64) PRIMARY KEY,
    description             TEXT NOT NULL,
    version                 VARCHAR(16) NOT NULL,
    schema_version          VARCHAR(32) NOT NULL,
    owner                   VARCHAR(128) NOT NULL,
    created_utc             TIMESTAMP NOT NULL,
    last_modified_utc       TIMESTAMP NOT NULL,
    vendor                  VARCHAR(32) NOT NULL,
    family                  INT NOT NULL,
    model                   INT NOT NULL,
    stepping                INT NOT NULL,
    socket                  VARCHAR(32) NOT NULL,
    isa                     VARCHAR(32) NOT NULL,
    microarchitecture       VARCHAR(64) NOT NULL,
    process_nm              INT NOT NULL,
    tdp_watts_nominal       INT NOT NULL,
    max_memory_gib          INT NOT NULL,
    cores_visible           INT NOT NULL,
    threads_per_core        INT NOT NULL,
    logical_processors      INT NOT NULL,
    smt_enabled             BOOLEAN NOT NULL,
    sockets                 INT NOT NULL,
    dies_per_socket         INT NOT NULL,
    ccd_per_die             INT NOT NULL,
    cores_per_ccd           INT NOT NULL,
    numa_nodes              INT NOT NULL,
    base_clock_mhz          INT NOT NULL,
    max_boost_1c_mhz        INT NOT NULL,
    max_boost_allc_mhz      INT NOT NULL,
    ref_clock_mhz           INT NOT NULL,
    vcore_min_mv            INT NOT NULL,
    vcore_nominal_mv        INT NOT NULL,
    vcore_max_mv            INT NOT NULL,
    virtual_tdp_watts       INT NOT NULL,
    tj_max_c                INT NOT NULL,
    throttle_start_c        INT NOT NULL,
    throttle_floor_mhz      INT NOT NULL,
    mem_channels            INT NOT NULL,
    mem_max_capacity_mib    INT NOT NULL,
    mem_peak_bw_gibps       DECIMAL(8,2) NOT NULL,
    ecc_supported           BOOLEAN NOT NULL,
    host_overcommit_ratio   DECIMAL(4,2) NOT NULL,
    max_guest_vcpu_per_vm   INT NOT NULL,
    max_concurrent_vms_rec  INT NOT NULL,
    legacy_cores            INT NOT NULL,
    legacy_threads          INT NOT NULL,
    legacy_base_mhz         INT NOT NULL,
    legacy_l2_kib           INT NOT NULL,
    legacy_l3_kib           INT NOT NULL,
    new_l2_kib              INT NOT NULL,
    new_l3_kib              INT NOT NULL,
    min_st_perf_factor      DECIMAL(4,2) NOT NULL,
    min_mt_perf_factor      DECIMAL(4,2) NOT NULL
);

-- Tags
CREATE TABLE vh_cpu_profile_tag (
    profile_id  VARCHAR(64) NOT NULL,
    tag         VARCHAR(64) NOT NULL,
    PRIMARY KEY (profile_id, tag),
    FOREIGN KEY (profile_id) REFERENCES vh_cpu_profile(profile_id)
);

-- Preserved and extended features
CREATE TABLE vh_cpu_feature (
    profile_id      VARCHAR(64) NOT NULL,
    feature_name    VARCHAR(64) NOT NULL,
    category        VARCHAR(32) NOT NULL,  -- PRESERVED | EXTENDED
    PRIMARY KEY (profile_id, feature_name),
    FOREIGN KEY (profile_id) REFERENCES vh_cpu_profile(profile_id)
);

-- Cache hierarchy
CREATE TABLE vh_cpu_cache (
    profile_id      VARCHAR(64) NOT NULL,
    level           VARCHAR(8) NOT NULL,   -- L1I | L1D | L2 | L3
    total_size_kib  INT NOT NULL,
    per_core_kib    INT,
    shared          BOOLEAN NOT NULL,
    ways            INT NOT NULL,
    latency_cycles  INT NOT NULL,
    write_policy    VARCHAR(32) NOT NULL,
    inclusive       BOOLEAN,
    segment_count   INT,
    PRIMARY KEY (profile_id, level),
    FOREIGN KEY (profile_id) REFERENCES vh_cpu_profile(profile_id)
);

CREATE TABLE vh_cpu_cache_segment (
    profile_id      VARCHAR(64) NOT NULL,
    level           VARCHAR(8) NOT NULL,
    segment_id      INT NOT NULL,
    size_kib        INT NOT NULL,
    home_core_min   INT NOT NULL,
    home_core_max   INT NOT NULL,
    PRIMARY KEY (profile_id, level, segment_id),
    FOREIGN KEY (profile_id, level) REFERENCES vh_cpu_cache(profile_id, level)
);

-- Supported memory types
CREATE TABLE vh_cpu_memory_type (
    profile_id      VARCHAR(64) NOT NULL,
    mem_type        VARCHAR(32) NOT NULL,  -- e.g. DDR4-2933
    PRIMARY KEY (profile_id, mem_type),
    FOREIGN KEY (profile_id) REFERENCES vh_cpu_profile(profile_id)
);

-- Virtualization capabilities
CREATE TABLE vh_cpu_virtualization_cap (
    profile_id      VARCHAR(64) NOT NULL,
    capability      VARCHAR(64) NOT NULL,  -- e.g. AMD-V, AMD-Vi, NPT
    enabled         BOOLEAN NOT NULL,
    PRIMARY KEY (profile_id, capability),
    FOREIGN KEY (profile_id) REFERENCES vh_cpu_profile(profile_id)
);

-- Scheduling policies
CREATE TABLE vh_cpu_scheduling_policy (
    profile_id      VARCHAR(64) NOT NULL,
    band_name       VARCHAR(32) NOT NULL,  -- HIGH | NORMAL | BATCH
    timeslice_ms    INT NOT NULL,
    overcommit_max  DECIMAL(4,2) NOT NULL,
    PRIMARY KEY (profile_id, band_name),
    FOREIGN KEY (profile_id) REFERENCES vh_cpu_profile(profile_id)
);

-- Platform bindings
CREATE TABLE vh_cpu_platform_binding (
    profile_id      VARCHAR(64) NOT NULL,
    platform        VARCHAR(32) NOT NULL,  -- LINUX | WINDOWS | KUBERNETES | HELM | PROMETHEUS
    key             VARCHAR(64) NOT NULL,
    value           TEXT NOT NULL,
    PRIMARY KEY (profile_id, platform, key),
    FOREIGN KEY (profile_id) REFERENCES vh_cpu_profile(profile_id)
);

-- Telemetry series (Prometheus-style)
CREATE TABLE vh_cpu_telemetry_series (
    profile_id      VARCHAR(64) NOT NULL,
    metric_name     VARCHAR(128) NOT NULL,
    labels          VARCHAR(256) NOT NULL, -- comma-separated keys
    unit            VARCHAR(32) NOT NULL,
    PRIMARY KEY (profile_id, metric_name),
    FOREIGN KEY (profile_id) REFERENCES vh_cpu_profile(profile_id)
);
