use crate::Usd;

/// Output fields for IRS Form 8866 (2025) — Interest Computation Under the Look-Back Method for Property Depreciated Under the Income Forecast Method.
#[derive(Debug, Clone, Default)]
pub struct Output8866 {
    // -----------------------------------------------------------------------
    // Header — Taxpayer Information
    // -----------------------------------------------------------------------
    /// Tax year beginning date
    pub tax_year_begin_dt: String,
    /// Tax year ending date
    pub tax_year_end_dt: String,
    /// Box B: Corporation indicator
    pub corporation_ind: bool,
    /// Box B: S corporation indicator
    pub s_corporation_ind: bool,
    /// Box B: Individual indicator
    pub individual_ind: bool,
    /// Box B: Partnership indicator
    pub partnership_ind: bool,
    /// Box B: Estate or trust indicator
    pub estate_or_trust_ind: bool,

    // -----------------------------------------------------------------------
    // Lines 1-8 — Interest Computation (per recomputation/prior year columns)
    // -----------------------------------------------------------------------
    /// Line 1: Taxable income (or loss) for the prior years shown on tax return (year ended date)
    pub year_ended_dt: String,
    /// Line 1: Taxable income or loss amount
    pub taxable_income_or_loss_amt: Usd,
    /// Line 2: Adjustment to taxable income for the difference between estimated and actual depreciation
    pub income_adjustment_amt: Usd,
    /// Line 3: Adjusted taxable income for look-back purposes. Combine lines 1 and 2
    pub adj_taxable_income_look_back_amt: Usd,
    /// Line 4: Income tax liability on line 3 amount using tax rates in effect for the prior years
    pub tax_liability_amt: Usd,
    /// Line 5: Income tax liability shown on return (or as previously adjusted) for the prior years
    pub federal_income_tax_liability_amt: Usd,
    /// Line 6: Increase (or decrease) in tax for the prior years. Subtract line 5 from line 4
    pub increase_or_decrease_in_tx_for_py_amt: Usd,
    /// Line 7: Interest due on increase, if any, shown on line 6
    pub interest_due_on_increase_amt: Usd,
    /// Line 8: Interest to be refunded on decrease, if any, shown on line 6
    pub interest_to_be_refunded_on_decr_amt: Usd,

    // -----------------------------------------------------------------------
    // Lines 9-10 — Totals and Net Interest
    // -----------------------------------------------------------------------
    /// Line 9a: Net interest to be refunded to you (column (c) totals)
    pub total_interest_to_be_refunded_amt: Usd,
    /// Line 9b: Routing number
    pub routing_transit_num: String,
    /// Line 9c: Bank account type (Checking/Savings)
    pub bank_account_type_cd: String,
    /// Line 9d: Account number
    pub depositor_account_num: String,
    /// Line 10: Net interest you owe (column (c) totals)
    pub net_amt_of_interest_owed_amt: Usd,

    // -----------------------------------------------------------------------
    // Computed totals
    // -----------------------------------------------------------------------
    /// Total adjustment to income amount (sum of line 2 across all columns)
    pub total_adjustment_to_income_amt: Usd,
    /// Total interest due on increase amount (sum of line 7 across all columns)
    pub total_interest_due_on_increase_amt: Usd,
}
