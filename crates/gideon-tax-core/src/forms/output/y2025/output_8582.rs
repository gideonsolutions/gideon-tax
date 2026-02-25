use crate::Usd;

/// Output fields for IRS Form 8582 (2025) — Passive Activity Loss Limitations.
#[derive(Debug, Clone, Default)]
pub struct Output8582 {
    // -----------------------------------------------------------------------
    // Part I — 2025 Passive Activity Loss
    // -----------------------------------------------------------------------
    // Rental Real Estate Activities With Active Participation
    /// Line 1a: Activities with net income (from Part IV, column (a))
    pub rental_realty_income_amt: Usd,
    /// Line 1b: Activities with net loss (from Part IV, column (b))
    pub rental_realty_loss_amt: Usd,
    /// Line 1c: Prior years' unallowed losses (from Part IV, column (c))
    pub py_unallowed_rental_loss_amt: Usd,
    /// Line 1d: Combine lines 1a, 1b, and 1c
    pub net_rental_realty_amt: Usd,
    // All Other Passive Activities
    /// Line 2a: Activities with net income (from Part V, column (a))
    pub other_activity_income_amt: Usd,
    /// Line 2b: Activities with net loss (from Part V, column (b))
    pub other_activity_loss_amt: Usd,
    /// Line 2c: Prior years' unallowed losses (from Part V, column (c))
    pub prior_year_unallowed_other_loss_amt: Usd,
    /// Line 2d: Combine lines 2a, 2b, and 2c
    pub net_other_activity_amt: Usd,
    /// Line 3: Combine lines 1d and 2d and subtract any prior year unallowed CRD
    pub total_passive_activity_amt: Usd,

    // -----------------------------------------------------------------------
    // Part II — Special Allowance for Rental Real Estate Activities With Active Participation
    // -----------------------------------------------------------------------
    /// Line 4: Smaller of the loss on line 1d or the loss on line 3
    pub rental_realty_loss_limit_amt: Usd,
    /// Line 5: $150,000 (or amount if married filing separately)
    pub maximum_allowed_income_amt: Usd,
    /// Line 6: Modified adjusted gross income (not less than zero)
    pub modified_agi_amt: Usd,
    /// Line 7: Subtract line 6 from line 5
    pub modified_agi_difference_amt: Usd,
    /// Line 8: Multiply line 7 by 50% (0.50), not more than $25,000
    pub percent_net_special_allowance_amt: Usd,
    /// Line 9: Smaller of line 4 or line 8 (or includes CRD if applicable)
    pub total_special_allowance_amt: Usd,

    // -----------------------------------------------------------------------
    // Part III — Total Losses Allowed
    // -----------------------------------------------------------------------
    /// Line 10: Add the income, if any, on lines 1a and 2a and enter the total
    pub total_income_amt: Usd,
    /// Line 11: Total losses allowed from all passive activities for 2025 (add lines 9 and 10)
    pub total_losses_allowed_amt: Usd,

    // -----------------------------------------------------------------------
    // Part IV — Rental Real Estate Activities (detail worksheet for Part I, Lines 1a, 1b, and 1c)
    // -----------------------------------------------------------------------
    /// Part IV total, column (a): Current year net income (line 1a)
    pub total_current_year_net_income_amt: Usd,
    /// Part IV total, column (b): Current year net loss (line 1b)
    pub total_current_year_net_loss_amt: Usd,
    /// Part IV total, column (c): Prior year unallowed loss (line 1c)
    pub total_prior_yr_rental_unallowed_amt: Usd,

    // -----------------------------------------------------------------------
    // Part V — All Other Passive Activities (detail worksheet for Part I, Lines 2a, 2b, and 2c)
    // -----------------------------------------------------------------------
    /// Part V total, column (a): Current year net income (line 2a)
    pub total_other_current_year_income_amt: Usd,
    /// Part V total, column (b): Current year net loss (line 2b)
    pub total_other_current_year_loss_amt: Usd,
    /// Part V total, column (c): Prior year unallowed loss (line 2c)
    pub total_other_py_unallowed_amt: Usd,

    // -----------------------------------------------------------------------
    // Part VI — Special Allowance Allocation (if amount on Part II, Line 9)
    // -----------------------------------------------------------------------
    /// Part VI total, column (a): Loss
    pub total_loss_amt: Usd,
    /// Part VI: Total net special allowance amount
    pub total_net_special_allowance_amt: Usd,

    // -----------------------------------------------------------------------
    // Part VII — Allocation of Unallowed Losses
    // -----------------------------------------------------------------------
    /// Part VII total, column (a): Loss allocation
    pub total_allocation_loss_amt: Usd,
    /// Part VII: Total unallowed loss
    pub total_unallowed_loss_amt: Usd,

    // -----------------------------------------------------------------------
    // Part VIII — Allowed Losses
    // -----------------------------------------------------------------------
    /// Part VIII: Total allowed loss
    pub total_allowed_loss_amt: Usd,
    /// Part VIII: Allowed rental realty loss amount
    pub allowed_rental_realty_loss_amt: Usd,
}
