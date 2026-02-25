use crate::Usd;

/// Output fields for IRS Form 6198 (2025) — At-Risk Limitations.
#[derive(Debug, Clone, Default)]
pub struct Output6198 {
    // -----------------------------------------------------------------------
    // Top-of-form
    // -----------------------------------------------------------------------
    /// Description of activity (see instructions)
    pub activity_description_txt: String,

    // -----------------------------------------------------------------------
    // Part I — Current Year Profit (Loss) From the Activity, Including Prior Year Nondeductible Amounts
    // -----------------------------------------------------------------------
    /// Line 1: Ordinary income (loss) from the activity (see instructions)
    pub ordinary_income_loss_amt: Usd,
    /// Line 2a: Schedule D gain or loss from the sale or other disposition of assets used in the activity
    pub schedule_d_gain_or_loss_amt: Usd,
    /// Line 2b: Form 4797 gain or loss from the sale or other disposition of assets used in the activity
    pub form4797_gain_or_loss_amt: Usd,
    /// Line 2c: Other form or schedule gain or loss
    pub other_form_or_schedule_amt: Usd,
    /// Line 2c: Schedule or form number
    pub schedule_or_form_num: String,
    /// Line 3: Other income and gains from the activity, from Schedule K-1 (Form 1065) or
    /// Schedule K-1 (Form 1120-S), that were not included on lines 1 through 2c
    pub schedule_k1_income_or_gain_amt: Usd,
    /// Line 4: Other deductions and losses from the activity, including investment interest expense
    /// allowed from Form 4952, that were not included on lines 1 through 2c
    pub other_deductions_amt: Usd,
    /// Line 5: Current year profit (loss) from the activity. Combine lines 1 through 4
    pub current_year_profit_or_loss_amt: Usd,

    // -----------------------------------------------------------------------
    // Part II — Simplified Computation of Amount at Risk
    // -----------------------------------------------------------------------
    /// Line 6: Adjusted basis (as defined in section 1011) in the activity on the first day of the tax year.
    /// Do not enter less than zero
    pub adjusted_basis_amt: Usd,
    /// Line 7: Increases for the tax year (see instructions)
    pub increases_amt: Usd,
    /// Line 8: Add lines 6 and 7
    pub sum_adj_basis_and_increase_risk_amt: Usd,
    /// Line 9: Decreases for the tax year (see instructions)
    pub decreases_amt: Usd,
    /// Line 10a: Subtract line 9 from line 8
    pub sum_adj_basis_incr_less_decr_risk_amt: Usd,
    /// Line 10b: If line 10a is more than zero, enter that amount here and go to line 20
    /// (or complete Part III). Otherwise, enter -0-
    pub simplified_computation_risk_amt: Usd,

    // -----------------------------------------------------------------------
    // Part III — Detailed Computation of Amount at Risk
    // -----------------------------------------------------------------------
    /// Line 11: Investment in the activity (or in your interest in the activity) at the effective date.
    /// Do not enter less than zero
    pub activity_investment_amt: Usd,
    /// Line 12: Increases at effective date
    pub effective_date_increase_amt: Usd,
    /// Line 13: Add lines 11 and 12
    pub sum_acty_investment_and_dt_incr_amt: Usd,
    /// Line 14: Decreases at effective date
    pub effective_date_decrease_amt: Usd,
    /// Line 15: Amount at risk (check box that applies):
    /// (a) At effective date. Subtract line 14 from line 13. Do not enter less than zero.
    /// (b) From your prior year Form 6198, line 19b. Do not enter the amount from line 10b
    /// of your prior year form
    pub amount_at_risk_amt: Usd,
    /// Line 15a: At effective date indicator
    pub effective_date_risk_amount_ind: bool,
    /// Line 15b: From prior year Form 6198 indicator
    pub form6198_prior_year_ind: bool,
    /// Line 16: Increases since (check box that applies):
    /// (a) Effective date, or (b) The end of your prior year
    pub current_year_increase_risk_amt: Usd,
    /// Line 16a: Increase since effective date indicator
    pub increase_since_effective_date_ind: bool,
    /// Line 16b: Increase since prior year indicator
    pub increase_since_prior_year_ind: bool,
    /// Line 17: Add lines 15 and 16
    pub sum_amount_at_risk_and_increase_amt: Usd,
    /// Line 18: Decreases since (check box that applies):
    /// (a) Effective date, or (b) The end of your prior year
    pub current_year_decrease_risk_amt: Usd,
    /// Line 18a: Decrease since effective date indicator
    pub decrease_since_effective_date_ind: bool,
    /// Line 18b: Decrease since prior year indicator
    pub decrease_since_prior_year_ind: bool,
    /// Line 19a: Subtract line 18 from line 17
    pub detailed_computation_risk1_amt: Usd,
    /// Line 19b: If line 19a is more than zero, enter that amount here and go to line 20.
    /// Otherwise, enter -0-
    pub detailed_computation_risk_amt: Usd,

    // -----------------------------------------------------------------------
    // Part IV — Deductible Loss
    // -----------------------------------------------------------------------
    /// Line 20: Amount at risk. Enter the larger of line 10b or line 19b
    pub net_at_risk_amt: Usd,
    /// Line 21: Deductible loss. Enter the smaller of the line 5 loss (treated as a positive number)
    /// or line 20
    pub deductible_loss_amt: Usd,
}
