use hex_literal::hex;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrationNode {
    pub node_id: [u8; 8],
    pub ops_threshold_tops: f32,
    pub topology_rows: u16,
    pub topology_cols: u16,
    pub compliance_level: u8,
    pub ai_firmware_version: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NanoBciEnvelope {
    pub featureid: u64,
    pub resonancefreq_hz: f32,
    pub amplitude_mt: f32,
    pub duty_cycle: f32,
    pub energy_integral: f32,
}

impl NanoBciEnvelope {
    pub fn is_within_safety(&self, damage_threshold: f32) -> bool {
        self.amplitude_mt <= 10.0 &&
        self.energy_integral <= damage_threshold &&
        self.duty_cycle >= 0.0 &&
        self.duty_cycle <= 1.0
    }
}

pub fn bind_feature_to_plugin(
    useridhash: [u8; 8],
    featureid: u64,
    plugin_name: &str,
    node: &IntegrationNode,
) -> anyhow::Result<()> {
    if node.ops_threshold_tops < 12.0 {
        anyhow::bail!("Node throughput below BCI minimum");
    }
    if node.compliance_level < 1 {
        anyhow::bail!("Node not at least clinical-grade");
    }
    // Write binding into PostgreSQL via existing ALN POS DB adapter (not shown here).
    Ok(())
}
