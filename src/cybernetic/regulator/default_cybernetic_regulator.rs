// path: src/cybernetic/regulator/default_cybernetic_regulator.rs

use serde::{Deserialize, Serialize};
use sha3::{Digest, Sha3_512};

/// Normalized cybernetic risk sample for neuromorphic implants. [file:1]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskSample {
    /// 0.0..1.0, higher = more risk
    pub risk_score: f64,
    /// 0.0..100.0, estimated effective energy drain %
    pub ed_percent: f64,
    /// >= 0.0, psych load factor
    pub sf_psych: f64,
    /// SHA3-512-derived modulus (lower 8 bytes as u64)
    pub h_mod: u64,
}

/// Regulator input state, mapped to AU.ET/CSP asset vectors upstream. [file:1]
#[derive(Debug, Clone)]
pub struct RegulatorInput {
    pub user_id: String,
    /// 256‑bit bio‑key, derived from TPM / secure element
    pub bio_key: [u8; 32],
    /// Neuromorphic depth / stack depth (0.0–10.0)
    pub depth: f64,
    /// Energy scalar (normalized AU/ET unit, 0.0–100.0)
    pub energy_scalar: f64,
    /// Unix time seconds
    pub t: u64,
    /// AU.ET balance (atto‑units)
    pub auet: u128,
    /// CSP balance (atto‑units)
    pub csp: u128,
}

/// Static, pure, side‑effect‑free regulator.
pub trait CyberneticRegulator {
    /// Deterministic: no I/O, no randomness.
    fn evaluate(&self, input: &RegulatorInput) -> RiskSample;
}

/// Concrete, parameterized regulator tuned for neuromorphic implants.
/// Weighting is compatible with upstream AU.ET/CSP and node‑swarm invariants. [file:1]
pub struct DefaultCyberneticRegulator {
    /// Weight for depth contribution
    pub w_depth: f64,
    /// Weight for energy scalar contribution
    pub w_energy: f64,
    /// Weight for AU.ET depletion contribution
    pub w_auet: f64,
    /// Weight for CSP depletion contribution
    pub w_csp: f64,
    /// Psych load scaling factor
    pub k_psych: f64,
    /// Minimum and maximum allowed risk clamp
    pub risk_min: f64,
    pub risk_max: f64,
}

impl Default for DefaultCyberneticRegulator {
    fn default() -> Self {
        Self {
            w_depth: 0.18,
            w_energy: 0.27,
            w_auet: 0.22,
            w_csp: 0.33,
            k_psych: 1.35,
            risk_min: 0.0,
            risk_max: 1.0,
        }
    }
}

impl DefaultCyberneticRegulator {
    #[inline]
    fn normalize_nonzero(v: f64, max: f64) -> f64 {
        if max <= 0.0 {
            0.0
        } else {
            (v / max).clamp(0.0, 1.0)
        }
    }

    /// Deterministic SHA3-512 modulus used for audit‑stable routing keys. [file:1]
    fn compute_hash_mod(user_id: &str, bio_key: &[u8; 32], t: u64) -> u64 {
        let mut hasher = Sha3_512::new();
        hasher.update(user_id.as_bytes());
        hasher.update(bio_key);
        hasher.update(t.to_le_bytes());
        let digest = hasher.finalize();
        let mut bytes = [0u8; 8];
        bytes.copy_from_slice(&digest[0..8]);
        u64::from_le_bytes(bytes)
    }
}

impl CyberneticRegulator for DefaultCyberneticRegulator {
    fn evaluate(&self, input: &RegulatorInput) -> RiskSample {
        // Normalize neuromorphic and energy factors into 0..1.
        let depth_n = Self::normalize_nonzero(input.depth, 10.0);
        let energy_n = Self::normalize_nonzero(input.energy_scalar, 100.0);

        // AU.ET / CSP floors aligned with upstream augmented‑human state vectors. [file:1]
        let auet_floor: f64 = 1.0e12;
        let csp_floor: f64 = 1.0e12;

        let auet_n = if input.auet == 0 {
            1.0
        } else {
            let v = (auet_floor / (input.auet as f64)).min(5.0);
            (v / 5.0).clamp(0.0, 1.0)
        };

        let csp_n = if input.csp == 0 {
            1.0
        } else {
            let v = (csp_floor / (input.csp as f64)).min(5.0);
            (v / 5.0).clamp(0.0, 1.0)
        };

        // Linear risk aggregation; stays within ALN invariants when weights sum ≤ 1. [file:1]
        let raw = self.w_depth * depth_n
            + self.w_energy * energy_n
            + self.w_auet * auet_n
            + self.w_csp * csp_n;

        let risk_score = raw.clamp(self.risk_min, self.risk_max);

        // Energy drain mapping: 0..1 -> 0..100 %
        let ed_percent = (risk_score * 100.0).clamp(0.0, 100.0);

        // Psych load factor: weighted combination of depth and CSP stress
        let sf_psych = self.k_psych * (0.6 * depth_n + 0.4 * csp_n);

        let h_mod = Self::compute_hash_mod(&input.user_id, &input.bio_key, input.t);

        RiskSample {
            risk_score,
            ed_percent,
            sf_psych,
            h_mod,
        }
    }
}
