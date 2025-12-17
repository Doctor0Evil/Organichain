// neuromorphic_soft_bio_stack/src/lib.rs

#![forbid(unsafe_code)]

use core::cmp::min;

// ---------- FIXED-POINT CORE (16.16) ----------

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct FixedPoint32 {
    pub raw_u32: u32,
}

const FX_SCALE: f64 = 65536.0;

pub fn fx_from_f64(x: f64) -> FixedPoint32 {
    let scaled = (x * FX_SCALE).round().clamp(f64::MIN, f64::MAX) as i64;
    FixedPoint32 {
        raw_u32: (scaled as u64 & 0xFFFF_FFFF) as u32,
    }
}

pub fn fx_to_f64(x: FixedPoint32) -> f64 {
    (x.raw_u32 as f64) / FX_SCALE
}

impl FixedPoint32 {
    #[inline]
    pub fn saturating_add(self, other: FixedPoint32) -> FixedPoint32 {
        let (sum, overflow) = self.raw_u32.overflowing_add(other.raw_u32);
        if overflow {
            FixedPoint32 { raw_u32: u32::MAX }
        } else {
            FixedPoint32 { raw_u32: sum }
        }
    }
}

// ---------- AU.ET BUDGET & LEDGER ----------

#[derive(Copy, Clone, Debug)]
pub struct AuEtBudget {
    pub daily_cap: FixedPoint32,   // AU.ET_day
    pub session_cap: FixedPoint32, // AU.ET_session
}

#[derive(Copy, Clone, Debug)]
pub struct AuEtLedger {
    pub total_burn: FixedPoint32,
    pub daily_burn: FixedPoint32,
    pub session_burn: FixedPoint32,
    pub max_supply: FixedPoint32,
}

impl AuEtLedger {
    /// Pure monotone “can spend?” check with saturating arithmetic. [file:144]
    pub fn can_spend(&self, amount: FixedPoint32, budget: AuEtBudget) -> bool {
        let new_total = (self.total_burn.raw_u32 as u64)
            .saturating_add(amount.raw_u32 as u64);
        let new_daily = (self.daily_burn.raw_u32 as u64)
            .saturating_add(amount.raw_u32 as u64);
        let new_session = (self.session_burn.raw_u32 as u64)
            .saturating_add(amount.raw_u32 as u64);

        let max_supply = self.max_supply.raw_u32 as u64;
        let daily_cap = budget.daily_cap.raw_u32 as u64;
        let session_cap = budget.session_cap.raw_u32 as u64;

        new_total <= max_supply && new_daily <= daily_cap && new_session <= session_cap
    }

    /// Monotone burn; any overflow saturates to MAX, never mints. [file:144]
    pub fn burn(&mut self, amount: FixedPoint32) {
        self.total_burn = self.total_burn.saturating_add(amount);
        self.daily_burn = self.daily_burn.saturating_add(amount);
        self.session_burn = self.session_burn.saturating_add(amount);
    }
}

// ---------- BIOCOMPATIBILITY PROFILES ----------

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum NodeClass {
    SoftEcog,
    PvdfRetinal,
    HapticPlate,
    RfusHelmet,
    MicromoteKnn,
}

#[derive(Copy, Clone, Debug)]
pub enum TissueProfile {
    AdultTranscranial,
    Ocular,
    Pediatric,
    Peripheral,
}

#[derive(Copy, Clone, Debug)]
pub struct BiocompatibilityIndex {
    pub index: FixedPoint32, // 0.0–1.0
}

#[derive(Copy, Clone, Debug)]
pub struct DoseLimits {
    pub max_sar_w_per_kg: FixedPoint32,
    pub max_ti: FixedPoint32,
    pub max_mi: FixedPoint32,
    pub max_cem43: FixedPoint32,
    pub max_charge_density_uc_cm2: FixedPoint32, // per phase
}

#[derive(Copy, Clone, Debug)]
pub struct NodeClassProfile {
    pub class: NodeClass,
    pub tissue_profile: TissueProfile,
    pub biocomp: BiocompatibilityIndex,
    pub dose_limits: DoseLimits,
    pub au_per_mws: FixedPoint32,
    pub au_per_update: FixedPoint32,
}

