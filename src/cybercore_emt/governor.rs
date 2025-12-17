// src/cybercore_emt/governor.rs
// Production EMT control-plane for AU.CyberCore.EnergyMassTissue.v1

#![no_std]

use core::cmp::Ordering;

#[repr(C)]
pub struct EMTEnvelope {
    pub envelopeid: u64,
    pub featureid: u64,
    pub useridhash: u64,
    pub timestampms: u64,
    pub power_mw: f32,
    pub eff_mass_kg: f32,
    pub tissue_depth_mm: f32,
    pub energy_density_mjmm3: f32,
    pub duty_cycle_pct: f32,
    pub actuation_freq_hz: f32,
    pub dose_1s_mjmm3: f32,
    pub dose_60s_mjmm3: f32,
    pub dose_1h_mjmm3: f32,
    pub limit_energy_mjmm3: f32,
    pub limit_duty_pct: f32,
    pub limit_freq_hz: f32,
    pub limit_temp_delta_c: f32,
    pub thermal_risk01: f32,
    pub mechanical_risk01: f32,
    pub neuro_risk01: f32,
    pub policyprofileid: u32,
    pub violation_flag: u8,
    pub _reserved: [u8; 7],
}

#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum GovernanceState {
    GovNormal      = 0,
    GovWarn        = 1,
    GovLocked      = 2,
    GovQuarantined = 3,
}

#[repr(C)]
pub struct GovernanceSnapshot {
    pub snapshotid: u64,
    pub featureid: u64,
    pub useridhash: u64,
    pub timestampms: u64,
    pub governance_state: u8,
    pub dominant_risk_axis: u8,
    pub combined_risk01: f32,
    pub require_halt: u8,
    pub require_derate: u8,
    pub derate_factor: f32,
    pub policyprofileid: u32,
    pub securitysnapshotid: u64,
    pub audit_eventid: u64,
    pub _reserved: [u8; 8],
}

pub struct PolicyProfile {
    pub policyprofileid: u32,
    pub max_energy_mjmm3: f32,
    pub max_duty_pct: f32,
    pub max_freq_hz: f32,
    pub max_temp_delta_c: f32,
    pub warn_risk_level: f32,
    pub lock_risk_level: f32,
    pub audit_risk_level: f32,
}

#[inline]
fn clamp01(x: f32) -> f32 {
    if x < 0.0 { 0.0 } else if x > 1.0 { 1.0 } else { x }
}

pub fn update_risks(env: &mut EMTEnvelope) {
    // Thermal risk
    if env.limit_energy_mjmm3 > 0.0 {
        env.thermal_risk01 =
            clamp01(env.energy_density_mjmm3 / env.limit_energy_mjmm3);
    } else {
        env.thermal_risk01 = 0.0;
    }

    // Mechanical risk
    if env.limit_duty_pct > 0.0 {
        env.mechanical_risk01 =
            clamp01(env.duty_cycle_pct / env.limit_duty_pct);
    } else {
        env.mechanical_risk01 = 0.0;
    }

    // Neuro risk
    if env.limit_freq_hz > 0.0 {
        env.neuro_risk01 =
            clamp01(env.actuation_freq_hz / env.limit_freq_hz);
    } else {
        env.neuro_risk01 = 0.0;
    }
}

pub fn evaluate_governance(
    env: &mut EMTEnvelope,
    policy: &PolicyProfile,
    snapshot_id: u64,
    now_ms: u64,
    securitysnapshotid: u64,
    audit_eventid: u64,
) -> GovernanceSnapshot {
    update_risks(env);

    let mut dominant_axis: u8 = 1;
    let mut risk = env.thermal_risk01;

    if env.mechanical_risk01 > risk {
        risk = env.mechanical_risk01;
        dominant_axis = 2;
    }
    if env.neuro_risk01 > risk {
        risk = env.neuro_risk01;
        dominant_axis = 3;
    }

    let mut state = GovernanceState::GovNormal;
    let mut require_halt: u8 = 0;
    let mut require_derate: u8 = 0;
    let mut derate_factor: f32 = 1.0;

    // Enforce universal ceilings (governance equalizer)
    match risk.partial_cmp(&policy.lock_risk_level).unwrap_or(Ordering::Less) {
        Ordering::Greater | Ordering::Equal => {
            state = GovernanceState::GovLocked;
            require_halt = 1;
            require_derate = 0;
            derate_factor = 0.0;
            env.violation_flag = 2;
        }
        Ordering::Less => {
            if risk >= policy.warn_risk_level {
                state = GovernanceState::GovWarn;
                require_halt = 0;
                require_derate = 1;
                let span = (policy.lock_risk_level - policy.warn_risk_level)
                    .max(0.01);
                let frac = clamp01((risk - policy.warn_risk_level) / span);
                derate_factor = 0.7 - frac * 0.4; // 0.7 -> 0.3
                env.violation_flag = 1;
            } else {
                state = GovernanceState::GovNormal;
                require_halt = 0;
                require_derate = 0;
                derate_factor = 1.0;
                env.violation_flag = 0;
            }
        }
    }

    GovernanceSnapshot {
        snapshotid:       snapshot_id,
        featureid:        env.featureid,
        useridhash:       env.useridhash,
        timestampms:      now_ms,
        governance_state: state as u8,
        dominant_risk_axis: dominant_axis,
        combined_risk01:  risk,
        require_halt,
        require_derate,
        derate_factor,
        policyprofileid:  env.policyprofileid,
        securitysnapshotid,
        audit_eventid,
        _reserved:        [0u8; 8],
    }
}
