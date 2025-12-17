// path: internal/schema/nanoswarm_blood_node_profile.go

package schema

import (
	"encoding/hex"
	"time"
)

// BloodType encodes ABO/Rh plus interoperability codes. [file:1]
type BloodType struct {
	ID                 string `json:"id"`
	ABOGroup           string `json:"abo_group"`
	RhFactor           string `json:"rh_factor"`
	IsUniversalDonor   bool   `json:"is_universal_donor"`
	IsUniversalRecipient bool `json:"is_universal_recipient"`
	LoincCode          string `json:"loinc_code"`
	SnomedCTCode       string `json:"snomed_ct_code"`
}

// NanoswarmNode describes a high-density neuromorphic / QPU node profile. [file:1]
type NanoswarmNode struct {
	NodeID          string    `json:"node_id"`
	CohortID        string    `json:"cohort_id"`
	OpsThresholdTOPS float64  `json:"ops_threshold_tops"`
	QPUMeshDim      int       `json:"qpu_mesh_dim"`
	ThermalLimitW   float64   `json:"thermal_limit_w"`
	SupplyVoltageV  float64   `json:"supply_voltage_v"`
	MaxCurrentA     float64   `json:"max_current_a"`
	BioCompatArray  []float64 `json:"bio_compat_array"`
	LatencyProfileMs []float64 `json:"latency_profile_ms"`
	ComplianceLevel string    `json:"compliance_level"`
	AIFirmwareVersion string  `json:"ai_firmware_version"`
	TopologyMatrix  []byte    `json:"topology_matrix_hex"`
	CreatedAt       time.Time `json:"created_at_utc"`
}

// BloodAndNodeProfile is a join-ready projection for AI / policy engines. [file:1]
type BloodAndNodeProfile struct {
	Blood   BloodType     `json:"blood_type"`
	Node    NanoswarmNode `json:"nanoswarm_node"`
	Binding struct {
		// Deterministic mapping slot; not a foreign key in SQL but used by policy tiers.
		ProfileSlot string `json:"profile_slot"`
	} `json:"binding"`
}

// DecodeTopologyHex converts a hex string into the TopologyMatrix field. [file:1]
func (n *NanoswarmNode) DecodeTopologyHex(hexStr string) error {
	b, err := hex.DecodeString(hexStr)
	if err != nil {
		return err
	}
	n.TopologyMatrix = b
	return nil
}

// SampleBloodRows returns the eight canonical blood types. [file:1]
func SampleBloodRows() []BloodType {
	return []BloodType{
		{
			ID:                   "11111111-1111-4111-8111-111111111111",
			ABOGroup:             "O",
			RhFactor:             "NEG",
			IsUniversalDonor:     true,
			IsUniversalRecipient: false,
			LoincCode:            "77397-8",
			SnomedCTCode:         "278148006",
		},
		{
			ID:                   "22222222-2222-4222-8222-222222222222",
			ABOGroup:             "O",
			RhFactor:             "POS",
			IsUniversalDonor:     false,
			IsUniversalRecipient: false,
			LoincCode:            "77397-8",
			SnomedCTCode:         "278147001",
		},
		{
			ID:                   "33333333-3333-4333-8333-333333333333",
			ABOGroup:             "A",
			RhFactor:             "NEG",
			IsUniversalDonor:     false,
			IsUniversalRecipient: false,
			LoincCode:            "77397-8",
			SnomedCTCode:         "278152006",
		},
		{
			ID:                   "44444444-4444-4444-8444-444444444444",
			ABOGroup:             "A",
			RhFactor:             "POS",
			IsUniversalDonor:     false,
			IsUniversalRecipient: false,
			LoincCode:            "77397-8",
			SnomedCTCode:         "278149003",
		},
		{
			ID:                   "55555555-5555-4555-8555-555555555555",
			ABOGroup:             "B",
			RhFactor:             "NEG",
			IsUniversalDonor:     false,
			IsUniversalRecipient: false,
			LoincCode:            "77397-8",
			SnomedCTCode:         "278153001",
		},
		{
			ID:                   "66666666-6666-4666-8666-666666666666",
			ABOGroup:             "B",
			RhFactor:             "POS",
			IsUniversalDonor:     false,
			IsUniversalRecipient: false,
			LoincCode:            "77397-8",
			SnomedCTCode:         "278150003",
		},
		{
			ID:                   "77777777-7777-4777-8777-777777777777",
			ABOGroup:             "AB",
			RhFactor:             "NEG",
			IsUniversalDonor:     false,
			IsUniversalRecipient: false,
			LoincCode:            "77397-8",
			SnomedCTCode:         "278154007",
		},
		{
			ID:                   "88888888-8888-4888-8888-888888888888",
			ABOGroup:             "AB",
			RhFactor:             "POS",
			IsUniversalDonor:     false,
			IsUniversalRecipient: true,
			LoincCode:            "77397-8",
			SnomedCTCode:         "278151004",
		},
	}
}

// SampleNanoswarmNode returns the nanoswarm_node row with decoded topology hex. [file:1]
func SampleNanoswarmNode() (NanoswarmNode, error) {
	n := NanoswarmNode{
		NodeID:           "ffffffff-ffff-4fff-8fff-fffffffffff1",
		CohortID:         "dddddddd-dddd-4ddd-8ddd-ddddddddddd4",
		OpsThresholdTOPS: 1000.00,
		QPUMeshDim:       32,
		ThermalLimitW:    0.450,
		SupplyVoltageV:   2.300,
		MaxCurrentA:      18.00,
		BioCompatArray: []float64{
			0.98, 0.97, 0.96, 0.95,
			0.97, 0.96, 0.95, 0.94,
		},
		LatencyProfileMs: []float64{
			22.5, 21.8, 19.6, 18.9,
			23.2, 20.4, 19.1, 17.7,
		},
		ComplianceLevel:   "surgical-grade",
		AIFirmwareVersion: "ALN-QPU-FW-2.3.7",
		CreatedAt:         time.Date(2025, 12, 15, 6, 40, 0, 0, time.UTC),
	}
	const topoHex = "B4E2D7A1C9F0837AD1E4B6C3F9A2D5E1B7C8D9F0A3E6B1C4D7F2A9E0C5B8D3F6"
	if err := n.DecodeTopologyHex(topoHex); err != nil {
		return NanoswarmNode{}, err
	}
	return n, nil
}