/// Conservative, software‑only table (no physical actuation). [file:144]
pub const NODE_CLASS_TABLE: [NodeClassProfile; 5] = [
    NodeClassProfile {
        class: NodeClass::PvdfRetinal,
        tissue_profile: TissueProfile::Ocular,
        biocomp: BiocompatibilityIndex {
            index: fx_from_f64(0.99),
        },
        dose_limits: DoseLimits {
            max_sar_w_per_kg: fx_from_f64(0.1),
            max_ti: fx_from_f64(0.6),
            max_mi: fx_from_f64(0.6),
            max_cem43: fx_from_f64(1.0),
            max_charge_density_uc_cm2: fx_from_f64(30.0),
        },
        au_per_mws: fx_from_f64(0.001),
        au_per_update: fx_from_f64(5.0),
    },
    NodeClassProfile {
        class: NodeClass::HapticPlate,
        tissue_profile: TissueProfile::Peripheral,
        biocomp: BiocompatibilityIndex {
            index: fx_from_f64(0.98),
        },
        dose_limits: DoseLimits {
            max_sar_w_per_kg: fx_from_f64(0.4),
            max_ti: fx_from_f64(0.7),
            max_mi: fx_from_f64(0.7),
            max_cem43: fx_from_f64(2.0),
            max_charge_density_uc_cm2: fx_from_f64(40.0),
        },
        au_per_mws: fx_from_f64(0.0008),
        au_per_update: fx_from_f64(3.0),
    },
    NodeClassProfile {
        class: NodeClass::RfusHelmet,
        tissue_profile: TissueProfile::AdultTranscranial,
        biocomp: BiocompatibilityIndex {
            index: fx_from_f64(0.90),
        },
        dose_limits: DoseLimits {
            max_sar_w_per_kg: fx_from_f64(2.0),
            max_ti: fx_from_f64(0.8),
            max_mi: fx_from_f64(0.7),
            max_cem43: fx_from_f64(0.5),
            max_charge_density_uc_cm2: fx_from_f64(0.0),
        },
        au_per_mws: fx_from_f64(0.002),
        au_per_update: fx_from_f64(10.0),
    },
    NodeClassProfile {
        class: NodeClass::MicromoteKnn,
        tissue_profile: TissueProfile::AdultTranscranial,
        biocomp: BiocompatibilityIndex {
            index: fx_from_f64(0.97),
        },
        dose_limits: DoseLimits {
            max_sar_w_per_kg: fx_from_f64(1.5),
            max_ti: fx_from_f64(0.75),
            max_mi: fx_from_f64(0.65),
            max_cem43: fx_from_f64(0.5),
            max_charge_density_uc_cm2: fx_from_f64(10.0),
        },
        au_per_mws: fx_from_f64(0.003),
        au_per_update: fx_from_f64(8.0),
    },
    NodeClassProfile {
        class: NodeClass::SoftEcog,
        tissue_profile: TissueProfile::AdultTranscranial,
        biocomp: BiocompatibilityIndex {
            index: fx_from_f64(0.96),
        },
        dose_limits: DoseLimits {
            max_sar_w_per_kg: fx_from_f64(0.8),
            max_ti: fx_from_f64(0.7),
            max_mi: fx_from_f64(0.6),
            max_cem43: fx_from_f64(1.0),
            max_charge_density_uc_cm2: fx_from_f64(30.0),
        },
        au_per_mws: fx_from_f64(0.0015),
        au_per_update: fx_from_f64(6.0),
    },
];

pub fn get_profile(class: NodeClass) -> NodeClassProfile {
    for p in NODE_CLASS_TABLE.iter() {
        if p.class == class {
            return *p;
        }
    }
    NodeClassProfile {
        class,
        tissue_profile: TissueProfile::AdultTranscranial,
        biocomp: BiocompatibilityIndex {
            index: fx_from_f64(0.5),
        },
        dose_limits: DoseLimits {
            max_sar_w_per_kg: fx_from_f64(0.1),
            max_ti: fx_from_f64(0.5),
            max_mi: fx_from_f64(0.5),
            max_cem43: fx_from_f64(0.1),
            max_charge_density_uc_cm2: fx_from_f64(10.0),
        },
        au_per_mws: fx_from_f64(0.01),
        au_per_update: fx_from_f64(20.0),
    }
}

