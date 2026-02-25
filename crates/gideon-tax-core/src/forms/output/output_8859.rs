use crate::Usd;

/// Output fields for IRS Form 8859 (2025) — Carryforward of the District of Columbia First-Time Homebuyer Credit.
#[derive(Debug, Clone, Default)]
pub struct Output8859 {
    /// Line 1: Credit carryforward from 2024. Enter the amount from line 4 of your 2024 Form 8859
    pub dc_hm_byr_credit_carryforward_py_amt: Usd,
    /// Line 2: Limitation based on tax liability. Enter the amount from the Tax Liability Limit Worksheet
    pub tax_liab_lmt_from_cr_lmt_wrksht_amt: Usd,
    /// Line 3: Current year credit. Enter the smaller of line 1 or line 2
    pub dc_hm_byr_current_year_credit_amt: Usd,
    /// Line 4: Credit carryforward to 2026. Subtract line 3 from line 1
    pub dc_hm_byr_credit_cfwd_next_year_amt: Usd,
}
