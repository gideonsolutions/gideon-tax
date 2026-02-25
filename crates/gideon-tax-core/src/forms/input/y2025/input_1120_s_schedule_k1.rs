use crate::Usd;

/// Input fields for IRS Schedule K-1 (Form 1120-S) 2025 — Shareholder's Share of Income, Deductions, Credits, etc.
#[derive(Debug, Clone, Default)]
pub struct Input1120SScheduleK1 {
    // -----------------------------------------------------------------------
    // Top-of-form — Final/Amended indicators
    // -----------------------------------------------------------------------
    /// Final K-1 checkbox
    pub final_k1_ind: bool,
    /// Amended K-1 checkbox
    pub amended_k1_ind: bool,

    // -----------------------------------------------------------------------
    // Part I — Information About the Corporation
    // -----------------------------------------------------------------------
    /// Item A: Corporation's employer identification number
    pub corporation_ein: String,
    /// Item A: Missing EIN reason code
    pub ein_missing_reason_cd: String,
    /// Item B: Corporation's name, address, city, state, and ZIP code — Name line 1
    pub business_name_line1_txt: String,
    /// Item B: Corporation's name line 2
    pub business_name_line2_txt: String,
    /// Item B: Corporation's U.S. address
    pub corporation_us_address: String,
    /// Item B: Corporation's foreign address
    pub corporation_foreign_address: String,
    /// Item C: IRS Center where corporation filed return
    pub service_center_where_ret_filed_cd: String,
    /// Item D: Corporation's total number of shares — Beginning of tax year
    pub corp_tot_begin_tax_year_shares_cnt: u32,
    /// Item D: Corporation's total number of shares — End of tax year
    pub corp_tot_end_tax_year_shares_cnt: u32,

    // -----------------------------------------------------------------------
    // Part II — Information About the Shareholder
    // -----------------------------------------------------------------------
    /// Item E: Shareholder's identifying number (SSN)
    pub shareholder_ssn: String,
    /// Item E: Shareholder's identifying number (EIN)
    pub shareholder_ein: String,
    /// Item E: Missing SSN reason code
    pub missing_ssn_reason_cd: String,
    /// Item E: Missing SSN/EIN reason code
    pub missing_ssnein_reason_cd: String,
    /// Item E: Missing EIN reason code
    pub missing_ein_reason_cd: String,
    /// Item E: Shareholder name control text
    pub shareholder_name_control_txt: String,
    /// Item F1: Shareholder's name, address, city, state, and ZIP code
    pub shareholder_us_address: String,
    /// Item F1: Shareholder's foreign address
    pub shareholder_foreign_address: String,
    /// Item F2: If the shareholder is a disregarded entity, a trust, an estate, or a nominee or
    /// similar person, enter the individual or entity responsible for reporting — TIN
    pub responsible_business_ein: String,
    /// Item F2: Responsible individual name
    pub responsible_individual_nm: String,
    /// Item F2: Responsible individual SSN
    pub responsible_individual_ssn: String,
    /// Item F3: What type of entity is this shareholder?
    pub entity_type_txt: String,
    /// Item G: Current year allocation percentage
    pub current_yr_allocation_pct: String,
    /// Item H: Shareholder's number of shares — Beginning of tax year
    pub begin_tax_year_shares_cnt: u32,
    /// Item H: Shareholder's number of shares — End of tax year
    pub end_tax_year_shares_cnt: u32,
    /// Item I: Loans from shareholder — Beginning of tax year
    pub loans_from_shareholder_boy_amt: Usd,
    /// Item I: Loans from shareholder — End of tax year
    pub loans_from_shareholder_eoy_amt: Usd,

    // -----------------------------------------------------------------------
    // Part III — Shareholder's Share of Current Year Income, Deductions, Credits, and Other Items
    // -----------------------------------------------------------------------
    /// Line 1: Ordinary business income (loss)
    pub ordinary_income_loss_amt: Usd,
    /// Line 2: Net rental real estate income (loss)
    pub real_estate_net_income_loss_amt: Usd,
    /// Line 3: Other net rental income (loss)
    pub other_rental_income_amt: Usd,
    /// Line 4: Interest income
    pub interest_income_amt: Usd,
    /// Line 5a: Ordinary dividends
    pub ordinary_dividends_amt: Usd,
    /// Line 5b: Qualified dividends
    pub qualified_dividends_amt: Usd,
    /// Line 6: Royalties
    pub portfolio_income_loss_rylts_amt: Usd,
    /// Line 7: Net short-term capital gain (loss)
    pub net_st_capital_gain_or_loss_amt: Usd,
    /// Line 8a: Net long-term capital gain (loss)
    pub net_lt_capital_gain_or_loss_amt: Usd,
    /// Line 8b: Collectibles (28%) gain (loss)
    pub collectibles_gain_loss_amt: Usd,
    /// Line 8c: Unrecaptured section 1250 gain
    pub unrecaptured_section1250_gain_amt: Usd,
    /// Line 9: Net section 1231 gain (loss)
    pub net_section1231_gain_loss_amt: Usd,
    /// Line 10: Other income (loss)
    pub irs1120_s_sch_k1_other_incm_loss_cd: String,
    /// Line 11: Section 179 deduction
    pub section179_expense_deduction_amt: Usd,
    /// Line 12: Other deductions
    pub irs1120_s_sch_k1_other_ded_cd: String,
    /// Line 13: Credits
    pub irs1120_s_sch_k1_credits_cd: String,
    /// Line 14: Schedule K-3 is attached if checked
    pub schedule_k_3attached_ind: bool,
    /// Line 14: Schedule K-3 attached indicator (alternate)
    pub schedule_k3_attached_ind: bool,
    /// Line 15: Alternative minimum tax (AMT) items
    pub irs1120_s_sch_k1_amt_items_cd: String,
    /// Line 16: Items affecting shareholder basis
    pub irs1120_s_sch_k1_affectng_shr_bss_cd: String,
    /// Line 17: Other information
    pub irs1120_s_sch_k1_other_info_cd: String,
    /// Line 18: More than one activity for at-risk purposes
    pub sect465_at_risk_aggregated_acty_ind: bool,
    /// Line 19: More than one activity for passive activity purposes
    pub sect469_pal_grouped_acty_ind: bool,
    /// Section 1377(a)(2) code
    pub section1377a2_cd: String,
}