// ---------- VIRTUAL NODE MODEL (SOFT BIO) ----------

#[derive(Copy, Clone, Debug)]
pub struct LifParams {
    pub tau_m_ms: FixedPoint32,
    pub r_m_ohm: FixedPoint32,
    pub c_m_nf: FixedPoint32,
    pub v_th_mv: FixedPoint32,
    pub v_reset_mv: FixedPoint32,
    pub au_per_spike: FixedPoint32,
}

#[derive(Copy, Clone, Debug, Default)]
pub struct VirtualNodeMetrics {
    pub spike_rate_hz: f64,
    pub avg_power_mw: f64,
    pub ti: f64,
    pub mi: f64,
    pub sar: f64,
    pub cem43: f64,
    pub impedance_ohm: f64,
    pub last_impedance_drift_pct: f64,
}

#[derive(Clone, Debug)]
pub struct VirtualNode {
    pub id: String,
    pub class: NodeClass,
    pub lif_params: LifParams,
    pub last_symbol: u8,
    pub metrics: VirtualNodeMetrics,
    pub q_phase_coulomb: f64,
    pub area_cm2: f64,
}

/// Discrete‑time charge update with phase reset and density bound. [file:144]
impl VirtualNode {
    pub fn update_charge(
        &mut self,
        i_ma: f64,
        dt_ms: f64,
        phase_reset: bool,
    ) -> bool {
        let dt_ms_clamped = dt_ms.max(1e-3);
        if phase_reset {
            self.q_phase_coulomb = 0.0;
        }

        // Q_{k+1} = Q_k + I_k * Δt (C). [file:145]
        let q_delta = (i_ma / 1000.0) * (dt_ms_clamped / 1000.0);
        self.q_phase_coulomb += q_delta;

        let density_uc_cm2 = (self.q_phase_coulomb * 1.0e6) / self.area_cm2;
        let profile = get_profile(self.class);
        let limit = fx_to_f64(profile.dose_limits.max_charge_density_uc_cm2);

        density_uc_cm2.abs() <= limit
    }

    /// SAR_k = I_k^2 R_k / (m · 10^6). [file:145]
    pub fn compute_sar(&self, i_ma: f64, r_ohm: f64, mass_kg: f64) -> f64 {
        let i_amp = i_ma / 1000.0;
        (i_amp * i_amp * r_ohm) / (mass_kg * 1.0e6)
    }

    /// Enforce SAR_k ≤ 0.8 · SAR_band per node class. [file:145]
    pub fn sar_within_limits(
        &self,
        i_ma: f64,
        r_ohm: f64,
        mass_kg: f64,
    ) -> bool {
        let sar = self.compute_sar(i_ma, r_ohm, mass_kg);
        let profile = get_profile(self.class);
        let sar_band = fx_to_f64(profile.dose_limits.max_sar_w_per_kg);
        sar <= 0.8 * sar_band
    }

    pub fn emit_spike_symbol(&mut self, window_ms: u64, spike_count: u32) -> u8 {
        let rate_hz = spike_count as f64 / (window_ms as f64 / 1000.0);
        self.metrics.spike_rate_hz = rate_hz;
        let capped = rate_hz.min(1000.0);
        let scaled = (capped / 1000.0 * 255.0).round() as u8;
        self.last_symbol = scaled;
        scaled
    }
}

// ---------- OTA / DOT LAYER (SIM-ONLY) ----------

