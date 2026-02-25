use crate::Usd;

/// Output fields for IRS Form 8697 (2025) — Interest Computation Under the Look-Back Method for Completed Long-Term Contracts.
#[derive(Debug, Clone, Default)]
pub struct Output8697 {
    // -----------------------------------------------------------------------
    // Top-of-form — Taxpayer information
    // -----------------------------------------------------------------------
    /// Name
    pub person_nm: String,
    /// Business name line 1
    pub business_name_line1_txt: String,
    /// Business name line 2
    pub business_name_line2_txt: String,
    /// Address line 1
    pub address_line1_txt: String,
    /// Address line 2
    pub address_line2_txt: String,
    /// City
    pub city_nm: String,
    /// State abbreviation
    pub state_abbreviation_cd: String,
    /// ZIP code
    pub zip_cd: String,
    /// A: Identifying number (SSN)
    pub ssn: String,
    /// A: Identifying number (EIN)
    pub ein: String,
    /// Reason code for missing EIN
    pub missing_ein_reason_cd: String,
    /// B: Check applicable box — Corporation
    pub corporation_ind: bool,
    /// B: Check applicable box — S corporation
    pub s_corporation_ind: bool,
    /// B: Check applicable box — Individual
    pub individual_ind: bool,
    /// B: Check applicable box — Partnership
    pub partnership_ind: bool,
    /// B: Check applicable box — Estate or trust
    pub estate_or_trust_ind: bool,
    /// C: Pass-through entity name
    pub pass_through_entity_ein: String,
    /// C: Reason code for missing pass-through entity EIN
    pub missing_ein_entity_reason_cd: String,

    // -----------------------------------------------------------------------
    // Part I — Regular Method
    // -----------------------------------------------------------------------
    /// Line 1: Taxable income or loss for the prior years shown on tax return (or as previously adjusted)
    pub taxable_income_or_loss_amt: Usd,
    /// Line 1: Year ended date
    pub year_ended_dt: String,
    /// Line 2: Adjustment to income to reflect the difference between (a) actual and (b) estimated contract price and costs
    pub income_adjustment_amt: Usd,
    /// Line 3: Adjusted taxable income for look-back purposes. Combine lines 1 and 2
    pub adj_taxable_income_look_back_amt: Usd,
    /// Line 4: Income tax liability on line 3 amount using tax rates in effect for the prior years
    pub tax_liability_amt: Usd,
    /// Line 5: Income tax liability shown on return (or as previously adjusted) for the prior years
    pub federal_income_tax_liability_amt: Usd,
    /// Line 6: Increase or decrease in tax for the prior years on which interest is due (or is to be refunded). Subtract line 5 from line 4
    pub increase_or_decrease_in_tx_for_py_amt: Usd,
    /// Line 7: Interest due on increase, if any, shown on line 6
    pub interest_due_on_increase_amt: Usd,
    /// Line 8: Interest to be refunded on decrease, if any, shown on line 6
    pub interest_to_be_refunded_on_decr_amt: Usd,
    /// Line 9a: Net interest to be refunded to you
    pub net_amt_of_interest_owed_amt: Usd,
    /// Line 9b: Routing number
    pub routing_transit_num: String,
    /// Line 9c: Type — Checking account
    pub checking_account_ind: bool,
    /// Line 9c: Type — Savings account
    pub savings_account_ind: bool,
    /// Line 9d: Account number
    pub depositor_account_num: String,
    /// Line 10: Net interest you owe
    pub total_interest_due_on_increase_amt: Usd,

    // -----------------------------------------------------------------------
    // Part II — Simplified Marginal Impact Method (SMIM)
    // -----------------------------------------------------------------------
    /// Part II, Line 1: Adjustment to regular taxable income to reflect the difference between (a) actual and (b) estimated contract price and costs
    pub adjustment_to_taxable_income_amt: Usd,
    /// Part II, Line 2: Regular tax increase or decrease for prior years
    pub regular_taxable_income_adj_amt: Usd,
    /// Part II, Line 3: Adjustment to alternative minimum taxable income (AMTI) to reflect the difference between (a) actual and (b) estimated contract price and costs
    pub alt_min_taxable_income_adj_amt: Usd,
    /// Part II, Line 4: Alternative minimum tax (AMT) increase or decrease for prior years
    pub prior_year_amt_incr_or_decr_amt: Usd,
    /// Part II, Line 5: Enter the larger of line 2 or line 4
    pub total_adjustment_to_income_amt: Usd,
    /// Part II, Line 6: Overpayment ceiling
    pub overpayment_ceiling_amt: Usd,
    /// Part II, Line 7: Increase or decrease in tax for the prior years on which interest is due (or is to be refunded). Enter the smaller of line 5 or line 6
    pub prior_year_reg_tax_incr_or_decr_amt: Usd,
    /// Part II, Line 8: Interest due on increase, if any, shown on line 7
    pub total_interest_to_be_refunded_amt: Usd,
    /// Part II, Line 9: Interest to be refunded on decrease, if any, shown on line 7
    pub simplified_method_computation: String,
}
