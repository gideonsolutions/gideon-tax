use crate::Usd;

/// Output fields for IRS Form 8912 (2025) — Credit to Holders of Tax Credit Bonds.
#[derive(Debug, Clone, Default)]
pub struct Output8912 {
    // -----------------------------------------------------------------------
    // Part I — Current Year Credit
    // -----------------------------------------------------------------------
    /// Line 1: Bond credit(s) from Part III (amount from line 14)
    pub total_all_form1097_btc_amt: Usd,
    /// Line 2: Bond credit(s) from Part IV (amount from line 20)
    pub total_other_not_rpt_f1097_btc_amt: Usd,
    /// Line 3: Carryforward of credits for qualified tax credit bonds and build America bonds to 2021
    pub carryforward_py_bond_credit_amt: Usd,
    /// Line 4: Total credit (add lines 1 through 3)
    pub total_credit_amt: Usd,
    /// Line 5: Amount allocated to beneficiaries of the estate or trust
    pub estate_or_trust_allocated_benef_amt: Usd,
    /// Line 6: Estates and trusts (subtract line 5 from line 4)
    pub est_tr_cy_bond_cr_amt: Usd,

    // -----------------------------------------------------------------------
    // Part II — Allowable Credit
    // -----------------------------------------------------------------------
    /// Line 7: Regular tax before credits
    pub regular_tax_before_credit_amt: Usd,
    /// Line 8: Alternative minimum tax
    pub alternative_minimum_tax_amt: Usd,
    /// Line 9: Add line 7 and line 8
    pub sum_regular_tax_and_alt_min_tx_amt: Usd,
    /// Line 10a: Foreign tax credit
    pub foreign_tax_credit_amt: Usd,
    /// Line 10b: Certain allowable credits
    pub certain_allowable_credits_amt: Usd,
    /// Line 10c: General business credit
    pub general_business_credit_amt: Usd,
    /// Line 10d: Credit for prior year minimum tax (Form 8801 or Form 8827)
    pub credit_prior_year_minimum_tax_amt: Usd,
    /// Line 10e: Add lines 10a through 10d
    pub total_credits_amt: Usd,
    /// Line 11: Net income tax (subtract line 10e from line 9)
    pub net_income_tax_amt: Usd,
    /// Line 12: Credit to holders of tax credit bonds allowed for the current year
    pub current_year_allowable_credit_amt: Usd,

    // -----------------------------------------------------------------------
    // Part III — Bond Credit(s) Reported to You on Form(s) 1097-BTC
    // -----------------------------------------------------------------------
    /// Line 14: Total of amounts reported on Form(s) 1097-BTC (enter here and on line 1)
    pub new_clean_energy_bond_amt: Usd,
}