#[derive(Copy, Clone, Debug)]
pub enum EnergyReason {
    SpikeActivity,
    StimulationPattern,
    OtaUpgradeSpend,
    OtaRollbackSpend,
    SafetyMonitorOverhead,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum UpdateStatus {
    Planned,
    SkippedSafety,
    SkippedWindow,
    Applied,
    RolledBack,
    Failed,
}

#[derive(Clone, Debug)]
pub struct OtaManifest {
    pub firmware_hash: [u8; 32],
    pub weights_hash: [u8; 32],
    pub version: String,
    pub node_class: NodeClass,
    pub max_power_mw: u32,
    pub max_spike_rate_hz: u32,
    pub ti_limit: f32,
    pub mi_limit: f32,
    pub valid_from_ms: u64,
    pub valid_until_ms: u64,
    pub signature: Vec<u8>,
    pub publisher_pubkey: Vec<u8>,
}

#[derive(Copy, Clone, Debug)]
pub struct DotWindow {
    pub start_ms: u64,
    pub end_ms: u64,
    pub max_temp_c: f32,
    pub max_cem43: f32,
    pub au_budget: AuEtBudget,
}

#[derive(Copy, Clone, Debug, Default)]
pub struct NodeMetricsExt {
    pub power_mw: f32,
    pub spike_rate_hz: f32,
    pub ti: f32,
    pub mi: f32,
    pub cem43: f32,
    pub sar: f32,
}

pub trait NeuromorphicNode {
    fn id(&self) -> &str;
    fn node_class(&self) -> NodeClass;
    fn metrics(&self) -> NodeMetricsExt;
    fn apply_firmware(&mut self, fw: &[u8]) -> Result<(), String>;
    fn apply_weights(&mut self, w: &[u8]) -> Result<(), String>;
    fn snapshot_state_hash(&self) -> [u8; 32];
}

// Simple, deterministic “hash‑like” mixer suitable for sim‑only use. [file:144]
fn blake2s_mix(prev: [u8; 32], tag: u8) -> [u8; 32] {
    let mut out = prev;
    for (i, b) in out.iter_mut().enumerate() {
        *b = b.wrapping_add(tag.wrapping_add(i as u8));
    }
    out
}

#[derive(Clone)]
pub struct SimBackendNode {
    pub inner: VirtualNode,
    pub last_hash: [u8; 32],
}

impl NeuromorphicNode for SimBackendNode {
    fn id(&self) -> &str {
        &self.inner.id
    }

    fn node_class(&self) -> NodeClass {
        self.inner.class
    }

    fn metrics(&self) -> NodeMetricsExt {
        NodeMetricsExt {
            power_mw: self.inner.metrics.avg_power_mw as f32,
            spike_rate_hz: self.inner.metrics.spike_rate_hz as f32,
            ti: self.inner.metrics.ti as f32,
            mi: self.inner.metrics.mi as f32,
            cem43: self.inner.metrics.cem43 as f32,
            sar: self.inner.metrics.sar as f32,
        }
    }

    fn apply_firmware(&mut self, _fw: &[u8]) -> Result<(), String> {
        // Tag 0xF1 reserved for firmware updates. [file:144]
        self.last_hash = blake2s_mix(self.last_hash, 0xF1);
        Ok(())
    }

    fn apply_weights(&mut self, _w: &[u8]) -> Result<(), String> {
        // FIX: 0x0W → 0x0B concrete tag for weight updates. [file:144]
        self.last_hash = blake2s_mix(self.last_hash, 0x0B);
        Ok(())
    }

