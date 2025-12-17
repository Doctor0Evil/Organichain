// path: src/aln_challenges/mod.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChallengeSpecV1 {
    pub challenge_id: String,
    pub epoch: u64,
    pub policy_hash: String,
    pub target_metric: String,
    pub target_constraint: String,
    pub reward_auet: u128,
    pub reward_csp: u128,
    pub penalty_auet: u128,
    pub penalty_csp: u128,
    pub verification_method: String,
    pub expires_epoch: u64,
    pub engine_tags: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChallengeBatch {
    pub epoch: u64,
    pub policy_hash: String,
    pub challenges: Vec<ChallengeSpecV1>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LedgerDelta {
    pub delta_auet: i128,
    pub delta_csp: i128,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ledger {
    pub auet_balance: u128,
    pub csp_balance: u128,
}

impl Ledger {
    pub fn apply_delta(&mut self, delta: &LedgerDelta) {
        let da = if delta.delta_auet.is_negative() {
            let mag = (-delta.delta_auet) as u128;
            self.auet_balance.saturating_sub(mag)
        } else {
            self.auet_balance.saturating_add(delta.delta_auet as u128)
        };
        let dc = if delta.delta_csp.is_negative() {
            let mag = (-delta.delta_csp) as u128;
            self.csp_balance.saturating_sub(mag)
        } else {
            self.csp_balance.saturating_add(delta.delta_csp as u128)
        };
        self.auet_balance = da;
        self.csp_balance = dc;
    }
}

fn next_challenge_id(epoch: u64, counter: u64) -> String {
    format!("epoch-{epoch}-ch-{counter:08x}")
}

pub fn generate_epoch_challenges(
    epoch: u64,
    policy_hash: &str,
    tsn_jitter_us: f64,
    avg_sar_wkg: f64,
    epoch_risk: f64,
) -> ChallengeBatch {
    let mut challenges = Vec::with_capacity(3);
    let mut idx: u64 = 0;

    if tsn_jitter_us > 1.0 {
        idx += 1;
        challenges.push(ChallengeSpecV1 {
            challenge_id: next_challenge_id(epoch, idx),
            epoch,
            policy_hash: policy_hash.to_string(),
            target_metric: "tsn_jitter_us".to_string(),
            target_constraint: "tsn_jitter_us <= 1.0".to_string(),
            reward_auet: 5_000,
            reward_csp: 500,
            penalty_auet: 1_000,
            penalty_csp: 100,
            verification_method: "TSN_ScheduleProof".to_string(),
            expires_epoch: epoch + 8,
            engine_tags: "Unreal:tsn-hardening,Unity:tsn-dungeon,Godot:tsn-grid".to_string(),
        });
    }

    if avg_sar_wkg > 0.2 {
        idx += 1;
        challenges.push(ChallengeSpecV1 {
            challenge_id: next_challenge_id(epoch, idx),
            epoch,
            policy_hash: policy_hash.to_string(),
            target_metric: "avg_sar_wkg".to_string(),
            target_constraint: "avg_sar_wkg <= 0.2".to_string(),
            reward_auet: 12_000,
            reward_csp: 2_000,
            penalty_auet: 3_000,
            penalty_csp: 600,
            verification_method: "SafetyEpochManifest".to_string(),
            expires_epoch: epoch + 12,
            engine_tags: "Unity:sar-lab,Unreal:sar-field,Godot:sar-sim".to_string(),
        });
    }

    if epoch_risk > 0.5 {
        idx += 1;
        challenges.push(ChallengeSpecV1 {
            challenge_id: next_challenge_id(epoch, idx),
            epoch,
            policy_hash: policy_hash.to_string(),
            target_metric: "epoch_risk".to_string(),
            target_constraint: "epoch_risk <= 0.5".to_string(),
            reward_auet: 2_000,
            reward_csp: 8_000,
            penalty_auet: 1_000,
            penalty_csp: 4_000,
            verification_method: "GovernanceAuditLog".to_string(),
            expires_epoch: epoch + 6,
            engine_tags: "Unreal:gov-ops,Unity:gov-lab,Godot:gov-trial".to_string(),
        });
    }

    ChallengeBatch {
        epoch,
        policy_hash: policy_hash.to_string(),
        challenges,
    }
}

pub fn apply_challenge_result(ch: &ChallengeSpecV1, success: bool, ledger: &mut Ledger) {
    let delta = if success {
        LedgerDelta {
            delta_auet: ch.reward_auet as i128,
            delta_csp: ch.reward_csp as i128,
        }
    } else {
        LedgerDelta {
            delta_auet: -(ch.penalty_auet as i128),
            delta_csp: -(ch.penalty_csp as i128),
        }
    };
    ledger.apply_delta(&delta);
}
