use crate::Usd;

/// Output fields for IRS Form 8834 (2025) — Qualified Electric Vehicle Credit.
#[derive(Debug, Clone, Default)]
pub struct Output8834 {
    /// Line 1: Qualified electric vehicle passive activity credits allowed for your current tax year
    pub qlfy_elec_motor_veh_cr_amt: Usd,
    /// Line 2: Regular tax before credits
    pub qlfy_elec_veh_regular_tx_bfr_cr_amt: Usd,
    /// Line 3a: Foreign tax credit
    pub foreign_tax_credit_amt: Usd,
    /// Line 3b: Certain allowable credits (see instructions)
    pub certain_allowable_credits_amt: Usd,
    /// Line 3c: Add lines 3a and 3b
    pub tot_tax_cr_bfr_qlfy_elec_veh_cr_amt: Usd,
    /// Line 4: Net regular tax. Subtract line 3c from line 2. If zero or less, enter -0-
    pub qlfy_elec_veh_net_regular_tax_amt: Usd,
    /// Line 5: Tentative minimum tax
    pub qlfy_elec_veh_tentative_min_tax_amt: Usd,
    /// Line 6: Subtract line 5 from line 4. If zero or less, enter -0-
    pub qlfy_elec_veh_adj_regular_tax_amt: Usd,
    /// Line 7: Qualified electric vehicle credit. Enter the smaller of line 1 or line 6
    pub qlfy_elec_veh_pssv_acty_cr_allw_amt: Usd,
}
