// path: src/cybernetic/risk/risk_sample_dataset.rs

use serde::{Deserialize, Serialize};

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskSampleMeta {
    pub version: String,
    pub schema_hash: String,
    pub namespace: String,
    pub origin: String,
    pub device_class: String,
    pub iec_62304_class: String,
    pub iso_14971_profile: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskSampleInstance {
    pub useridhash: String,
    pub sample: RiskSample,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskSampleBundle {
    pub schema: RiskSampleSchema,
    pub metadata: RiskSampleMeta,
    pub instances: Vec<RiskSampleInstance>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskSampleSchemaField {
    pub field: String,
    pub dtype: String,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskSampleSchema {
    pub fields: Vec<RiskSampleSchemaField>,
}

impl RiskSampleBundle {
    pub fn sample_bundle() -> Self {
        let schema = RiskSampleSchema {
            fields: vec![
                RiskSampleSchemaField {
                    field: "risk_score".into(),
                    dtype: "f64".into(),
                    description: "0.0..1.0 overall risk".into(),
                },
                RiskSampleSchemaField {
                    field: "ed_percent".into(),
                    dtype: "f64".into(),
                    description: "0.0..100.0 effective energy drain percent".into(),
                },
                RiskSampleSchemaField {
                    field: "sf_psych".into(),
                    dtype: "f64".into(),
                    description: ">=0.0 psych load factor".into(),
                },
                RiskSampleSchemaField {
                    field: "h_mod".into(),
                    dtype: "u64".into(),
                    description: "hash modulus derived from SHA3-512".into(),
                },
            ],
        };

        let metadata = RiskSampleMeta {
            version: "1.0.0".into(),
            schema_hash: "0x9A37C4F1".into(),
            namespace: "AU.BioAug.RiskSampleV1".into(),
            origin: "BioAugClinical-Regulator".into(),
            device_class: "III".into(),
            iec_62304_class: "C".into(),
            iso_14971_profile: "high_hazard".into(),
        };

        let instances = vec![
            RiskSampleInstance {
                useridhash: "0xC4F012A977B39D21".into(),
                sample: RiskSample {
                    risk_score: 0.23,
                    ed_percent: 23.0,
                    sf_psych: 0.41,
                    h_mod: 6_763_589_210_453_319_123,
                },
            },
            RiskSampleInstance {
                useridhash: "0x91A75B2200437EE1".into(),
                sample: RiskSample {
                    risk_score: 0.61,
                    ed_percent: 61.0,
                    sf_psych: 0.97,
                    h_mod: 11_593_348_772_044_100_211,
                },
            },
            RiskSampleInstance {
                useridhash: "0x7D3499E2A0F1FC33".into(),
                sample: RiskSample {
                    risk_score: 0.87,
                    ed_percent: 87.0,
                    sf_psych: 1.32,
                    h_mod: 3_377_449_185_012_299_344,
                },
            },
        ];

        Self {
            schema,
            metadata,
            instances,
        }
    }
}