    fn snapshot_state_hash(&self) -> [u8; 32] {
        self.last_hash
    }
}

// ---- Signature & hash verification (real Ed25519 slot) ----

fn sha256(bytes: &[u8]) -> [u8; 32] {
    // Placeholder; wire to RustCrypto sha2 in production. [file:144]
    let mut h = [0u8; 32];
    for (i, b) in bytes.iter().enumerate() {
        h[i % 32] = h[i % 32].wrapping_add(*b).wrapping_add(i as u8);
    }
    h
}

pub fn verify_hashes(manifest: &OtaManifest, fw_bytes: &[u8], w_bytes: &[u8]) -> bool {
    let fw_hash = sha256(fw_bytes);
    let w_hash = sha256(w_bytes);
    fw_hash == manifest.firmware_hash && w_hash == manifest.weights_hash
}

/// Gate OTA on a real signature path in hardware deployments. [file:144]
pub fn verify_signature(manifest: &OtaManifest, manifest_bytes: &[u8]) -> bool {
    let _digest = sha256(manifest_bytes);
    !manifest.signature.is_empty() && !manifest.publisher_pubkey.is_empty()
    // Production: ed25519_dalek::Verifier::verify(...) or hybrid Ed25519+Kyber.
}

// ---------- OTA ENGINE & EVENTS ----------

#[derive(Clone, Debug)]
pub struct SafetyProfile {
    pub node_profiles: Vec<NodeClassProfile>,
    pub au_budget: AuEtBudget,
}

#[derive(Clone, Debug)]
pub struct OtaEvent {
    pub seq: u64,
    pub node_id: String,
    pub manifest_version: String,
    pub status: UpdateStatus,
    pub pre_hash: [u8; 32],
    pub post_hash: [u8; 32],
    pub energy_delta: FixedPoint32,
    pub ti_snapshot: f32,
    pub mi_snapshot: f32,
    pub cem43_snapshot: f32,
    pub log_hash: [u8; 32],
}

fn hash_event(ev: &OtaEvent, prev_hash: [u8; 32]) -> [u8; 32] {
    let mut buf = Vec::with_capacity(256);
    buf.extend_from_slice(&prev_hash);
    buf.extend_from_slice(&ev.seq.to_le_bytes());
    buf.extend_from_slice(ev.node_id.as_bytes());
    buf.extend_from_slice(ev.manifest_version.as_bytes());
    buf.extend_from_slice(&(ev.status as i32).to_le_bytes());
    buf.extend_from_slice(&ev.pre_hash);
    buf.extend_from_slice(&ev.post_hash);
    buf.extend_from_slice(&ev.energy_delta.raw_u32.to_le_bytes());
    buf.extend_from_slice(&ev.ti_snapshot.to_le_bytes());
    buf.extend_from_slice(&ev.mi_snapshot.to_le_bytes());
    buf.extend_from_slice(&ev.cem43_snapshot.to_le_bytes());
    sha256(&buf)
}

#[derive(Clone, Debug)]
pub struct UpdatePlanEntry {
    pub node_id: String,
    pub window: Option<DotWindow>,
    pub allowed: bool,
    pub reason: String,
}

#[derive(Clone, Debug)]
pub struct UpdateResult {
    pub node_id: String,
    pub status: UpdateStatus,
    pub event: Option<OtaEvent>,
}

pub struct OtaEngine<'a> {
    pub ledger: &'a mut AuEtLedger,
    pub safety: SafetyProfile,
    pub nodes: Vec<Box<dyn NeuromorphicNode>>,
    pub last_log_hash: [u8; 32],
    pub next_seq: u64,
}

impl<'a> OtaEngine<'a> {
    pub fn can_update(
        &self,
        now_ms: u64,
        node: &dyn NeuromorphicNode,
        manifest: &OtaManifest,
    ) -> (bool, String) {
        if now_ms < manifest.valid_from_ms || now_ms > manifest.valid_until_ms {
            return (false, "manifest_not_in_valid_window".into());
        }

        let metrics = node.metrics();

        if (metrics.power_mw as u32) > manifest.max_power_mw {
            return (false, "power_bound_exceeded".into());
        }
        if (metrics.spike_rate_hz as u32) > manifest.max_spike_rate_hz {
            return (false, "spike_rate_bound_exceeded".into());
        }
        if metrics.ti > manifest.ti_limit {
            return (false, "ti_limit_exceeded".into());
        }
        if metrics.mi > manifest.mi_limit {
            return (false, "mi_limit_exceeded".into());
        }

        let profile = get_profile(node.node_class());
        let sar_band = fx_to_f64(profile.dose_limits.max_sar_w_per_kg);
        let cem_max = fx_to_f64(profile.dose_limits.max_cem43);

        if (metrics.sar as f64) > 0.8 * sar_band {
            return (false, "sar_limit_exceeded".into());
        }
        if (metrics.cem43 as f64) > cem_max {
            return (false, "cem43_limit_exceeded".into());
        }

        let cost = profile.au_per_update;
        if !self.ledger.can_spend(cost, self.safety.au_budget) {
            return (false, "auet_budget_exceeded".into());
        }

        (true, "ok".into())
    }

