use crate::Usd;

/// Output fields for IRS Form 4684 (2025) — Casualties and Thefts.
#[derive(Debug, Clone, Default)]
pub struct Output4684 {
    // -----------------------------------------------------------------------
    // Section A — Personal Use Property
    // -----------------------------------------------------------------------
    /// Section A: Federally declared disaster indicator (check here if loss is attributable to a federally declared disaster)
    pub federally_declared_disaster_ind: bool,
    /// Section A: FEMA disaster declaration number (DR- or EM-)
    pub fema_disaster_declaration_num: String,
    /// Section A: Description of federally declared disaster
    pub federally_declared_disaster_desc: String,
    /// Section A: US address (ZIP code for property most affected)
    pub us_address: String,
    /// Line 1: Description of personal use properties (type, location, ZIP code, date acquired)
    pub personal_use_properties: String,
    /// Line 2: Cost or other basis of each property
    pub cost_or_other_basis_amt: Usd,
    /// Line 3: Insurance or other reimbursement (whether or not you filed a claim)
    pub insurance_or_oth_reimbursement_amt: Usd,
    /// Line 4: Gain from casualty or theft (if line 3 is more than line 2)
    pub gain_from_casualty_or_theft_amt: Usd,
    /// Line 5: Fair market value before casualty or theft
    pub fair_market_value_before_theft_amt: Usd,
    /// Line 6: Fair market value after casualty or theft
    pub fair_market_value_after_theft_amt: Usd,
    /// Line 7: Subtract line 6 from line 5 (net fair market value decrease)
    pub net_fair_market_value_amt: Usd,
    /// Line 8: Enter the smaller of line 2 or line 7
    pub smllr_of_cost_or_net_fair_mrkt_vl_amt: Usd,
    /// Line 9: Subtract line 3 from line 8. If zero or less, enter -0-
    pub property_minus_insurance_value_amt: Usd,
    /// Line 10: Casualty or theft loss. Add the amounts on line 9 in columns A through D
    pub total_prsnl_property_theft_loss_amt: Usd,
    /// Line 11: Enter $100 ($500 if qualified disaster loss rules apply)
    pub casualty_or_theft_loss_limit_amt: Usd,
    /// Line 12: Subtract line 11 from line 10. If zero or less, enter -0-
    pub net_casualty_or_theft_loss_amt: Usd,
    /// Line 13: Add the amounts on line 4 of all Forms 4684
    pub total_casualty_and_theft_gain_amt: Usd,
    /// Line 14: Add the amounts on line 12 of all Forms 4684
    pub total_net_casualty_or_theft_loss_amt: Usd,
    /// Line 15: Casualty or theft gain/loss difference (if line 13 is more than line 14)
    pub total_theft_gain_less_total_loss_amt: Usd,
    /// Line 16: Add lines 13 and 15. Subtract the result from line 14
    pub total_loss_less_total_theft_gain_amt: Usd,
    /// Line 17: Enter 10% of your adjusted gross income from Form 1040, 1040-SR, or 1040-NR, line 11b
    pub ten_percent_of_agi_amt: Usd,
    /// Line 18: Subtract line 17 from line 16. If zero or less, enter -0-
    pub calc_adj_gro_incm_mns_tot_net_loss_amt: Usd,
    /// Passive activity loss literal code
    pub passive_activity_loss_literal_cd: String,
    /// See attached statement code
    pub see_attached_statement_cd: String,
    /// Revenue Procedure 2009-20 code
    pub revenue_procedure200920_cd: String,
    /// Revenue Procedure 2010-36 code
    pub revenue_procedure201036_cd: String,

    // -----------------------------------------------------------------------
    // Section B — Business and Income-Producing Property
    // -----------------------------------------------------------------------
    // Part I — Casualty or Theft Gain or Loss
    // -----------------------------------------------------------------------
    /// Line 19: Description of business properties (type, location, date acquired)
    pub business_properties: String,
    /// Line 20: Cost or adjusted basis of each property
    pub cost_or_adjusted_basis_amt: Usd,
    /// Line 21: Insurance or other reimbursement (whether or not you filed a claim)
    pub smllr_of_adj_or_net_fair_mrkt_vl_amt: Usd,
    /// Line 28: Casualty or theft loss per property. Enter total here and on line 29 or line 34
    pub total_bus_property_theft_loss_amt: Usd,

    // -----------------------------------------------------------------------
    // Section B, Part II — Summary of Gains and Losses (from separate Parts I)
    // -----------------------------------------------------------------------
    // Casualty or Theft of Property Held One Year or Less
    // -----------------------------------------------------------------------
    /// Line 29: Short-term casualty or theft property descriptions
    pub short_term_theft_property: String,
    /// Line 30: Short-term totals of gains from casualties or thefts
    pub short_term_total_gains_theft_amt: Usd,
    /// Line 31: Combine line 30, columns (b)(i) and (c). Enter the net gain or (loss) here and on Form 4797, line 14
    pub short_term_prop_net_gain_or_loss_amt: Usd,
    /// Line 32: Enter the amount from line 30, column (b)(ii) — income-producing property loss
    pub short_term_prop_income_prod_tot_amt: Usd,
    /// Line 30/31: Total short-term trade or business amounts
    pub tot_short_term_trade_or_business_amt: Usd,

    // -----------------------------------------------------------------------
    // Casualty or Theft of Property Held More Than One Year
    // -----------------------------------------------------------------------
    /// Line 33: Casualty or theft gains from Form 4797, line 32
    pub casualty_or_theft_gain_from4797_amt: Usd,
    /// Line 34: Long-term casualty or theft property descriptions
    pub long_term_theft_property: String,
    /// Line 35: Long-term totals
    pub long_term_total_gains_theft_amt: Usd,
    /// Line 36: Total gains. Add lines 33 and 34, column (c)
    pub long_term_prop_income_plus_gain_amt: Usd,
    /// Line 37: Add amounts on line 35, columns (b)(i) and (b)(ii)
    pub long_term_prop_income_prod_tot_amt: Usd,
    /// Line 38a: Combine line 35, column (b)(i), and line 36, and enter the net gain or (loss)
    pub long_term_prop_net_gain_or_loss_amt: Usd,
    /// Line 38b: Enter the amount from line 35, column (b)(ii) — income-producing property loss
    pub net_business_property_loss_amt: Usd,
    /// Line 39: If the loss on line 37 is less than or equal to the gain on line 36, combine lines 36 and 37
    pub long_term_trade_or_business_tot_amt: Usd,
}
