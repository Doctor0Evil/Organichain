// destination-path: src/organichain/consensus/organichain_neural_consensus_v1.rs

use sha2::{Digest, Sha256};
use serde::{Serialize, Deserialize};

/// Canonical energy state for an Organichain validator, in AU.ET and CSP units
/// AU.ET: usage energy (up to 1e18 internal units); CSP: structural risk / trust budget.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EnergyState {
    pub au_et: u128,
    pub csp:   u128,
}

/// Safety envelope calibrated against ICNIRP / IEEE SAR and current-density guidance
/// sar_w_kg_max: local SAR, j_density_ma_cm2: induced current density, temp_rise_mdeg_c: m°C.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SafetyEnvelope {
    pub sar_w_kg_max:      u32,
    pub j_density_ma_cm2:  u32,
    pub temp_rise_mdeg_c:  u32,
}

/// Static metadata for each validator node, including region and neuromorphic flag.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ValidatorMeta {
    pub id:           String,
    pub stake:        u128,
    pub reliability:  f32,
    pub region:       String,
    pub energy:       EnergyState,
    pub safety:       SafetyEnvelope,
    pub neuromorphic: bool,
}

/// Fixed-point neural score for consensus ranking: score_fp = score * 1e6.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NeuralScore {
    pub validator_id: String,
    pub score_fp:     i64,
}

/// zk-SNARK proof that a neuromorphic (SNN) model evaluated a given state correctly.
/// model_hash, state_root and proof_bytes are all included in the proof circuit.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ZkNeuroProof {
    pub proof_bytes:  Vec<u8>,
    pub model_hash:   [u8; 32],
    pub state_root:   [u8; 32],
}

/// Composite attestation combining TPM/TEE quotes and zk-SNARK proof for neuromorphic output.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NeuroCompositeAttest {
    pub host_tpm_quote: Vec<u8>,
    pub tee_report:     Vec<u8>,
    pub driver_hash:    [u8; 32],
    pub zk_neuro:       ZkNeuroProof,
    pub event_hash:     [u8; 32],
}

/// Trait for verifying composite neuromorphic attestations.
pub trait AttestationVerifier {
    fn verify_neuro_attest(&self, attest: &NeuroCompositeAttest) -> bool;
}

/// Simple SHA-256 helper used to bind NeuralScore to a specific attestation record.
pub fn hash_bytes(b: &[u8]) -> [u8; 32] {
    let mut h = Sha256::new();
    h.update(b);
    h.finalize().into()
}

/// Hard-coded global safety limits for Organichain v1, aligned with conservative implants:
/// SAR <= 1.6 W/kg (head-torso), J-density <= 10 mA/cm^2, ΔT <= 500 m°C (0.5 °C).
pub const GLOBAL_SAFETY_LIMITS: SafetyEnvelope = SafetyEnvelope {
    sar_w_kg_max:      16,   // scaled by 10 -> 1.6 W/kg
    j_density_ma_cm2:  10,   // 10 mA/cm^2
    temp_rise_mdeg_c:  500,  // 0.5 °C
};

/// Absolute minimum AU.ET and CSP required to participate as a validator.
/// AU.ET_MIN ~ 1e12 internal units; CSP_MIN ~ 1e9 internal units.
pub const MIN_AU_ET: u128 = 1_000_000_000_000;
pub const MIN_CSP:   u128 =   500_000_000;

/// Example driver hash for Loihi/Akida bridge driver (SHA-256 of binary, placeholder bytes).
pub const LOIHI_AKIDA_DRIVER_HASH: [u8; 32] = [
    0xa9, 0x71, 0x3b, 0x5f, 0x42, 0xc1, 0x9d, 0x88,
    0x13, 0x7e, 0xb2, 0x6a, 0xcd, 0x34, 0x80, 0x51,
    0x9c, 0x4e, 0x27, 0x6f, 0xd8, 0x01, 0x72, 0xaa,
    0x3c, 0x59, 0xe1, 0x0f, 0xbc, 0x44, 0x2d, 0x93,
];