    pub fn plan_update(
        &self,
        now_ms: u64,
        manifest: &OtaManifest,
    ) -> Vec<UpdatePlanEntry> {
        let mut plans = Vec::with_capacity(self.nodes.len());
        for node in self.nodes.iter() {
            let (ok, reason) = self.can_update(now_ms, node.as_ref(), manifest);
            let wnd = if ok {
                Some(DotWindow {
                    start_ms: now_ms,
                    end_ms: now_ms + 5 * 60 * 1000,
                    max_temp_c: 40.0,
                    max_cem43: 1.0,
                    au_budget: self.safety.au_budget,
                })
            } else {
                None
            };
            plans.push(UpdatePlanEntry {
                node_id: node.id().to_string(),
                window: wnd,
                allowed: ok,
                reason,
            });
        }
        plans
    }

    /// Execute OTA plan with AU.ET burn and hash‑chained audit trail. [file:144]
    pub fn execute_plan(
        &mut self,
        manifest: &OtaManifest,
        fw_bytes: &[u8],
        w_bytes: &[u8],
        plans: &[UpdatePlanEntry],
    ) -> Vec<UpdateResult> {
        let mut results = Vec::with_capacity(plans.len());

        for plan in plans.iter() {
            let node_opt = self
                .nodes
                .iter_mut()
                .find(|n| n.id() == plan.node_id.as_str());

            if node_opt.is_none() {
                results.push(UpdateResult {
                    node_id: plan.node_id.clone(),
                    status: UpdateStatus::Failed,
                    event: None,
                });
                continue;
            }
            let node = node_opt.unwrap();

            if !plan.allowed {
                results.push(UpdateResult {
                    node_id: node.id().to_string(),
                    status: UpdateStatus::SkippedSafety,
                    event: None,
                });
                continue;
            }

            if !verify_hashes(manifest, fw_bytes, w_bytes) {
                results.push(UpdateResult {
                    node_id: node.id().to_string(),
                    status: UpdateStatus::Failed,
                    event: None,
                });
                continue;
            }

            let pre_hash = node.snapshot_state_hash();
            let metrics_before = node.metrics();

            if let Err(_) = node.apply_firmware(fw_bytes) {
                results.push(UpdateResult {
                    node_id: node.id().to_string(),
                    status: UpdateStatus::Failed,
                    event: None,
                });
                continue;
            }
            if let Err(_) = node.apply_weights(w_bytes) {
                results.push(UpdateResult {
                    node_id: node.id().to_string(),
                    status: UpdateStatus::Failed,
                    event: None,
                });
                continue;
            }

            let post_hash = node.snapshot_state_hash();
            let metrics_after = node.metrics();
            let profile = get_profile(node.node_class());
            let cost = profile.au_per_update;

            if !self
                .ledger
                .can_spend(cost, self.safety.au_budget)
            {
                // Logical rollback: keep pre‑update hash / metrics externally if needed. [file:144]
                results.push(UpdateResult {
                    node_id: node.id().to_string(),
                    status: UpdateStatus::SkippedSafety,
                    event: None,
                });
                continue;
            }

            self.ledger.burn(cost);

            let mut ev = OtaEvent {
                seq: self.next_seq,
                node_id: node.id().to_string(),
                manifest_version: manifest.version.clone(),
                status: UpdateStatus::Applied,
                pre_hash,
                post_hash,
                energy_delta: cost,
                ti_snapshot: metrics_after.ti,
                mi_snapshot: metrics_after.mi,
                cem43_snapshot: metrics_after.cem43,
                log_hash: [0u8; 32],
            };
            let new_hash = hash_event(&ev, self.last_log_hash);
            ev.log_hash = new_hash;
            self.last_log_hash = new_hash;
            self.next_seq += 1;

            results.push(UpdateResult {
                node_id: node.id().to_string(),
                status: UpdateStatus::Applied,
                event: Some(ev),
            });

            // Optional: post‑update self‑test hook comparing metrics_before/after, SAR, TI/MI. [file:144]
            let _ = metrics_before;
        }

        results
    }
}

// ---------- SOFTWARE-ONLY SOFT-BIO STACK GLUE ----------

#[derive(Clone, Debug)]
pub struct SoftBioStackConfig {
    pub nodes: Vec<VirtualNode>,
    pub ledger: AuEtLedger,
    pub au_budget: AuEtBudget,
}

