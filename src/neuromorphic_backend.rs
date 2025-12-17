use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnergyReport {
    pub backend_name: String,
    pub spikes_in: u64,
    pub duration_ms: u32,
    pub energy_mj: f32,
    pub ed_percent: f32,
}

pub trait NeuromorphicBackend {
    fn name(&self) -> &'static str;
    fn infer(&self, spikes: &[u8]) -> Vec<f32>;
    fn energy_mj(&self) -> f32;
}

pub struct LoihiBackend {
    pub calibrated_energy_mj: f32,
}

pub struct AkidaBackend {
    pub calibrated_energy_mj: f32,
}

impl NeuromorphicBackend for LoihiBackend {
    fn name(&self) -> &'static str {
        "Loihi"
    }
    fn infer(&self, spikes: &[u8]) -> Vec<f32> {
        // In production, this forwards via gRPC to Loihi service.
        vec![spikes.len() as f32 * 0.01, 0.0, 0.0]
    }
    fn energy_mj(&self) -> f32 {
        self.calibrated_energy_mj
    }
}

impl NeuromorphicBackend for AkidaBackend {
    fn name(&self) -> &'static str {
        "AkidaEdge"
    }
    fn infer(&self, spikes: &[u8]) -> Vec<f32> {
        // In production, this binds to Akida SDK APIs.
        vec![spikes.len() as f32 * 0.008, 0.0, 0.0]
    }
    fn energy_mj(&self) -> f32 {
        self.calibrated_energy_mj
    }
}

pub enum NeuromorphicImpl {
    Loihi(LoihiBackend),
    Akida(AkidaBackend),
}

impl NeuromorphicBackend for NeuromorphicImpl {
    fn name(&self) -> &'static str {
        match self {
            NeuromorphicImpl::Loihi(_) => "Loihi",
            NeuromorphicImpl::Akida(_) => "AkidaEdge",
        }
    }

    fn infer(&self, spikes: &[u8]) -> Vec<f32> {
        match self {
            NeuromorphicImpl::Loihi(b) => b.infer(spikes),
            NeuromorphicImpl::Akida(b) => b.infer(spikes),
        }
    }

    fn energy_mj(&self) -> f32 {
        match self {
            NeuromorphicImpl::Loihi(b) => b.energy_mj(),
            NeuromorphicImpl::Akida(b) => b.energy_mj(),
        }
    }
}

pub fn energy_to_ed_percent(energy_mj: f32, baseline_mj: f32) -> f32 {
    if baseline_mj <= 0.0 {
        0.0
    } else {
        (energy_mj / baseline_mj * 100.0).clamp(0.0, 100.0)
    }
}

pub fn build_energy_report<B: NeuromorphicBackend>(
    backend: &B,
    spikes: &[u8],
    baseline_mj: f32,
    duration_ms: u32,
) -> EnergyReport {
    let _ = backend.infer(spikes);
    let e = backend.energy_mj();
    let ed = energy_to_ed_percent(e, baseline_mj);
    EnergyReport {
        backend_name: backend.name().to_string(),
        spikes_in: spikes.len() as u64,
        duration_ms,
        energy_mj: e,
        ed_percent: ed,
    }
}