/// Example model hash for the Organichain SNN consensus policy deployed on neuromorphic nodes.
/// This is the SHA-256 of an ONNX or Lava model file.
pub const ORGANICHAIN_SNN_MODEL_HASH: [u8; 32] = [
    0x4f, 0x21, 0x9a, 0x7b, 0xde, 0x10, 0x63, 0x48,
    0xba, 0x92, 0x01, 0xee, 0x5c, 0x39, 0xa0, 0xd7,
    0x84, 0x56, 0xf2, 0x11, 0x6e, 0x33, 0x7c, 0xb9,
    0x08, 0xda, 0xe7, 0x45, 0x12, 0xfe, 0x65, 0xc0,
];

/// Example SNN state Merkle root (commitment to neuromorphic state at evaluation time).
pub const ORGANICHAIN_SNN_STATE_ROOT: [u8; 32] = [
    0x90, 0x3c, 0x51, 0x78, 0x0a, 0xbf, 0x2e, 0x67,
    0xd9, 0x04, 0x8b, 0x3a, 0xce, 0x16, 0x25, 0xf1,
    0x6d, 0xaa, 0x19, 0x8e, 0x37, 0x5b, 0xc8, 0x02,
    0x4a, 0x99, 0xed, 0x73, 0x58, 0x0c, 0xe3, 0x41,
];

/// Determines whether a validator meets minimum energy and falls under safety limits.
pub fn can_participate(
    v: &ValidatorMeta,
    min_au: u128,
    min_csp: u128,
    safety_limits: &SafetyEnvelope,
) -> bool {
    if v.energy.au_et < min_au || v.energy.csp < min_csp {
        return false;
    }
    if v.safety.sar_w_kg_max > safety_limits.sar_w_kg_max {
        return false;
    }
    if v.safety.j_density_ma_cm2 > safety_limits.j_density_ma_cm2 {
        return false;
    }
    if v.safety.temp_rise_mdeg_c > safety_limits.temp_rise_mdeg_c {
        return false;
    }
    true
}

/// Select the top-k validators by neural score, enforcing AU.ET/CSP thresholds and
/// neuromorphic attestation for SNN-derived scores.
pub fn pick_validators(
    validators: &[ValidatorMeta],
    neural_scores: &[NeuralScore],
    attestations: &[NeuroCompositeAttest],
    attest_verifier: &dyn AttestationVerifier,
    min_au: u128,
    min_csp: u128,
    safety_limits: &SafetyEnvelope,
    k: usize,
) -> Vec<ValidatorMeta> {
    let mut scored: Vec<(ValidatorMeta, i64)> = Vec::new();

    for v in validators.iter() {
        if !can_participate(v, min_au, min_csp, safety_limits) {
            continue;
        }

        let mut score_fp = 0_i64;

        if let Some(ns) = neural_scores.iter().find(|s| s.validator_id == v.id) {
            if v.neuromorphic {
                let evh = hash_bytes(serde_json::to_string(ns).unwrap().as_bytes());
                if let Some(att) = attestations
                    .iter()
                    .find(|a| a.event_hash == evh && attest_verifier.verify_neuro_attest(a))
                {
                    let _ = att; // proof already checked in verifier
                    score_fp = ns.score_fp;
                } else {
                    score_fp = 0; // discard un-attested neuromorphic scores
                }
            } else {
                score_fp = ns.score_fp;
            }
        }

        scored.push((v.clone(), score_fp));
    }

    scored.sort_by(|a, b| b.1.cmp(&a.1));
    scored.into_iter().take(k).map(|(v, _)| v).collect()
}

/// ---------------------------------------------------------------------------
/// Example in-memory datasets for high-performance cybernetic / neuromorphic nodes
/// These are realistic Organichain validator profiles covering neuromorphic,
/// organic-computing, and conventional cybernetic chipsets.
/// ---------------------------------------------------------------------------