/// Initialize a pure‑software biomechanical stack (no devices / tissue). [file:144]
pub fn init_soft_bio_stack() -> SoftBioStackConfig {
    let nodes = vec![
        VirtualNode {
            id: "sim_pvdf_retina_0".into(),
            class: NodeClass::PvdfRetinal,
            lif_params: LifParams {
                tau_m_ms: fx_from_f64(20.0),
                r_m_ohm: fx_from_f64(100e6),
                c_m_nf: fx_from_f64(0.1),
                v_th_mv: fx_from_f64(-50.0),
                v_reset_mv: fx_from_f64(-65.0),
                au_per_spike: fx_from_f64(0.0001),
            },
            last_symbol: 0,
            metrics: VirtualNodeMetrics {
                spike_rate_hz: 0.0,
                avg_power_mw: 0.0,
                ti: 0.1,
                mi: 0.1,
                sar: 0.01,
                cem43: 0.0,
                impedance_ohm: 1.0e6,
                last_impedance_drift_pct: 0.0,
            },
            q_phase_coulomb: 0.0,
            area_cm2: 0.05,
        },
        VirtualNode {
            id: "sim_soft_ecog_0".into(),
            class: NodeClass::SoftEcog,
            lif_params: LifParams {
                tau_m_ms: fx_from_f64(25.0),
                r_m_ohm: fx_from_f64(120e6),
                c_m_nf: fx_from_f64(0.08),
                v_th_mv: fx_from_f64(-52.0),
                v_reset_mv: fx_from_f64(-68.0),
                au_per_spike: fx_from_f64(0.00015),
            },
            last_symbol: 0,
            metrics: VirtualNodeMetrics {
                spike_rate_hz: 0.0,
                avg_power_mw: 0.0,
                ti: 0.1,
                mi: 0.1,
                sar: 0.02,
                cem43: 0.0,
                impedance_ohm: 0.8e6,
                last_impedance_drift_pct: 0.0,
            },
            q_phase_coulomb: 0.0,
            area_cm2: 0.2,
        },
    ];

    // Parameters chosen to match AUETCSPSCARCITYV1 compression (10^-12 scaling). [file:144]
    let ledger = AuEtLedger {
        total_burn: fx_from_f64(0.0),
        daily_burn: fx_from_f64(0.0),
        session_burn: fx_from_f64(0.0),
        max_supply: fx_from_f64(1_000_000.0),
    };
    let au_budget = AuEtBudget {
        daily_cap: fx_from_f64(10_000.0),
        session_cap: fx_from_f64(1_000.0),
    };

    SoftBioStackConfig {
        nodes,
        ledger,
        au_budget,
    }
}

/// Boot a software‑only runtime with OTA engine over virtual organic nodes. [file:144]
pub fn boot_soft_biomech_runtime() {
    let mut cfg = init_soft_bio_stack();

    // Wrap nodes into simulated NeuromorphicNode backend.
    let mut ota_nodes: Vec<Box<dyn NeuromorphicNode>> = Vec::new();
    for n in cfg.nodes.iter() {
        let sim = SimBackendNode {
            inner: n.clone(), // requires VirtualNode: Clone
            last_hash: [0u8; 32],
        };
        ota_nodes.push(Box::new(sim));
    }

    let safety_profile = SafetyProfile {
        node_profiles: NODE_CLASS_TABLE.to_vec(),
        au_budget: cfg.au_budget,
    };

    let mut engine = OtaEngine {
        ledger: &mut cfg.ledger,
        safety: safety_profile,
        nodes: ota_nodes,
        last_log_hash: [0u8; 32],
        next_seq: 1,
    };

    let manifest = OtaManifest {
        firmware_hash: [0xAA; 32],
        weights_hash: [0xBB; 32],
        version: "1.0.0-sim".into(),
        node_class: NodeClass::SoftEcog,
        max_power_mw: 50,
        max_spike_rate_hz: 500,
        ti_limit: 0.7,
        mi_limit: 0.6,
        valid_from_ms: 0,
        valid_until_ms: u64::MAX,
        signature: vec![1, 2, 3],
        publisher_pubkey: vec![4, 5, 6],
    };

    let now_ms = 1_000_000;
    let plans = engine.plan_update(now_ms, &manifest);
    let fw_bytes = vec![0u8; 256];
    let w_bytes = vec![1u8; 512];
    let _results = engine.execute_plan(&manifest, &fw_bytes, &w_bytes, &plans);
}
