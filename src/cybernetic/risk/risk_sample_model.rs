// path: src/cybernetic/risk/risk_sample_model.rs

use serde::{Deserialize, Serialize};

/// Deterministic risk evaluation output for neuromorphic cybernetic regulators.
/// Conforms to https://au.bioaug.org/schemas/RiskSample.schema.json
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskSample {
    /// 0.0..1.0, higher = more risk
    #[serde(rename = "risk_score")]
    pub risk_score: f64,

    /// 0.0..100.0, estimated effective energy drain %
    #[serde(rename = "ed_percent")]
    pub ed_percent: f64,

    /// >= 0.0, psych load factor
    #[serde(rename = "sf_psych")]
    pub sf_psych: f64,

    /// SHA3-512-derived modulus (lower 8 bytes as u64)
    #[serde(rename = "h_mod")]
    pub h_mod: u64,
}

impl RiskSample {
    /// Constructs a RiskSample while enforcing schema bounds at runtime.
    /// This is deterministic and side-effect-free.
    pub fn new(
        risk_score: f64,
        ed_percent: f64,
        sf_psych: f64,
        h_mod: u64,
    ) -> Self {
        let risk_score = risk_score.clamp(0.0, 1.0);
        let ed_percent = ed_percent.clamp(0.0, 100.0);
        let sf_psych = sf_psych.max(0.0);

        Self {
            risk_score,
            ed_percent,
            sf_psych,
            h_mod,
        }
    }

    /// Validates that all fields satisfy the JSON Schema constraints.
    /// Returns true only if the instance is fully compliant.
    pub fn is_schema_compliant(&self) -> bool {
        (0.0..=1.0).contains(&self.risk_score)
            && (0.0..=100.0).contains(&self.ed_percent)
            && self.sf_psych >= 0.0
            // u64 already guarantees 0..=18446744073709551615
    }
}
