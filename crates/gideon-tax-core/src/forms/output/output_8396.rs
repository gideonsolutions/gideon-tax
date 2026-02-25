use crate::Usd;

/// Output fields for IRS Form 8396 (2025) — Mortgage Interest Credit.
#[derive(Debug, Clone, Default)]
pub struct Output8396 {
    // -----------------------------------------------------------------------
    // Header Information
    // -----------------------------------------------------------------------
    /// Header: Name of issuer of Mortgage Credit Certificate
    pub mortg_sbsdy_cert_issuer_agency_nm: String,
    /// Header: Mortgage Credit Certificate number
    pub mortgage_credit_certificate_num: String,
    /// Header: Issue date
    pub mortg_cr_certificate_issue_dt: String,
    /// Header: Address of main home to which the qualified mortgage certificate relates
    pub qlfy_mortgage_cert_us_address: String,

    // -----------------------------------------------------------------------
    // Part I — Current-Year Mortgage Interest Credit
    // -----------------------------------------------------------------------
    /// Line 1: Interest paid on the certified indebtedness amount
    pub certified_mortgage_int_cr_pd_amt: Usd,
    /// Line 2: Certificate credit rate shown on Mortgage Credit Certificate
    pub mortgage_credit_certificate_rt: String,
    /// Line 3: Mortgage interest credit amount (line 1 times line 2, or limited amount)
    pub mortgage_interest_credit_amt: Usd,
    /// Line 4: 2022 credit carryforward from line 16 of 2024 Form 8396
    pub mortg_int_previous3_yr_cfwd_cr_amt: Usd,
    /// Line 5: 2023 credit carryforward from line 14 of 2024 Form 8396
    pub mortg_int_previous2_yr_cfwd_cr_amt: Usd,
    /// Line 6: 2024 credit carryforward from line 17 of 2024 Form 8396
    pub mortg_int_py_carryforward_cr_amt: Usd,
    /// Line 7: Sum of lines 3 through 6
    pub larger_of_mortg_int_cr_or_cfwd_amt: Usd,
    /// Line 8: Limitation based on tax liability (from Credit Limit Worksheet line 3)
    pub tax_liab_lmt_from_cr_lmt_wrksht_amt: Usd,
    /// Line 9: Current-year mortgage interest credit (smaller of line 7 or line 8)
    pub mortgage_interest_reduction_amt: Usd,

    // -----------------------------------------------------------------------
    // Part II — Mortgage Interest Credit Carryforward to 2026
    // -----------------------------------------------------------------------
    /// Line 10: Add lines 3 and 4
    pub mortg_int_red_plus_oldest_cfwd_cr_amt: Usd,
    /// Line 11: Enter the amount from line 7
    pub mortg_int_tot_previous_cfwd_cr_amt: Usd,
    /// Line 12: Enter the larger of line 9 or line 10
    pub mortg_int_tent3_year_cfwd_cr_amt: Usd,
    /// Line 13: Subtract line 12 from line 11
    pub mortg_int_tent_two_year_cfwd_cr_amt: Usd,
    /// Line 14: 2024 credit carryforward to 2026 (smaller of line 6 or line 13)
    pub mortg_int_next_years_py_cfwd_cr_amt: Usd,
    /// Line 15: Subtract line 14 from line 13
    pub mortg_int_next_years2_yr_cfwd_cr_amt: Usd,
    /// Line 16: 2023 credit carryforward to 2026 (smaller of line 5 or line 15)
    pub mortg_int_next_years3_yr_cfwd_cr_amt: Usd,
}