/// Example validators dataset for testnets and CI.
/// - validator_loihi_edge: SNN-based Loihi/Akida node in EU smart-city edge.
/// - validator_organoid_lab: organic-computing research node with strict safety envelope.
/// - validator_gpu_core: GPU-based Prometheus-style node for heavy neural-policy training.
/// - validator_helm_policy: Helm policy node with high CSP and very low SAR exposure.
pub fn example_validators() -> Vec<ValidatorMeta> {
    vec![
        ValidatorMeta {
            id:           "validator_loihi_edge".to_string(),
            stake:        5_000_000_000_000_000,  // 5e15 internal stake units
            reliability:  0.9975,                 // 99.75% uptime
            region:       "EU.DE.Berlin.EdgeZone1".to_string(),
            energy: EnergyState {
                au_et: 8_500_000_000_000,        // AU.ET budget ~ 8.5e12
                csp:   1_250_000_000,            // CSP budget ~ 1.25e9
            },
            safety: SafetyEnvelope {
                sar_w_kg_max:      8,            // 0.8 W/kg, below global limit
                j_density_ma_cm2:  4,            // 4 mA/cm^2
                temp_rise_mdeg_c:  220,          // 0.22 °C
            },
            neuromorphic: true,
        },
        ValidatorMeta {
            id:           "validator_organoid_lab".to_string(),
            stake:        1_000_000_000_000_000,  // 1e15
            reliability:  0.9850,
            region:       "US.CO.Boulder.BioLab01".to_string(),
            energy: EnergyState {
                au_et: 2_000_000_000_000,        // 2e12 AU.ET
                csp:     750_000_000,            // 7.5e8 CSP
            },
            safety: SafetyEnvelope {
                sar_w_kg_max:      4,            // 0.4 W/kg, lab-restricted
                j_density_ma_cm2:  2,            // 2 mA/cm^2
                temp_rise_mdeg_c:  150,          // 0.15 °C
            },
            neuromorphic: true,                  // treated as neuromorphic/organic accelerator
        },
        ValidatorMeta {
            id:           "validator_gpu_core".to_string(),
            stake:        9_500_000_000_000_000,  // 9.5e15
            reliability:  0.9990,
            region:       "AP.SG.SmartCityCore01".to_string(),
            energy: EnergyState {
                au_et: 15_000_000_000_000,       // 1.5e13 AU.ET
                csp:    3_500_000_000,           // 3.5e9 CSP
            },
            safety: SafetyEnvelope {
                sar_w_kg_max:      2,            // 0.2 W/kg (data-center only)
                j_density_ma_cm2:  1,            // ~ no direct tissue coupling
                temp_rise_mdeg_c:  50,           // 0.05 °C
            },
            neuromorphic: false,
        },
        ValidatorMeta {
            id:           "validator_helm_policy".to_string(),
            stake:        7_250_000_000_000_000,  // 7.25e15
            reliability:  0.9995,
            region:       "LATAM.CL.Santiago.GovHelm01".to_string(),
            energy: EnergyState {
                au_et: 6_000_000_000_000,        // 6e12 AU.ET
                csp:   5_000_000_000,            // 5e9 CSP (high governance trust)
            },
            safety: SafetyEnvelope {
                sar_w_kg_max:      1,            // 0.1 W/kg
                j_density_ma_cm2:  1,
                temp_rise_mdeg_c:  20,           // 0.02 °C
            },
            neuromorphic: false,
        },
    ]
}

/// Example neural scores generated by Prometheus/Helm neural policies, fixed-point 1e6.
/// These reflect combined factors: stake, reliability, latency, and energy-efficiency.
pub fn example_neural_scores() -> Vec<NeuralScore> {
    vec![
        NeuralScore {
            validator_id: "validator_loihi_edge".to_string(),
            score_fp:     945_000,  // 0.945
        },
        NeuralScore {
            validator_id: "validator_organoid_lab".to_string(),
            score_fp:     812_500,  // 0.8125
        },
        NeuralScore {
            validator_id: "validator_gpu_core".to_string(),
            score_fp:     973_250,  // 0.97325
        },
        NeuralScore {
            validator_id: "validator_helm_policy".to_string(),
            score_fp:     988_750,  // 0.98875 (governance bias)
        },
    ]
}

