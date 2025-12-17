INSERT INTO vh_cpu_profile (
    profile_id, description, version, schema_version, owner,
    created_utc, last_modified_utc, vendor, family, model, stepping,
    socket, isa, microarchitecture, process_nm, tdp_watts_nominal,
    max_memory_gib, cores_visible, threads_per_core, logical_processors,
    smt_enabled, sockets, dies_per_socket, ccd_per_die, cores_per_ccd,
    numa_nodes, base_clock_mhz, max_boost_1c_mhz, max_boost_allc_mhz,
    ref_clock_mhz, vcore_min_mv, vcore_nominal_mv, vcore_max_mv,
    virtual_tdp_watts, tj_max_c, throttle_start_c, throttle_floor_mhz,
    mem_channels, mem_max_capacity_mib, mem_peak_bw_gibps, ecc_supported,
    host_overcommit_ratio, max_guest_vcpu_per_vm, max_concurrent_vms_rec,
    legacy_cores, legacy_threads, legacy_base_mhz,
    legacy_l2_kib, legacy_l3_kib, new_l2_kib, new_l3_kib,
    min_st_perf_factor, min_mt_perf_factor
) VALUES (
    'VH-R5-2400G-V2',
    'Upgraded virtual CPU based on AMD Ryzen 5 2400G-class core (Raven Ridge), 8C/16T, extended cache, high-fidelity VM simulation.',
    '2.0.0',
    'ALN-VirtualHardware-1.3',
    'Virtual-Hardwire.CyberneticStack',
    '2025-12-12 08:50:00',
    '2025-12-12 08:50:00',
    'AuthenticAMD',
    23,
    17,
    0,
    'AM4',
    'x86-64',
    'Zen-RavenRidge',
    14,
    65,
    64,
    8,
    2,
    16,
    TRUE,
    1,
    1,
    1,
    8,
    1,
    4000,
    4400,
    4200,
    100,
    800,
    1150,
    1350,
    95,
    95,
    90,
    3600,
    2,
    65536,
    44.00,
    TRUE,
    3.00,
    16,
    32,
    4,
    8,
    3600,
    2048,
    4096,
    4096,
    8192,
    1.10,
    2.00
);

-- Tags
INSERT INTO vh_cpu_profile_tag (profile_id, tag) VALUES
('VH-R5-2400G-V2','x86-64'),
('VH-R5-2400G-V2','AMD-Zen'),
('VH-R5-2400G-V2','Raven-Ridge'),
('VH-R5-2400G-V2','virtual-cpu'),
('VH-R5-2400G-V2','kvm'),
('VH-R5-2400G-V2','hyper-v'),
('VH-R5-2400G-V2','kubernetes'),
('VH-R5-2400G-V2','prometheus');

-- Preserved features
INSERT INTO vh_cpu_feature (profile_id, feature_name, category) VALUES
('VH-R5-2400G-V2','x86-64','PRESERVED'),
('VH-R5-2400G-V2','sse','PRESERVED'),
('VH-R5-2400G-V2','sse2','PRESERVED'),
('VH-R5-2400G-V2','sse3','PRESERVED'),
('VH-R5-2400G-V2','ssse3','PRESERVED'),
('VH-R5-2400G-V2','sse4_1','PRESERVED'),
('VH-R5-2400G-V2','sse4_2','PRESERVED'),
('VH-R5-2400G-V2','avx','PRESERVED'),
('VH-R5-2400G-V2','avx2','PRESERVED'),
('VH-R5-2400G-V2','aes','PRESERVED'),
('VH-R5-2400G-V2','fma3','PRESERVED');

-- Extended features
INSERT INTO vh_cpu_feature (profile_id, feature_name, category) VALUES
('VH-R5-2400G-V2','amd-v','EXTENDED'),
('VH-R5-2400G-V2','amd-vi','EXTENDED'),
('VH-R5-2400G-V2','smep','EXTENDED'),
('VH-R5-2400G-V2','smap','EXTENDED'),
('VH-R5-2400G-V2','nx-bit','EXTENDED'),
('VH-R5-2400G-V2','1gb-pages','EXTENDED');

-- Caches
INSERT INTO vh_cpu_cache
(profile_id, level, total_size_kib, per_core_kib, shared, ways, latency_cycles, write_policy, inclusive, segment_count)
VALUES
('VH-R5-2400G-V2','L1I',512,64,FALSE,4,4,'NA',NULL,NULL),
('VH-R5-2400G-V2','L1D',256,32,FALSE,8,4,'WriteBack',NULL,NULL),
('VH-R5-2400G-V2','L2',4096,512,FALSE,8,12,'WriteBack',FALSE,NULL),
('VH-R5-2400G-V2','L3',8192,NULL,TRUE,16,35,'WriteBack',FALSE,2);

INSERT INTO vh_cpu_cache_segment
(profile_id, level, segment_id, size_kib, home_core_min, home_core_max) VALUES
('VH-R5-2400G-V2','L3',0,4096,0,3),
('VH-R5-2400G-V2','L3',1,4096,4,7);

-- Memory types
INSERT INTO vh_cpu_memory_type (profile_id, mem_type) VALUES
('VH-R5-2400G-V2','DDR4-2400'),
('VH-R5-2400G-V2','DDR4-2666'),
('VH-R5-2400G-V2','DDR4-2933');

-- Virtualization caps
INSERT INTO vh_cpu_virtualization_cap (profile_id, capability, enabled) VALUES
('VH-R5-2400G-V2','AMD-V',TRUE),
('VH-R5-2400G-V2','AMD-Vi',TRUE),
('VH-R5-2400G-V2','NPT',TRUE),
('VH-R5-2400G-V2','NestedVirtualization',TRUE),
('VH-R5-2400G-V2','PostedInterrupts',TRUE);

-- Scheduling bands
INSERT INTO vh_cpu_scheduling_policy
(profile_id, band_name, timeslice_ms, overcommit_max) VALUES
('VH-R5-2400G-V2','HIGH',1,1.00),
('VH-R5-2400G-V2','NORMAL',4,2.00),
('VH-R5-2400G-V2','BATCH',8,3.00);

-- Platform bindings examples
INSERT INTO vh_cpu_platform_binding (profile_id, platform, key, value) VALUES
('VH-R5-2400G-V2','LINUX','qemu_cpu_model','EPYC-v4-vhr5-2400g-v2'),
('VH-R5-2400G-V2','LINUX','kernel_cmdline','amd_iommu=on kvm_amd.npt=1 kvm_amd.avic=1'),
('VH-R5-2400G-V2','WINDOWS','hypervisor','Hyper-V'),
('VH-R5-2400G-V2','WINDOWS','min_os_version','10.0.19044'),
('VH-R5-2400G-V2','KUBERNETES','node_label','vh.cpu/model=vh-r5-2400g-v2'),
('VH-R5-2400G-V2','KUBERNETES','cpu_manager_policy','static'),
('VH-R5-2400G-V2','PROMETHEUS','scrape_interval_seconds','5'),
('VH-R5-2400G-V2','PROMETHEUS','endpoint_path','/metrics');

-- Telemetry series (Prometheus-style)
INSERT INTO vh_cpu_telemetry_series
(profile_id, metric_name, labels, unit) VALUES
('VH-R5-2400G-V2','vh_cpu_core_utilization','core_id,socket_id','percent'),
('VH-R5-2400G-V2','vh_cpu_freq_effective_mhz','core_id','MHz'),
('VH-R5-2400G-V2','vh_cpu_vm_vcpu_runtime_ms','vm_id,vcpu_id','milliseconds');
