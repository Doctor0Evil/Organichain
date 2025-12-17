# BioAug EMT Bench: MT6883 Phantom Experiment

## Goal and topology

This bench experiment couples an MT6883-class ARMv8 SoC dev board, an isolated DAQ, and an ISO-aligned tissue phantom to enforce Bio-Aug EMT envelopes in real time while logging all actuation and thermal responses through a GitHub-backed workflow and PostgreSQL schema.[web:1][web:2]

### Core hardware path

- MT6883 dev board on a lab carrier with UART + I2C/SPI exposed to DAQ and host.[web:2]  
- Medical-grade stimulator or programmable current source drives an ISO 10993/14708 phantom with embedded thermocouples and a miniature load cell.[web:1]  
- Isolated DAQ (NI/Keysight-class) samples temperature, force, voltage, and current at 100 Hz or faster and streams data to the host for EMT envelope reconstruction.[web:1]

## Host + firmware workflow

The host workstation (Linux or Windows) runs the Rust toolchain, ALN interpreter, MATLAB, and PostgreSQL, binding EMT envelopes to physical measurements using the EMTBenchSample records and EMTBenchSafety semantic block.[web:1][web:3]  

- `src/mt6883_agent/` builds a no_std Rust agent that reads DAQ samples, updates EMTEnvelope energy density, and invokes the EMT governor loop to compute derate factors or hard halts for the stimulator.[web:2][web:3]  
- `lua/emt_stimulus_bt.lua` runs on the host to select NORMAL/DERATE/STOP stimulus modes from current risk indices, with explicit numeric thresholds aligned to EMT policy profiles.[web:3]  
- `matlab/EMTBenchAnalysis.m` replays logged samples and envelopes, recomputes thermal and mechanical risk, and returns a merged table suitable for publication or regulatory attachments.[web:1]

## Data logging and reproducibility

All EMT envelopes and phantom samples are persisted in PostgreSQL using `sql/emt_experiment_schema.sql`, then exported as CSV and versioned in the GitHub repository alongside ALN datasets and analysis scripts, enabling full audit of firmware changes and EMT law conformance.[web:1][web:3]  

- The ALN dataset `ALN/Experiment.EMTBench.v1.aln` (extended here as `BioAug.EMTBench.Design.v1`) defines EMTBenchSample instances and a formal policy ensuring phantom temperature rise and energy density stay within EMT limits for every envelope.[web:1]  
- Each EMTBenchRun record binds a physical run to a specific Git commit, host, phantom, and analysis path, making the bench protocol repeatable across labs with compatible MT6883-class or similar ARM SoC hardware.[web:1][web:2]