/// Example neuromorphic attestations for the neuromorphic validators.
/// In a live system, proof_bytes, host_tpm_quote and tee_report are real outputs from
/// a zRA/TPM stack; here we provide filled, deterministic arrays.
pub fn example_attestations() -> Vec<NeuroCompositeAttest> {
    // First build the NeuralScore objects and compute their event hashes.
    let scores = example_neural_scores();

    let loihi_score = scores
        .iter()
        .find(|s| s.validator_id == "validator_loihi_edge")
        .unwrap();
    let organoid_score = scores
        .iter()
        .find(|s| s.validator_id == "validator_organoid_lab")
        .unwrap();

    let loihi_event_hash = hash_bytes(
        serde_json::to_string(loihi_score).unwrap().as_bytes()
    );
    let organoid_event_hash = hash_bytes(
        serde_json::to_string(organoid_score).unwrap().as_bytes()
    );

    let zk_loihi = ZkNeuroProof {
        proof_bytes: vec![0x01; 192], // placeholder 192-byte zkSNARK proof
        model_hash:  ORGANICHAIN_SNN_MODEL_HASH,
        state_root:  ORGANICHAIN_SNN_STATE_ROOT,
    };

    let zk_organoid = ZkNeuroProof {
        proof_bytes: vec![0x02; 192],
        model_hash:  ORGANICHAIN_SNN_MODEL_HASH,
        state_root:  ORGANICHAIN_SNN_STATE_ROOT,
    };

    vec![
        NeuroCompositeAttest {
            host_tpm_quote: vec![0x11; 128],   // TPM quote bytes
            tee_report:     vec![0x22; 256],   // TEE attestation report
            driver_hash:    LOIHI_AKIDA_DRIVER_HASH,
            zk_neuro:       zk_loihi,
            event_hash:     loihi_event_hash,
        },
        NeuroCompositeAttest {
            host_tpm_quote: vec![0x33; 128],
            tee_report:     vec![0x44; 256],
            driver_hash:    LOIHI_AKIDA_DRIVER_HASH,
            zk_neuro:       zk_organoid,
            event_hash:     organoid_event_hash,
        },
    ]
}

/// Simple verifier that checks driver_hash, model_hash and proof length; in production this
/// would call a real zkSNARK verifier and TPM/TEE libraries.
pub struct DemoAttestationVerifier;

impl AttestationVerifier for DemoAttestationVerifier {
    fn verify_neuro_attest(&self, attest: &NeuroCompositeAttest) -> bool {
        if attest.driver_hash != LOIHI_AKIDA_DRIVER_HASH {
            return false;
        }
        if attest.zk_neuro.model_hash != ORGANICHAIN_SNN_MODEL_HASH {
            return false;
        }
        if attest.zk_neuro.proof_bytes.len() != 192 {
            return false;
        }
        // Basic integrity check on tee_report / host_tpm_quote lengths.
        if attest.host_tpm_quote.len() < 64 || attest.tee_report.len() < 128 {
            return false;
        }
        true
    }
}

/// Convenience function: run a full selection over the example dataset,
/// using global limits and the demo verifier.
pub fn pick_example_validators(k: usize) -> Vec<ValidatorMeta> {
    let validators     = example_validators();
    let scores         = example_neural_scores();
    let attestations   = example_attestations();
    let verifier       = DemoAttestationVerifier;

    pick_validators(
        &validators,
        &scores,
        &attestations,
        &verifier,
        MIN_AU_ET,
        MIN_CSP,
        &GLOBAL_SAFETY_LIMITS,
        k,
    )
}
