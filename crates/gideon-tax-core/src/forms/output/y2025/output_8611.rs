use crate::Usd;

/// Output fields for IRS Form 8611 (2025) — Recapture of Low-Income Housing Credit.
#[derive(Debug, Clone, Default)]
pub struct Output8611 {
    // -----------------------------------------------------------------------
    // Header Information (Items A–F)
    // -----------------------------------------------------------------------
    /// Item C: Address of building (as shown on Form 8609)
    pub building_us_address: String,
    /// Item C: Building foreign address (if applicable)
    pub building_foreign_address: String,
    /// Item D: Building identification number (BIN)
    pub bin: String,
    /// Item E: Date placed in service (from Form 8609)
    pub placed_in_service_dt: String,
    /// Item F(1): Issuer's name (tax-exempt bond financing)
    pub business_name_line1_txt: String,
    /// Item F(1): Issuer's name line 2
    pub business_name_line2_txt: String,
    /// Item F(2): Date of issue (tax-exempt bond financing)
    pub issue_dt: String,
    /// Item F(3): Name of issue (tax-exempt bond financing)
    pub issue_nm: String,
    /// Item F(4): CUSIP number (tax-exempt bond financing)
    pub cusip_num: String,
    /// Item F(4): Missing CUSIP reason code
    pub missing_cusip_reason_cd: String,

    // -----------------------------------------------------------------------
    // Lines 1–7 — Credit Recapture Computation
    // -----------------------------------------------------------------------
    /// Line 1: Total credits reported on Form 8586 in prior years for this building
    pub py_total_credits_on_form8586_amt: Usd,
    /// Line 2: Credits included on line 1 attributable to additions to qualified basis
    pub credits_included_amt: Usd,
    /// Line 3: Credits subject to recapture (subtract line 2 from line 1)
    pub credits_subject_to_recapture_amt: Usd,
    /// Line 4: Credit recapture percentage
    pub credit_recapture_percent_rt: String,
    /// Line 5: Accelerated portion of credit (multiply line 3 by line 4)
    pub accelerated_portion_of_credit_amt: Usd,
    /// Line 6: Percentage decrease in qualified basis (decimal amount)
    pub decrease_in_qualified_basis_pct_rt: String,
    /// Line 7: Amount of accelerated portion recaptured (multiply line 5 by line 6)
    pub accelerated_prtn_recaptured_amt: Usd,

    // -----------------------------------------------------------------------
    // Lines 8–15 — Recapture Tax and Carryforward
    // -----------------------------------------------------------------------
    /// Line 8: Recapture amount from flow-through entity
    pub flow_thru_entity_recapture_amt: Usd,
    /// Line 9: Unused portion of the accelerated amount from line 7
    pub accelerated_prtn_of_unsd_credit_amt: Usd,
    /// Line 10: Net recapture (subtract line 9 from line 7 or line 8, not less than zero)
    pub net_recapture_amt: Usd,
    /// Line 11: Interest on the line 10 recapture amount
    pub interest_on_recapture_amt: Usd,
    /// Line 12: Total amount subject to recapture (add lines 10 and 11)
    pub total_subject_to_recapture_amt: Usd,
    /// Line 13: Unused credits attributable to this building reduced by the accelerated portion on line 9
    pub unused_credit_red_by_accel_prtn_amt: Usd,
    /// Line 14: Recapture tax (subtract line 13 from line 12, not less than zero)
    pub recapture_tax_amt: Usd,
    /// Line 15: Carryforward of the low-income housing credit attributable to this building
    pub carryforward_credit_amt: Usd,

    // -----------------------------------------------------------------------
    // Lines 16–17 — Section 42(j)(5) Partnerships Only
    // -----------------------------------------------------------------------
    /// Line 16: Interest on the line 7 recapture amount (section 42(j)(5) partnerships)
    pub recapture_amt: Usd,
    /// Line 17: Total recapture (add lines 7 and 16) (section 42(j)(5) partnerships)
    pub total_recapture_amt: Usd,

    // -----------------------------------------------------------------------
    // Additional / Computed
    // -----------------------------------------------------------------------
    /// Section 42(j)(5) election code
    pub section42j5_cd: String,
}
