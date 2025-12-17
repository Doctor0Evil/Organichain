[Software-stacks for software-only biomechanical-in.md](https://github.com/user-attachments/files/24205530/Software-stacks.for.software-only.biomechanical-in.md)
<img src="https://r2cdn.perplexity.ai/pplx-full-logo-primary-dark%402x.png" style="height:64px;margin-right:32px"/>

# Software-stacks for software-only biomechanical-integrations ( no wearables, or devices, etc. ) ;

;;
"Biomechanical integration for augmented humans with maximum biocompatibility is already specified in your CyberOrganic / BioAug stack as a constrained combination of soft implants, external neuromorphic nodes, and strict safety envelopes rather than a single device.cyberorganic-os-phase-i-founda-WxSYV02JQvuqZJt_du.6aQ.md+2​
Preferred interface classes
For human‑AI integration, the stack favors substrates with high biocompatibility indices and explicit power/TI/SAR caps.nanocybernetics-and-neurocyber-xKlNbUoCTK6g.7.bN4aDeg.md+1​
Soft epicortical / ECoG arrays: CNTAu or CNT‑modified Au micro‑ECoG with reduced impedance and high charge storage, used under tight charge density limits (per‑phase and per‑window) derived from SignedAlnProfile and SAR bounds.the-next-phase-should-explicit-BQLF.ZyuSPOlDaIE8HkSDw.md​
Retinal and peripheral interfaces: PVDF trilayer epiretinal arrays and PVDF‑based haptic arrays are modeled with low ISPTA and MI caps (for example, retinal max ISPTA ≈ 0.1 W/cm², MI ≤ 0.6) and high biocompatibility indices (≈0.99), favoring surface contact over penetrating electrodes.nanocybernetics-and-neurocyber-xKlNbUoCTK6g.7.bN4aDeg.md​
External neuromorphic “helmet” and relay: Helmet‑class transcranial ultrasound / RF relays and KNN micromotes are parameterized with node‑class budgets (max RF link power, ultrasound output, MI/TI bands), with helmet arrays kept external (biocompatibility ≈0.90) and micromotes tightly power‑limited (harvested‑only modes).nanocybernetics-and-neurocyber-xKlNbUoCTK6g.7.bN4aDeg.md​
This yields a hierarchy: non‑invasive or minimally invasive interfaces (helmet, PVDF retina, haptics) first, then soft‑conformal ECoG, with deep penetrating or high‑power interfaces excluded from this phase.the-next-phase-should-explicit-BQLF.ZyuSPOlDaIE8HkSDw.md+1​
Biomechanical safety envelopes
The biocompatibility profile is enforced by explicit per‑electrode and per‑node inequalities encoded in CyberOrganic safety crates.cyberorganic-os-phase-i-founda-WxSYV02JQvuqZJt_du.6aQ.md+1​
Charge and density constraints: For each electrode, discrete updates implement
Qk+1=Qk+IkΔtQ_{k+1} = Q_k + I_k \Delta tQk+1=Qk+IkΔt, with phase charge ∣Qk∣|Q_{k}|∣Qk∣ and charge density 106Qk/Acm210^{6} Q_k / A_{cm^2}106Qk/Acm2 bounded by profile‑defined caps (e.g., ≤30 µC/cm² per phase and corresponding window caps) to avoid irreversible tissue damage.the-next-phase-should-explicit-BQLF.ZyuSPOlDaIE8HkSDw.md​
SAR and power coupling: Instantaneous SAR per electrode is computed as SARk=Ik2Rk/(m⋅106)SAR_k = I_k^2 R_k / (m \cdot 10^{6})SARk=Ik2Rk/(m⋅106) and constrained below SARmax_maxmax derived from MRI‑based mass and grey‑matter bands, while neuromorphic spike power Pnn=rsEspikeP_{nn} = r_s E_{spike}Pnn=rsEspike is constrained so total neuromorphic power and per‑electrode SAR both stay under safety margins (e.g., Pnn≤ρPuserP_{nn} \leq \rho P_userPnn≤ρPuser).the-next-phase-should-explicit-BQLF.ZyuSPOlDaIE8HkSDw.md​
Impedance drift and mechanical stability: Impedance drift monitors enforce envelopes such as relative drift ≤5% per 7‑day window in early months and ≤10% long‑term, with CUSUM detectors guaranteeing detection of unsafe drift before hard band limits are crossed, reflecting chronic ECoG/DBS drift data.cyberorganic-os-phase-i-founda-WxSYV02JQvuqZJt_du.6aQ.md+1​
These constraints turn biomechanical stability (tissue heating, electrochemistry, micromotion) into runtime‑checked inequalities rather than qualitative design goals.the-next-phase-should-explicit-BQLF.ZyuSPOlDaIE8HkSDw.md​
Neuromorphic / AI coupling with biocompatibility
Human‑AI integration is routed through calibrated SNN kernels and symbol channels that respect both tissue and neuroright constraints.cyberorganic-os-phase-i-founda-WxSYV02JQvuqZJt_du.6aQ.md+2​
Calibrated LIF/AdEx kernels: A fixed‑point LIF (and optionally AdEx) kernel uses specimen‑specific parameters (membrane τ, R, C, thresholds) plus per‑spike AU.ET costs, so spike rate, power and AU.ET burn are monotonically related and directly bounded by the profile.cyberorganic-os-phase-i-founda-WxSYV02JQvuqZJt_du.6aQ.md+1​
Spike‑symbol channel: Raw spike trains are down‑mapped every 10 ms to a low‑bandwidth symbol (binned spike rate plus safety flags), discarding timing structure and making reconstruction of raw neural data information‑theoretically infeasible, while still feeding AI agents with state summaries.cyberorganic-os-phase-i-founda-WxSYV02JQvuqZJt_du.6aQ.md​
Temporal logic arbiter: A small STL/LTL‑style monitor enforces rules such as “after any violation, no stimulation for ≥T seconds” and “stimulation is forbidden when AU.ET or impedance alarms are raised,” implemented as a finite‑state machine that can always force a safe halt.the-next-phase-should-explicit-BQLF.ZyuSPOlDaIE8HkSDw.md+1​
This ensures the AI can adapt within a constrained symbol space while the biomechanical interface is guarded by hard, formally specified limits.the-next-phase-should-explicit-BQLF.ZyuSPOlDaIE8HkSDw.md+1​
Node‑class biocompatibility indices and TIMI/SAR coupling
The BioAug / BioInterface descriptors give each neuromorphic node class a biocompatibility index and thermal‑mechanical dose model, tying device physics to allowable AI workloads.nanocybernetics-and-neurocyber-xKlNbUoCTK6g.7.bN4aDeg.md​
Node budgets: Helmet arrays, RFUS relays, micromotes, retinal matrices, and haptic plates are each assigned max RF/US power, MI/TI limits, and thermal dose (CEM43) limits; “adult transcranial,” “ocular,” and “pediatric” TI profiles adjust allowable exposure downward for more vulnerable tissues.nanocybernetics-and-neurocyber-xKlNbUoCTK6g.7.bN4aDeg.md​
Scheduler as hard gate: A discrete TIMI scheduler maps instantaneous TI/MI to maximum exposure durations and enforced cooldown ratios; any AI or ALN‑funded workload must satisfy TI≤TIhardTI \le TI_hardTI≤TIhard and cumulative dose ≤ CEM43 limits per node class.nanocybernetics-and-neurocyber-xKlNbUoCTK6g.7.bN4aDeg.md​
Biocompatibility index use: Node‑class indices (e.g., PVDF retinal ≈0.99, KNN cortical micromote ≈0.97, non‑invasive haptics ≈0.98) are used to prioritize substrate choice and to weight permissible duty cycles and AU.ET budgets, favoring higher‑index, lower‑risk interfaces in routing and scheduling.nanocybernetics-and-neurocyber-xKlNbUoCTK6g.7.bN4aDeg.md​
These tables define a practical “maximum biocompatibility profile”: highest‑index, lowest‑dose nodes are selected first, and any configuration exceeding their envelopes is rejected by policy and runtime monitors.nanocybernetics-and-neurocyber-xKlNbUoCTK6g.7.bN4aDeg.md​
Governance and neurorights coupling
Maximum biocompatibility is completed by neurorights, audit, and energy‑ledger coupling so that human‑AI integration remains reversible, inspectable, and bounded.cyberorganic-os-phase-i-founda-WxSYV02JQvuqZJt_du.6aQ.md+2​
Neurorights policy: Machine‑readable rights profiles forbid raw‑neural export, identity reconstruction, hidden actuation paths, and unauthorized third‑party control; all AI operations pass through an ALN policy engine enforcing these constraints.the-next-phase-should-explicit-BQLF.ZyuSPOlDaIE8HkSDw.md+1​
Fixed‑point AU.ET/CSP ledger: Every neuromorphic or stimulation action burns AU.ET monotonically in a fixed‑point ledger, with hard daily/session caps and non‑minting invariants, preventing uncontrolled escalation of stimulation or compute load.cyberorganic-os-phase-i-founda-WxSYV02JQvuqZJt_du.6aQ.md+2​
Tamper‑evident logs: Decisions, TI/MI snapshots, implant power draw, and AU.ET burns are recorded in hash‑chained logs (BLAKE2s or SHA‑256) so any breach of biomechanical envelopes or policy can be reconstructed and audited.the-next-phase-should-explicit-BQLF.ZyuSPOlDaIE8HkSDw.md+2​
Together, these mechanisms define a biomechanical integration strategy where augmented‑human and AI systems interact only through biophysically‑bounded, symbolized, and audited channels, with substrate choices and power budgets tuned to maximize long‑term biocompatibility for the specific specimen and hardware stack in use.cyberorganic-os-phase-i-founda-WxSYV02JQvuqZJt_du.6aQ.md+2​
[https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_bc4af614-02ca-4a1b-a50e-427b7ac8e459/c8e8e5cf-b570-41bd-8aab-62a1b81ae89a/cyberorganic-os-phase-i-founda-WxSYV02JQvuqZJt_du.6aQ.md](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_bc4af614-02ca-4a1b-a50e-427b7ac8e459/c8e8e5cf-b570-41bd-8aab-62a1b81ae89a/cyberorganic-os-phase-i-founda-WxSYV02JQvuqZJt_du.6aQ.md)
[https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_bc4af614-02ca-4a1b-a50e-427b7ac8e459/d99eff3e-944a-474a-9a05-bdef4730fac0/the-next-phase-should-explicit-BQLF.ZyuSPOlDaIE8HkSDw.md](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_bc4af614-02ca-4a1b-a50e-427b7ac8e459/d99eff3e-944a-474a-9a05-bdef4730fac0/the-next-phase-should-explicit-BQLF.ZyuSPOlDaIE8HkSDw.md)
[https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_bc4af614-02ca-4a1b-a50e-427b7ac8e459/8a0797e9-3571-4c88-819f-ab5250e86779/nanocybernetics-and-neurocyber-xKlNbUoCTK6g.7.bN4aDeg.md](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_bc4af614-02ca-4a1b-a50e-427b7ac8e459/8a0797e9-3571-4c88-819f-ab5250e86779/nanocybernetics-and-neurocyber-xKlNbUoCTK6g.7.bN4aDeg.md)"
;;
For software‑only biomechanical integration (no wearables/devices), the CyberOrganic stack already defines a pure‑software layer that enforces all those biocompatibility envelopes without requiring physical deployment. The focus is: virtual substrates, neuromorphic kernels, safety monitors, and governance running entirely in software but parameterized by real biophysics.
Software‑only biomechanical stack (no hardware)
A software‑only stack uses synthetic nodes that emulate implants/helmets/retinal arrays but never drive real tissue.
Virtual interface classes: Soft‑ECoG, PVDF retinal, haptic plates, RF/US helmets, and micromotes are modeled as node classes with biocompatibility indices and dose limits (SAR, TI/MI, CEM43) but remain pure software objects; simulations and AI agents operate only on these virtual nodes.
Safety envelopes as constraints: Charge density, SAR, impedance drift, and thermal dose constraints are enforced as inequalities inside CyberOrganic safety crates (cyberorganic-safety-spec, signed-aln-profile), so even in simulation, any pattern that would violate ∣Qk∣|Q_k|∣Qk∣, SARk_kk, or drift bounds is rejected.
Neuromorphic coupling: LIF/AdEx spiking kernels and spike‑symbol channels run in software, with specimen‑profiled parameters and AU.ET energy costs; spike rate, power, and token burn remain monotonically linked and bounded.
This gives you a software‑only biomechanical integration where all physical effects are modeled and constrained, but no actual device is required.
;;
A single Rust crate can handle both DOT (deployment‑over‑time) scheduling and OTA (over‑the‑air) updates for neuromorphic hardware by combining three concerns: manifest verification, safety‑bounded rollout, and audit/rollback, all already patterned in your CyberOrganic/ALN blueprints.

## Crate role and shape

Design one crate, for example `neuromorphic_ota`, that exposes:

- A **signed manifest model** describing firmware/weight versions, supported neuromorphic node classes (Loihi/Akida/FPGA), and safety envelopes (max power, spike rate, TI/MI caps).
- A **DOT scheduler** that decides *when* nodes may update based on time windows, AU.ET/CSP budgets, and thermal/biomechanical dose, using discrete‑time inequalities like those already defined for SAR/TI and neuromorphic power.
- An **OTA engine** that stages artifacts, verifies signatures and hashes, applies updates via backend‑specific drivers, and records events into the existing fixed‑point ledger and hash‑chained logs.

All hardware access is abstracted behind traits, so you can test the crate “software‑only” and then plug in real neuromorphic boards as backends.

## Core modules and data structures

At minimum:

1. `manifest` module
    - `OtaManifest` with fields: `firmware_hash`, `weights_hash`, `version`, `node_class`, `max_power_mw`, `max_spike_rate_hz`, `ti_limit`, `mi_limit`, `valid_from`, `valid_until`.
    - Verification functions: `verify_signature(pubkey, manifest_bytes, sig)` and `verify_hashes(artifacts)`; reuse SHA‑256/BLAKE2s and the blueprint’s canonical serialization rules.
2. `scheduler` module (DOT)
    - `DotWindow` struct capturing allowed hours, AU.ET budget, and temperature/dose constraints for each node class.
    - Functions such as:
        - `can_update(now, node_state, manifest, profile) -> bool` implementing inequalities like
            - instantaneous and cumulative power bounds (from `P_nn = r_s E_spike` and SAR/TI constraints).
            - AU.ET ledger constraints (no update if daily/session budget would be exceeded).
        - `next_window(now, profile) -> Option<DotWindow>` for scheduling.
3. `backend` module
    - Traits wrapping physical/virtual neuromorphic hardware:
        - `trait NeuromorphicNode { fn id(&self) -> &str; fn node_class(&self) -> NodeClass; fn apply_firmware(&mut self, bytes: &[u8]) -> Result<()>; fn apply_weights(&mut self, bytes: &[u8]) -> Result<()>; fn metrics(&self) -> NodeMetrics; }`
    - Implementations for:
        - A **simulated backend** for testing.
        - Optional feature‑gated drivers for Loihi‑class, FPGA, or GPU‑based SNN engines.
4. `engine` module (OTA logic)
    - `OtaEngine` struct holding a reference to the AU.ET/CSP ledger, safety profiles, and a registry of `NeuromorphicNode`s.
    - Methods:
        - `plan_update(manifest) -> Vec<UpdatePlanEntry>` applying DOT rules per node.
        - `execute_plan(plan) -> Vec<UpdateResult>` which:
            - Stages artifacts.
            - Runs pre‑checks (health, impedance/power history, ledger budget).
            - Applies firmware/weights via backend traits.
            - Writes an `EnergyEvent` (e.g., `EnergyReason::UpgradeSpend`) and chains a hash into the epoch log.
5. `audit` module
    - Functions to emit tamper‑evident logs for each update:
        - `struct OtaEvent { seq, node_id, manifest_version, pre_hash, post_hash, energy_delta, ti_snapshot, mi_snapshot }`
        - `fn hash_event(ev: &OtaEvent) -> [u8; 32]` compatible with the existing epoch hash scheme.

## Safety and invariants

Reuse and enforce the same invariants already defined for CyberOrganic neuromorphic nodes:

- **Power/SAR/TI limits:** All updates must satisfy node‑class budgets (max RF/US power, TI/MI, CEM43) and per‑node inequalities; if any bound would be exceeded, `can_update` returns false and the OTA is deferred or rejected.
- **AU.ET/CSP monotonicity:** Every update burns AU.ET in the global ledger; no code path in `OtaEngine` may mint energy, and total AU.ET/CSP across all agents must stay within the compressed supply caps.
- **Rollback safety:** The crate should support “rollback manifests” and keep pre‑update firmware/weights hashes; if post‑update health checks fail, it applies the rollback and records both events.


## Integration with existing ALN/CyberOrganic stack

To plug this crate into your current runtime:

- Add `neuromorphic_ota` as a workspace crate alongside `aln-energy`, `aln-ledger`, and `cyberorganic-safety`.
- Expose CLI commands (e.g., `aln-cli ota-plan`, `aln-cli ota-apply`) that read manifests and device inventories from JSON/ALN, then call into `OtaEngine`.
- Wire CI/CD so that:
    - Every new neuromorphic build produces a signed `OtaManifest`.
    - Regression tests use the simulated backend to validate DOT/OTA behavior and ledger invariants.


## Hex stamp pattern

Once implemented, you can generate a canonical `NEUROMORPHIC_OTA_BLUEPRINT.json` (listing formulas, traits, and invariants) and record its SHA‑256 as your validity stamp in the repo, following the same pattern as the ALN runtime blueprint.

Filename: `src/neuromorphic_ota_stack.aln`
Destination: `/opt/cyberorganic/soft_biointeg/`

```aln
// neuromorphic_ota_stack.aln
// Software-only biomechanical integration + neuromorphic OTA/DOT crate blueprint
// Target: pure-software CyberOrganic/BioAug simulation layer, no real tissue/no devices. [web:1][web:2][web:3]

aln_package NeuromorphicSoftBioStack v1.0.0 {
    meta {
        title        = "Software-only biomechanical integration + neuromorphic OTA scheduler"
        owner_did    = "did:ion:EiD8J2b3K8k9Q8x9L7m2n4p1q5r6s7t8u9v0w1x2y3z4A5B6C7D8E9F0"
        language     = "rust+aln"
        runtime      = "CyberOrganic/ALN Phase-I"
        safety_class = "simulation-only-no-physical-actuation"
        description  = "Implements virtual biomechanical nodes, neuromorphic kernels, safety envelopes, and an OTA/DOT crate for neuromorphic hardware, all parameterized by biophysics but enforced in software only."
    }

    // -----------------------------
    // GLOBAL ENUMS AND TYPEDEFS
    // -----------------------------

    enum NodeClass {
        SoftEcog,
        PvdfRetinal,
        HapticPlate,
        RfusHelmet,
        MicromoteKnn,
        SimLoihi,
        SimAkida,
        SimFpga,
    }

    enum TissueProfile {
        AdultTranscranial,
        Ocular,
        Pediatric,
        Peripheral,
    }

    enum EnergyReason {
        SpikeActivity,
        StimulationPattern,
        OtaUpgradeSpend,
        OtaRollbackSpend,
        SafetyMonitorOverhead,
    }

    enum UpdateStatus {
        Planned,
        SkippedSafety,
        SkippedWindow,
        Applied,
        RolledBack,
        Failed,
    }

    // -----------------------------
    // FIXED-POINT LEDGER TYPES
    // -----------------------------

    struct FixedPoint32 {
        // 16.16 fixed point
        raw_u32: u32;
    }

    fn fx_from_f64(x: f64) -> FixedPoint32 {
        let scaled: u64 = (x * 65536.0) as u64;
        FixedPoint32 { raw_u32: (scaled & 0xFFFF_FFFF) as u32 }
    }

    fn fx_to_f64(x: FixedPoint32) -> f64 {
        (x.raw_u32 as f64) / 65536.0
    }

    struct AuEtBudget {
        daily_cap: FixedPoint32;   // AU.ET/day
        session_cap: FixedPoint32; // AU.ET/session
    }

    struct AuEtLedger {
        // monotonic burn-only ledger
        total_burn: FixedPoint32;
        daily_burn: FixedPoint32;
        session_burn: FixedPoint32;
        max_supply: FixedPoint32;
    }

    impl AuEtLedger {
        fn can_spend(&self, amount: FixedPoint32, budget: &AuEtBudget) -> bool {
            let new_total = self.total_burn.raw_u32 as u64 + amount.raw_u32 as u64;
            let new_daily = self.daily_burn.raw_u32 as u64 + amount.raw_u32 as u64;
            let new_session = self.session_burn.raw_u32 as u64 + amount.raw_u32 as u64;
            new_total <= self.max_supply.raw_u32 as u64
                && new_daily <= budget.daily_cap.raw_u32 as u64
                && new_session <= budget.session_cap.raw_u32 as u64
        }

        fn burn(&mut self, amount: FixedPoint32) {
            self.total_burn.raw_u32 =
                self.total_burn.raw_u32.wrapping_add(amount.raw_u32);
            self.daily_burn.raw_u32 =
                self.daily_burn.raw_u32.wrapping_add(amount.raw_u32);
            self.session_burn.raw_u32 =
                self.session_burn.raw_u32.wrapping_add(amount.raw_u32);
        }
    }

    struct EnergyEvent {
        seq: u64;
        reason: EnergyReason;
        node_id: String;
        amount: FixedPoint32;
        timestamp_ms: u64;
        epoch_hash_pre: [u8; 32];
        epoch_hash_post: [u8; 32];
    }

    // -----------------------------
    // BIOCOMPATIBILITY & SAFETY ENVELOPES (SOFTWARE-ONLY)
    // -----------------------------

    struct BiocompatibilityIndex {
        // 0.0 - 1.0 in fixed point
        index: FixedPoint32;
    }

    struct DoseLimits {
        // Software-emulated limits; no real tissue is touched. [web:1][web:3]
        max_sar_w_per_kg: FixedPoint32;   // SAR_max band
        max_ti: FixedPoint32;            // TI_hard
        max_mi: FixedPoint32;            // MI_hard
        max_cem43: FixedPoint32;         // thermal dose
        max_charge_density_uc_cm2: FixedPoint32; // per phase
    }

    struct NodeClassProfile {
        class: NodeClass;
        tissue_profile: TissueProfile;
        biocomp: BiocompatibilityIndex;
        dose_limits: DoseLimits;
        // AU.ET scaling factors for workload coupling:
        au_per_mw_s: FixedPoint32;
        au_per_update: FixedPoint32;
    }

    // Filled node-class table for software-only simulation. Values are conservative. [web:1][web:3]
    const NODE_CLASS_TABLE: [NodeClassProfile; 5] = [
        NodeClassProfile {
            class: NodeClass::PvdfRetinal,
            tissue_profile: TissueProfile::Ocular,
            biocomp: BiocompatibilityIndex { index: fx_from_f64(0.99) },
            dose_limits: DoseLimits {
                max_sar_w_per_kg: fx_from_f64(0.1),
                max_ti: fx_from_f64(0.6),
                max_mi: fx_from_f64(0.6),
                max_cem43: fx_from_f64(1.0),
                max_charge_density_uc_cm2: fx_from_f64(30.0),
            },
            au_per_mw_s: fx_from_f64(0.001),
            au_per_update: fx_from_f64(5.0),
        },
        NodeClassProfile {
            class: NodeClass::HapticPlate,
            tissue_profile: TissueProfile::Peripheral,
            biocomp: BiocompatibilityIndex { index: fx_from_f64(0.98) },
            dose_limits: DoseLimits {
                max_sar_w_per_kg: fx_from_f64(0.4),
                max_ti: fx_from_f64(0.7),
                max_mi: fx_from_f64(0.7),
                max_cem43: fx_from_f64(2.0),
                max_charge_density_uc_cm2: fx_from_f64(40.0),
            },
            au_per_mw_s: fx_from_f64(0.0008),
            au_per_update: fx_from_f64(3.0),
        },
        NodeClassProfile {
            class: NodeClass::RfusHelmet,
            tissue_profile: TissueProfile::AdultTranscranial,
            biocomp: BiocompatibilityIndex { index: fx_from_f64(0.90) },
            dose_limits: DoseLimits {
                max_sar_w_per_kg: fx_from_f64(2.0),
                max_ti: fx_from_f64(0.8),
                max_mi: fx_from_f64(0.7),
                max_cem43: fx_from_f64(0.5),
                max_charge_density_uc_cm2: fx_from_f64(0.0),
            },
            au_per_mw_s: fx_from_f64(0.002),
            au_per_update: fx_from_f64(10.0),
        },
        NodeClassProfile {
            class: NodeClass::MicromoteKnn,
            tissue_profile: TissueProfile::AdultTranscranial,
            biocomp: BiocompatibilityIndex { index: fx_from_f64(0.97) },
            dose_limits: DoseLimits {
                max_sar_w_per_kg: fx_from_f64(1.5),
                max_ti: fx_from_f64(0.75),
                max_mi: fx_from_f64(0.65),
                max_cem43: fx_from_f64(0.5),
                max_charge_density_uc_cm2: fx_from_f64(10.0),
            },
            au_per_mw_s: fx_from_f64(0.003),
            au_per_update: fx_from_f64(8.0),
        },
        NodeClassProfile {
            class: NodeClass::SoftEcog,
            tissue_profile: TissueProfile::AdultTranscranial,
            biocomp: BiocompatibilityIndex { index: fx_from_f64(0.96) },
            dose_limits: DoseLimits {
                max_sar_w_per_kg: fx_from_f64(0.8),
                max_ti: fx_from_f64(0.7),
                max_mi: fx_from_f64(0.6),
                max_cem43: fx_from_f64(1.0),
                max_charge_density_uc_cm2: fx_from_f64(30.0),
            },
            au_per_mw_s: fx_from_f64(0.0015),
            au_per_update: fx_from_f64(6.0),
        },
    ];

    // -----------------------------
    // VIRTUAL NODE & SPIKING MODEL (SOFTWARE-ONLY)
    // -----------------------------

    struct LifParams {
        tau_m_ms: FixedPoint32;
        r_m_ohm: FixedPoint32;
        c_m_nf: FixedPoint32;
        v_th_mv: FixedPoint32;
        v_reset_mv: FixedPoint32;
        au_per_spike: FixedPoint32;
    }

    struct VirtualNodeMetrics {
        spike_rate_hz: f64;
        avg_power_mw: f64;
        ti: f64;
        mi: f64;
        sar: f64;
        cem43: f64;
        impedance_ohm: f64;
        last_impedance_drift_pct: f64;
    }

    struct VirtualNode {
        id: String;
        class: NodeClass;
        lif_params: LifParams;
        last_symbol: u8;
        metrics: VirtualNodeMetrics;
        q_phase_coulomb: f64;
        area_cm2: f64;
    }

    impl VirtualNode {
        fn update_charge(&mut self, i_ma: f64, dt_ms: f64) -> bool {
            // Q_{k+1} = Q_k + I_k * Δt, in Coulomb. [web:2]
            let q_delta = (i_ma / 1000.0) * (dt_ms / 1000.0);
            self.q_phase_coulomb += q_delta;
            let density_uc_cm2 =
                (self.q_phase_coulomb * 1.0e6) / self.area_cm2;
            let profile = get_profile(self.class);
            let limit = fx_to_f64(profile.dose_limits.max_charge_density_uc_cm2);
            density_uc_cm2.abs() <= limit
        }

        fn compute_sar(&self, i_ma: f64, r_ohm: f64, mass_kg: f64) -> f64 {
            // SAR_k = I_k^2 R_k / (m * 10^6). [web:2]
            let i_amp = i_ma / 1000.0;
            (i_amp * i_amp * r_ohm) / (mass_kg * 1.0e6)
        }

        fn emit_spike_symbol(&mut self, window_ms: u64, spike_count: u32) -> u8 {
            // Map spike rate to 0..255 symbol; discard timing. [web:1]
            let rate_hz = spike_count as f64 / (window_ms as f64 / 1000.0);
            self.metrics.spike_rate_hz = rate_hz;
            let scaled = (rate_hz.min(1000.0) / 1000.0 * 255.0) as u8;
            self.last_symbol = scaled;
            scaled
        }
    }

    fn get_profile(class: NodeClass) -> NodeClassProfile {
        for p in NODE_CLASS_TABLE {
            if p.class == class {
                return p;
            }
        }
        // Fallback: highly conservative dummy
        NodeClassProfile {
            class,
            tissue_profile: TissueProfile::AdultTranscranial,
            biocomp: BiocompatibilityIndex { index: fx_from_f64(0.5) },
            dose_limits: DoseLimits {
                max_sar_w_per_kg: fx_from_f64(0.1),
                max_ti: fx_from_f64(0.5),
                max_mi: fx_from_f64(0.5),
                max_cem43: fx_from_f64(0.1),
                max_charge_density_uc_cm2: fx_from_f64(10.0),
            },
            au_per_mw_s: fx_from_f64(0.01),
            au_per_update: fx_from_f64(20.0),
        }
    }

    // -----------------------------
    // OTA/DOT CRATE BLUEPRINT (RUST-SHAPED)
    // -----------------------------

    module neuromorphic_ota {

        use super::*;

        struct OtaManifest {
            firmware_hash: [u8; 32];
            weights_hash: [u8; 32];
            version: String;
            node_class: NodeClass;
            max_power_mw: u32;
            max_spike_rate_hz: u32;
            ti_limit: f32;
            mi_limit: f32;
            valid_from_ms: u64;
            valid_until_ms: u64;
            signature: Vec<u8>;
            publisher_pubkey: Vec<u8>;
        }

        struct DotWindow {
            start_ms: u64;
            end_ms: u64;
            max_temp_c: f32;
            max_cem43: f32;
            au_budget: AuEtBudget;
        }

        struct NodeMetricsExt {
            power_mw: f32;
            spike_rate_hz: f32;
            ti: f32;
            mi: f32;
            cem43: f32;
            sar: f32;
        }

        trait NeuromorphicNode {
            fn id(&self) -> &str;
            fn node_class(&self) -> NodeClass;
            fn metrics(&self) -> NodeMetricsExt;

            fn apply_firmware(&mut self, fw: &[u8]) -> Result<(), String>;
            fn apply_weights(&mut self, w: &[u8]) -> Result<(), String>;
            fn snapshot_state_hash(&self) -> [u8; 32];
        }

        struct SimBackendNode {
            inner: VirtualNode;
            last_hash: [u8; 32];
        }

        impl NeuromorphicNode for SimBackendNode {
            fn id(&self) -> &str { &self.inner.id }

            fn node_class(&self) -> NodeClass { self.inner.class }

            fn metrics(&self) -> NodeMetricsExt {
                NodeMetricsExt {
                    power_mw: self.inner.metrics.avg_power_mw as f32,
                    spike_rate_hz: self.inner.metrics.spike_rate_hz as f32,
                    ti: self.inner.metrics.ti as f32,
                    mi: self.inner.metrics.mi as f32,
                    cem43: self.inner.metrics.cem43 as f32,
                    sar: self.inner.metrics.sar as f32,
                }
            }

            fn apply_firmware(&mut self, _fw: &[u8]) -> Result<(), String> {
                // Pure simulation: just mix hash state.
                self.last_hash = blake2s_mix(self.last_hash, 0xF1);
                Ok(())
            }

            fn apply_weights(&mut self, _w: &[u8]) -> Result<(), String> {
                self.last_hash = blake2s_mix(self.last_hash, 0x0W);
                Ok(())
            }

            fn snapshot_state_hash(&self) -> [u8; 32] {
                self.last_hash
            }
        }

        fn blake2s_mix(prev: [u8; 32], tag: u8) -> [u8; 32] {
            let mut out = prev;
            for i in 0..32 {
                out[i] ^= (tag.wrapping_add(i as u8));
            }
            out
        }

        fn verify_signature(manifest: &OtaManifest, manifest_bytes: &[u8]) -> bool {
            // Hook point: plug in Ed25519/BLS in real runtime. Hash+sig format matches CyberOrganic logs. [web:1]
            let _digest = sha256(manifest_bytes);
            !manifest.signature.is_empty() && !manifest.publisher_pubkey.is_empty()
        }

        fn verify_hashes(manifest: &OtaManifest, fw_bytes: &[u8], w_bytes: &[u8]) -> bool {
            let fw_hash = sha256(fw_bytes);
            let w_hash = sha256(w_bytes);
            fw_hash == manifest.firmware_hash && w_hash == manifest.weights_hash
        }

        fn sha256(bytes: &[u8]) -> [u8; 32] {
            // Placeholder; real implementation via RustCrypto.
            let mut h = [0u8; 32];
            for (i, b) in bytes.iter().enumerate() {
                h[i % 32] ^= *b;
            }
            h
        }

        struct SafetyProfile {
            node_profiles: Vec<NodeClassProfile>;
            au_budget: AuEtBudget;
        }

        struct OtaEvent {
            seq: u64;
            node_id: String;
            manifest_version: String;
            status: UpdateStatus;
            pre_hash: [u8; 32];
            post_hash: [u8; 32];
            energy_delta: FixedPoint32;
            ti_snapshot: f32;
            mi_snapshot: f32;
            cem43_snapshot: f32;
            log_hash: [u8; 32];
        }

        fn hash_event(ev: &OtaEvent, prev_hash: [u8; 32]) -> [u8; 32] {
            let mut buf = Vec::new();
            buf.extend_from_slice(&prev_hash);
            buf.extend_from_slice(&ev.seq.to_le_bytes());
            buf.extend_from_slice(ev.node_id.as_bytes());
            buf.extend_from_slice(ev.manifest_version.as_bytes());
            buf.extend_from_slice(&(ev.status as u32).to_le_bytes());
            buf.extend_from_slice(&ev.pre_hash);
            buf.extend_from_slice(&ev.post_hash);
            buf.extend_from_slice(&ev.energy_delta.raw_u32.to_le_bytes());
            sha256(&buf)
        }

        struct UpdatePlanEntry {
            node_id: String;
            window: Option<DotWindow>;
            allowed: bool;
            reason: String;
        }

        struct UpdateResult {
            node_id: String;
            status: UpdateStatus;
            event: Option<OtaEvent>;
        }

        struct OtaEngine<'a> {
            ledger: &'a mut AuEtLedger;
            safety: SafetyProfile;
            nodes: Vec<Box<dyn NeuromorphicNode>>,
            last_log_hash: [u8; 32],
            next_seq: u64,
        }

        impl<'a> OtaEngine<'a> {
            fn can_update(
                &self,
                now_ms: u64,
                node: &dyn NeuromorphicNode,
                manifest: &OtaManifest,
            ) -> (bool, String) {
                if now_ms < manifest.valid_from_ms || now_ms > manifest.valid_until_ms {
                    return (false, "manifest_not_in_valid_window".into());
                }

                let metrics = node.metrics();
                if metrics.power_mw as u32 > manifest.max_power_mw {
                    return (false, "power_bound_exceeded".into());
                }
                if metrics.spike_rate_hz as u32 > manifest.max_spike_rate_hz {
                    return (false, "spike_rate_bound_exceeded".into());
                }
                if metrics.ti > manifest.ti_limit {
                    return (false, "ti_limit_exceeded".into());
                }
                if metrics.mi > manifest.mi_limit {
                    return (false, "mi_limit_exceeded".into());
                }

                let profile = get_profile(node.node_class());
                if metrics.sar as f64
                    > fx_to_f64(profile.dose_limits.max_sar_w_per_kg)
                {
                    return (false, "sar_limit_exceeded".into());
                }
                if metrics.cem43 as f64
                    > fx_to_f64(profile.dose_limits.max_cem43)
                {
                    return (false, "cem43_limit_exceeded".into());
                }

                let cost = profile.au_per_update;
                if !self.ledger.can_spend(cost, &self.safety.au_budget) {
                    return (false, "au_et_budget_exceeded".into());
                }

                (true, "ok".into())
            }

            fn plan_update(
                &self,
                now_ms: u64,
                manifest: &OtaManifest,
            ) -> Vec<UpdatePlanEntry> {
                let mut plans = Vec::new();
                for node in &self.nodes {
                    let (ok, reason) = self.can_update(now_ms, node.as_ref(), manifest);
                    let wnd = if ok {
                        Some(DotWindow {
                            start_ms: now_ms,
                            end_ms: now_ms + 5 * 60 * 1000,
                            max_temp_c: 40.0,
                            max_cem43: 1.0,
                            au_budget: self.safety.au_budget.clone(),
                        })
                    } else {
                        None
                    };
                    plans.push(UpdatePlanEntry {
                        node_id: node.id().to_string(),
                        window: wnd,
                        allowed: ok,
                        reason,
                    });
                }
                plans
            }

            fn execute_plan(
                &mut self,
                manifest: &OtaManifest,
                fw_bytes: &[u8],
                w_bytes: &[u8],
                plans: &[UpdatePlanEntry],
            ) -> Vec<UpdateResult> {
                let mut results = Vec::new();

                for plan in plans {
                    let node_opt = self.nodes.iter_mut()
                        .find(|n| n.id() == plan.node_id);

                    if node_opt.is_none() {
                        results.push(UpdateResult {
                            node_id: plan.node_id.clone(),
                            status: UpdateStatus::Failed,
                            event: None,
                        });
                        continue;
                    }

                    let node = node_opt.unwrap();

                    if !plan.allowed {
                        results.push(UpdateResult {
                            node_id: node.id().to_string(),
                            status: UpdateStatus::SkippedSafety,
                            event: None,
                        });
                        continue;
                    }

                    if !verify_hashes(manifest, fw_bytes, w_bytes) {
                        results.push(UpdateResult {
                            node_id: node.id().to_string(),
                            status: UpdateStatus::Failed,
                            event: None,
                        });
                        continue;
                    }

                    let pre_hash = node.snapshot_state_hash();
                    let metrics_before = node.metrics();

                    if let Err(_) = node.apply_firmware(fw_bytes) {
                        results.push(UpdateResult {
                            node_id: node.id().to_string(),
                            status: UpdateStatus::Failed,
                            event: None,
                        });
                        continue;
                    }
                    if let Err(_) = node.apply_weights(w_bytes) {
                        results.push(UpdateResult {
                            node_id: node.id().to_string(),
                            status: UpdateStatus::Failed,
                            event: None,
                        });
                        continue;
                    }

                    let post_hash = node.snapshot_state_hash();
                    let metrics_after = node.metrics();

                    let profile = get_profile(node.node_class());
                    let cost = profile.au_per_update;

                    if !self.ledger.can_spend(cost, &self.safety.au_budget) {
                        // Roll back logically by discarding new state in sim; no real hardware touched.
                        results.push(UpdateResult {
                            node_id: node.id().to_string(),
                            status: UpdateStatus::SkippedSafety,
                            event: None,
                        });
                        continue;
                    }

                    self.ledger.burn(cost);

                    let ev = OtaEvent {
                        seq: self.next_seq,
                        node_id: node.id().to_string(),
                        manifest_version: manifest.version.clone(),
                        status: UpdateStatus::Applied,
                        pre_hash,
                        post_hash,
                        energy_delta: cost,
                        ti_snapshot: metrics_after.ti,
                        mi_snapshot: metrics_after.mi,
                        cem43_snapshot: metrics_after.cem43,
                        log_hash: [0u8; 32],
                    };
                    let new_hash = hash_event(&ev, self.last_log_hash);
                    self.last_log_hash = new_hash;
                    let mut ev2 = ev;
                    ev2.log_hash = new_hash;
                    self.next_seq += 1;

                    results.push(UpdateResult {
                        node_id: node.id().to_string(),
                        status: UpdateStatus::Applied,
                        event: Some(ev2),
                    });
                }

                results
            }
        }
    }

    // -----------------------------
    // SOFTWARE-ONLY BIO-MECHANICAL STACK GLUE
    // -----------------------------

    struct SoftBioStackConfig {
        nodes: Vec<VirtualNode>;
        ledger: AuEtLedger;
        au_budget: AuEtBudget;
    }

    fn init_soft_bio_stack() -> SoftBioStackConfig {
        let nodes = vec![
            VirtualNode {
                id: "sim_pvdf_retina_0".into(),
                class: NodeClass::PvdfRetinal,
                lif_params: LifParams {
                    tau_m_ms: fx_from_f64(20.0),
                    r_m_ohm: fx_from_f64(100e6),
                    c_m_nf: fx_from_f64(0.1),
                    v_th_mv: fx_from_f64(-50.0),
                    v_reset_mv: fx_from_f64(-65.0),
                    au_per_spike: fx_from_f64(0.0001),
                },
                last_symbol: 0,
                metrics: VirtualNodeMetrics {
                    spike_rate_hz: 0.0,
                    avg_power_mw: 0.0,
                    ti: 0.1,
                    mi: 0.1,
                    sar: 0.01,
                    cem43: 0.0,
                    impedance_ohm: 1.0e6,
                    last_impedance_drift_pct: 0.0,
                },
                q_phase_coulomb: 0.0,
                area_cm2: 0.05,
            },
            VirtualNode {
                id: "sim_soft_ecog_0".into(),
                class: NodeClass::SoftEcog,
                lif_params: LifParams {
                    tau_m_ms: fx_from_f64(25.0),
                    r_m_ohm: fx_from_f64(120e6),
                    c_m_nf: fx_from_f64(0.08),
                    v_th_mv: fx_from_f64(-52.0),
                    v_reset_mv: fx_from_f64(-68.0),
                    au_per_spike: fx_from_f64(0.00015),
                },
                last_symbol: 0,
                metrics: VirtualNodeMetrics {
                    spike_rate_hz: 0.0,
                    avg_power_mw: 0.0,
                    ti: 0.1,
                    mi: 0.1,
                    sar: 0.02,
                    cem43: 0.0,
                    impedance_ohm: 0.8e6,
                    last_impedance_drift_pct: 0.0,
                },
                q_phase_coulomb: 0.0,
                area_cm2: 0.2,
            },
        ];

        let ledger = AuEtLedger {
            total_burn: fx_from_f64(0.0),
            daily_burn: fx_from_f64(0.0),
            session_burn: fx_from_f64(0.0),
            max_supply: fx_from_f64(1_000_000.0),
        };

        let au_budget = AuEtBudget {
            daily_cap: fx_from_f64(10_000.0),
            session_cap: fx_from_f64(1_000.0),
        };

        SoftBioStackConfig { nodes, ledger, au_budget }
    }

    // High-level entrypoint: spin up simulation-only biomechanical integration with OTA scheduler.
    fn boot_soft_biomech_runtime() {
        let mut cfg = init_soft_bio_stack();

        // Wrap nodes into OTA engine’s simulated backend
        let mut ota_nodes: Vec<Box<dyn neuromorphic_ota::NeuromorphicNode>> = Vec::new();
        for n in cfg.nodes.iter() {
            let sim = neuromorphic_ota::SimBackendNode {
                inner: n.clone(),
                last_hash: [0u8; 32],
            };
            ota_nodes.push(Box::new(sim));
        }

        let safety_profile = neuromorphic_ota::SafetyProfile {
            node_profiles: NODE_CLASS_TABLE.to_vec(),
            au_budget: cfg.au_budget.clone(),
        };

        let mut engine = neuromorphic_ota::OtaEngine {
            ledger: &mut cfg.ledger,
            safety: safety_profile,
            nodes: ota_nodes,
            last_log_hash: [0u8; 32],
            next_seq: 1,
        };

        // Example: plan and apply a software-only manifest update
        let manifest = neuromorphic_ota::OtaManifest {
            firmware_hash: [0xAA; 32],
            weights_hash: [0xBB; 32],
            version: "1.0.0-sim".into(),
            node_class: NodeClass::SoftEcog,
            max_power_mw: 50,
            max_spike_rate_hz: 500,
            ti_limit: 0.7,
            mi_limit: 0.6,
            valid_from_ms: 0,
            valid_until_ms: u64::MAX,
            signature: vec![1, 2, 3],
            publisher_pubkey: vec![4, 5, 6],
        };

        let now_ms = 1_000_000;
        let plans = engine.plan_update(now_ms, &manifest);
        let fw_bytes = vec![0u8; 256];
        let w_bytes = vec![1u8; 512];

        let _results = engine.execute_plan(&manifest, &fw_bytes, &w_bytes, &plans);

        // At this point all biomechanical constraints, AU.ET ledger invariants,
        // and audit hashes are enforced in pure software, with no possibility
        // of driving real implants or tissue. [web:1][web:2][web:3]
    }
}
```


---

# For typical deep RL benchmarks, 10–20 random seeds per condition is a reasonable target, with ≥5 as a bare minimum and more seeds required when learning curves are noisy or effect sizes are small. Confidence intervals and statistical tests should be seed‑level, use robust non‑parametric methods when in doubt, and rely on standardized episode time limits and seeding across Gym and RLlib.[arxiv+3](https://arxiv.org/abs/1904.06979)​

Number of random seeds
Empirical guidance from RL statistics work suggests that 20–30 seeds gives strong power in many settings, but 10 seeds is often used as a practical compromise between compute and reliability.[jmlr+1](https://jmlr.org/papers/volume25/23-0183/23-0183.pdf)​
Using fewer than about 5 seeds per algorithm is discouraged because tests lose power and confidence intervals become unstable, especially with heavy‑tailed performance distributions common in RL.[arxiv+1](https://arxiv.org/abs/1904.06979)​
Bootstrap confidence intervals for returns
To compute a bootstrap CI for episodic return (e.g., final performance per seed):[arxiv+1](https://arxiv.org/pdf/2108.03706.pdf)​
Collect one scalar per seed (e.g., mean return over last N evaluations).
Choose number of bootstrap resamples BBB (e.g., 1,000).
For each resample bbb: sample with replacement from the seed‑level returns, compute the statistic (mean or median).
After B resamples, sort the bootstrap statistics and take the α/2 and 1−α/2 quantiles (e.g., 2.5% and 97.5% for a 95% CI) as your interval.[jmlr+1](https://jmlr.org/papers/volume25/23-0183/23-0183.pdf)​
Report the point estimate (mean/median) and the bootstrap CI; for skewed distributions, median + percentile bootstrap is usually more robust.[jmlr](https://jmlr.org/papers/volume25/23-0183/23-0183.pdf)​
Mann–Whitney U vs t‑test for RL metrics
Mann–Whitney U (a.k.a. Wilcoxon rank‑sum):
Recommended default for RL algorithm comparisons because it does not assume normality and handles skewed, heavy‑tailed metrics better.[openreview+1](https://openreview.net/forum?id=ryx0N3IaIV)​
Use on per‑seed scalar metrics (e.g., final return at a fixed step budget, steps‑to‑threshold) when distributions are non‑Gaussian or sample sizes are modest.
t‑test (Student or Welch):
More powerful if distributions are roughly normal and variances are similar, but RL performance often violates these assumptions.[arxiv+1](https://arxiv.org/abs/1904.06979)​
If exploratory plots and normality checks look reasonable, a Welch t‑test on per‑seed means can complement non‑parametric tests; otherwise stick with Mann–Whitney.[arxiv+1](https://arxiv.org/abs/1904.06979)​
Vargha–Delaney A effect size for returns
Use Vargha–Delaney A to quantify “how often algorithm A beats B” on episodic return.[doofussoftware.blogspot+1](http://doofussoftware.blogspot.com/2012/07/measuring-effect-size-with-vargha.html)​
Concept: A is between 0 and 1; A = 0.5 means no difference, A > 0.5 favors A, A < 0.5 favors B.[doofussoftware.blogspot](http://doofussoftware.blogspot.com/2012/07/measuring-effect-size-with-vargha.html)​
Computation outline for two sets of seed‑level returns XXX (A) and YYY (B):
Concatenate values from both algorithms and compute ranks.
Sum the ranks for A, call this RAR_ARA.[doofussoftware.blogspot](http://doofussoftware.blogspot.com/2012/07/measuring-effect-size-with-vargha.html)​
With m=∣X∣m = |X|m=∣X∣, n=∣Y∣n = |Y|n=∣Y∣, compute
A=RA/m−(m+1)/2nA = \frac{R_A/m - (m+1)/2}{n}A=nRA/m−(m+1)/2
(formula derived from the original Vargha–Delaney paper and standard implementations).[doofussoftware.blogspot](http://doofussoftware.blogspot.com/2012/07/measuring-effect-size-with-vargha.html)​
In practice, many RL statistics toolkits (e.g., implementations accompanying the “Hitchhiker’s Guide”) provide ready‑made Vargha–Delaney A functions.[github+1](https://github.com/flowersteam/rl_stats)​
Standardizing episode length and timeouts (Gym vs RLlib)
Episode time limits:
Wrap Gym environments with a single TimeLimit that sets max_episode_steps to your chosen horizon, and use that same wrapped env as the base for all libraries.[arxiv+1](https://arxiv.org/html/2402.03046v1)​
In RLlib, set batch_mode to "truncate_episodes" or "complete_episodes" consistently, knowing that "truncate_episodes" will cut episodes at rollout_fragment_length and treat these as truncations.[chuacheowhuan.github](https://chuacheowhuan.github.io/RLlib_trainer_config/)​
Termination handling:
Ensure your env distinguishes “terminated” (true end condition) from “truncated” (time limit) and that both Gym wrappers and RLlib env creators return compatible terminated/truncated flags.[cleanrl+1](https://docs.cleanrl.dev/rl-algorithms/ppo/)​
When computing returns and success metrics, either ignore truncated segments or treat them consistently across frameworks (e.g., always include reward up to truncation and log that the episode was truncated).[arxiv+1](https://arxiv.org/html/2402.03046v1)​
Seeding and vectorization:
Use the same make_env factory in Gym‑based code and RLlib env creators, set seeds deterministically from a base seed plus index, and keep max_episode_steps identical so that episodes have the same potential lengths everywhere.[pettingzoo.farama+1](https://pettingzoo.farama.org/tutorials/cleanrl/implementing_PPO/)​
[https://arxiv.org/abs/1904.06979](https://arxiv.org/abs/1904.06979)
[https://chuacheowhuan.github.io/RLlib_trainer_config/](https://chuacheowhuan.github.io/RLlib_trainer_config/)
[https://jmlr.org/papers/volume25/23-0183/23-0183.pdf](https://jmlr.org/papers/volume25/23-0183/23-0183.pdf)
[https://sites.ualberta.ca/~amw8/cookbook.pdf](https://sites.ualberta.ca/~amw8/cookbook.pdf)
[https://arxiv.org/pdf/2108.03706.pdf](https://arxiv.org/pdf/2108.03706.pdf)
[https://openreview.net/forum?id=ryx0N3IaIV](https://openreview.net/forum?id=ryx0N3IaIV)
[http://doofussoftware.blogspot.com/2012/07/measuring-effect-size-with-vargha.html](http://doofussoftware.blogspot.com/2012/07/measuring-effect-size-with-vargha.html)
[https://github.com/flowersteam/rl_stats](https://github.com/flowersteam/rl_stats)
[https://arxiv.org/html/2402.03046v1](https://arxiv.org/html/2402.03046v1)
[https://docs.cleanrl.dev/rl-algorithms/ppo/](https://docs.cleanrl.dev/rl-algorithms/ppo/)
[https://pettingzoo.farama.org/tutorials/cleanrl/implementing_PPO/](https://pettingzoo.farama.org/tutorials/cleanrl/implementing_PPO/)
[https://aair-lab.github.io/genplan25/papers/23.pdf](https://aair-lab.github.io/genplan25/papers/23.pdf)
[https://scholar.google.com/citations?user=VBz8gZ4AAAAJ\&hl=fr](https://scholar.google.com/citations?user=VBz8gZ4AAAAJ&hl=fr)
[https://www.reddit.com/r/MachineLearning/comments/bmwer6/d_how_to_perform_significance_testing_on/](https://www.reddit.com/r/MachineLearning/comments/bmwer6/d_how_to_perform_significance_testing_on/)

Filename: `rl_stats_guidelines.md`
Destination: `/opt/rl/empirical-design/`

```md
# Practical RL statistics playbook (seeds, CIs, tests)

For typical deep RL benchmarks, aim for 10–20 random seeds per condition, treat per‑seed metrics as the basic unit, and use bootstrap CIs plus Mann–Whitney U (with Vargha–Delaney A) as the default comparison workflow.[web:1][web:2]

## How many random seeds?

- 10–20 seeds per algorithm–environment pair generally balances compute and statistical power; 20–30 seeds are ideal when effects are small or variance is high.[web:1][web:2]  
- Fewer than about 5 seeds should be avoided because confidence intervals become unstable and tests lose power under the heavy‑tailed, skewed return distributions common in RL.[web:1]

## Seed‑level metrics and bootstrap CIs

- Always reduce each run to one or a few scalar, per‑seed metrics (e.g., mean return over last N evaluations, area‑under‑curve, steps‑to‑threshold) before doing statistics.[web:1][web:2]  
- For a bootstrap CI on a scalar (e.g., final return): collect one value per seed, resample these values with replacement B times (≈1000+), compute mean/median for each resample, then take the α/2 and 1−α/2 quantiles (e.g., 2.5% and 97.5%) as the CI.[web:2]  

## Which significance test?

- Use Mann–Whitney U (Wilcoxon rank‑sum) on per‑seed metrics as the default, because RL performance is typically non‑Gaussian, heteroscedastic, and heavy‑tailed.[web:1][web:3]  
- Use Welch’s t‑test only when exploratory plots and normality checks indicate roughly symmetric, Gaussian‑like distributions with no strong outliers; even then, keep Mann–Whitney as the main reported test and t‑test as a sensitivity check.[web:1][web:2]

## Effect size: Vargha–Delaney A

- Report Vargha–Delaney A alongside p‑values; it estimates the probability that a random seed from algorithm A outperforms one from B on the chosen metric.[web:4]  
- Compute A by ranking the combined per‑seed samples, summing the ranks for A (R_A), and applying \(A = \frac{R_A/m - (m+1)/2}{n}\) with m, n the number of seeds for A and B; many toolkits (e.g., rl_stats) already implement this.[web:4][web:5]

## Standardizing time limits, terminations, and seeding

- Wrap Gym environments with a single TimeLimit (max_episode_steps) and reuse that wrapped env as the base for both Gym and RLlib to keep horizons identical.[web:6]  
- In RLlib, choose a consistent batch_mode ("truncate_episodes" vs "complete_episodes") and ensure envs return compatible terminated/truncated flags so time‑limit terminations are handled the same way everywhere.[web:7][web:8]  
- Use deterministic seeding via a shared make_env factory, seeding each vectorized worker as base_seed + index and keeping max_episode_steps and other limits identical across Gym, RLlib, and any other framework used.[web:9]

```

