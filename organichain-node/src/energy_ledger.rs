// organichain-node/src/energy_ledger.rs
// Non-minting AU.ET / CSP ledger with compression 10^-12, mathematically aligned with ALN specs.
use std::collections::HashMap;

#[derive(Clone, Copy, Debug)]
pub struct EnergyCompression {
    pub c_e: f64, // AU.ET compression
    pub c_s: f64, // CSP compression
    pub d_src: u8,
    pub d_aln: u8,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct EnergyVector {
    pub au_et: u128,
    pub csp: u128,
}

#[derive(Debug, Default)]
pub struct EnergyLedger {
    balances: HashMap<[u8; 20], EnergyVector>,
    total_au_et: u128,
    total_csp: u128,
    max_total_au_et: u128,
    max_total_csp: u128,
}

impl EnergyLedger {
    pub fn new(max_total_au_et: u128, max_total_csp: u128) -> Self {
        Self {
            balances: HashMap::new(),
            total_au_et: 0,
            total_csp: 0,
            max_total_au_et,
            max_total_csp,
        }
    }

    pub fn map_source_amount(
        &self,
        amt_src_minimal: u128,
        cfg: EnergyCompression,
    ) -> EnergyVector {
        // Asrc = Bi * 10^-d_src
        let asrc = (amt_src_minimal as f64) * 10f64.powi(-(cfg.d_src as i32));
        let a_e = asrc * cfg.c_e;
        let a_s = asrc * cfg.c_s;
        let factor = 10f64.powi(cfg.d_aln as i32);
        let be = (a_e * factor).floor() as u128;
        let bs = (a_s * factor).floor() as u128;
        EnergyVector { au_et: be, csp: bs }
    }

    pub fn credit(
        &mut self,
        addr: [u8; 20],
        delta: EnergyVector,
    ) -> Result<(), &'static str> {
        // Non-minting at origin-layer: credit is only allowed from sealed refactor bridge;
        // enforce global caps.
        let new_total_au = self
            .total_au_et
            .checked_add(delta.au_et)
            .ok_or("overflow total AU.ET")?;
        let new_total_cs = self
            .total_csp
            .checked_add(delta.csp)
            .ok_or("overflow total CSP")?;
        if new_total_au > self.max_total_au_et || new_total_cs > self.max_total_csp {
            return Err("global energy cap exceeded");
        }

        let entry = self.balances.entry(addr).or_default();
        entry.au_et = entry
            .au_et
            .checked_add(delta.au_et)
            .ok_or("overflow user AU.ET")?;
        entry.csp = entry
            .csp
            .checked_add(delta.csp)
            .ok_or("overflow user CSP")?;
        self.total_au_et = new_total_au;
        self.total_csp = new_total_cs;
        Ok(())
    }

    pub fn debit(
        &mut self,
        addr: [u8; 20],
        delta: EnergyVector,
    ) -> Result<(), &'static str> {
        let entry = self.balances.get_mut(&addr).ok_or("no balance")?;
        if entry.au_et < delta.au_et || entry.csp < delta.csp {
            return Err("insufficient energy");
        }
        entry.au_et -= delta.au_et;
        entry.csp -= delta.csp;
        self.total_au_et -= delta.au_et;
        self.total_csp -= delta.csp;
        Ok(())
    }

    pub fn balance_of(&self, addr: [u8; 20]) -> EnergyVector {
        *self.balances.get(&addr).unwrap_or(&EnergyVector::default())
    }
}

// Simple invariant checker
pub fn check_invariants(ledger: &EnergyLedger) -> bool {
    let mut sum_e = 0u128;
    let mut sum_s = 0u128;
    for v in ledger.balances.values() {
        sum_e = sum_e.saturating_add(v.au_et);
        sum_s = sum_s.saturating_add(v.csp);
    }
    sum_e == ledger.total_au_et
        && sum_s == ledger.total_csp
        && ledger.total_au_et <= ledger.max_total_au_et
        && ledger.total_csp <= ledger.max_total_csp
}
