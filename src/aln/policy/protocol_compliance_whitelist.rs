// path: src/aln/policy/protocol_compliance_whitelist.rs

use serde::{Deserialize, Serialize};
use std::collections::HashSet;

/// Whitelisted protocol stack components for ALN artifacts.
/// Values are normalized, exact-match tokens. [file:1]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProtocolWhitelist {
    pub values: HashSet<String>,
}

/// Whitelisted compliance frameworks / standards for ALN artifacts. [file:1]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceWhitelist {
    pub values: HashSet<String>,
}

/// Static validation rules for ALN paths and fields. [file:1]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationRule {
    /// Glob-style target path (logical, not filesystem).
    pub target: String,
    /// Human-readable rule description, enforced by code.
    pub rule: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationConfig {
    pub protocol_whitelist: ProtocolWhitelist,
    pub compliance_whitelist: ComplianceWhitelist,
    pub rules: Vec<ValidationRule>,
}

impl ValidationConfig {
    /// Deterministic in-memory configuration derived from the CSV-style spec. [file:1]
    pub fn builtin() -> Self {
        let protocol_whitelist = ProtocolWhitelist {
            values: [
                "TLS1.3",
                "AES-256-GCM",
                "ChaCha20-Poly1305",
                "TSN",
                "PQC-Kyber",
                "IEEE-11073-SDC",
                "HL7-FHIR-R4",
                "QUIC-mTLS",
            ]
            .into_iter()
            .map(String::from)
            .collect(),
        };

        let compliance_whitelist = ComplianceWhitelist {
            values: [
                "GDPR",
                "NIST-CSF",
                "EU-AI-Act-2024",
                "HIPAA",
                "ISO27001",
                "IEC62304",
                "ISO14971",
                "EU-MDR",
            ]
            .into_iter()
            .map(String::from)
            .collect(),
        };

        let rules = vec![
            ValidationRule {
                target: "/aln/functions/*.aln".into(),
                rule: "protocol_stack MUST reference only PROTOCOL_WHITELIST entries joined by '+'"
                    .into(),
            },
            ValidationRule {
                target: "/aln/functions/*.aln".into(),
                rule: "compliance_check MUST be '+'-separated values all in COMPLIANCE_WHITELIST"
                    .into(),
            },
            ValidationRule {
                target: "/aln/datashards/*.aln".into(),
                rule: "compliance field MUST be '+'-separated subset of COMPLIANCE_WHITELIST"
                    .into(),
            },
            ValidationRule {
                target: "/aln/datasets/*.aln".into(),
                rule: "iec_62304_class MUST be one of 'A','B','C'".into(),
            },
            ValidationRule {
                target: "/aln/datasets/*.aln".into(),
                rule: "iso_14971_profile MUST be one of 'low_hazard','medium_hazard','high_hazard'"
                    .into(),
            },
        ];

        Self {
            protocol_whitelist,
            compliance_whitelist,
            rules,
        }
    }

    /// Validates a '+'-separated protocol_stack string against the whitelist. [file:1]
    pub fn validate_protocol_stack(&self, stack: &str) -> bool {
        if stack.is_empty() {
            return false;
        }
        stack
            .split('+')
            .all(|tok| self.protocol_whitelist.values.contains(tok))
    }

    /// Validates a '+'-separated compliance string against the whitelist. [file:1]
    pub fn validate_compliance_list(&self, list: &str) -> bool {
        if list.is_empty() {
            return false;
        }
        list.split('+')
            .all(|tok| self.compliance_whitelist.values.contains(tok))
    }

    /// Validates iec_62304_class ∈ {'A','B','C'}. [file:1]
    pub fn validate_iec_62304_class(&self, cls: &str) -> bool {
        matches!(cls, "A" | "B" | "C")
    }

    /// Validates iso_14971_profile ∈ {'low_hazard','medium_hazard','high_hazard'}. [file:1]
    pub fn validate_iso_14971_profile(&self, profile: &str) -> bool {
        matches!(profile, "low_hazard" | "medium_hazard" | "high_hazard")
    }
}
