use us_tax_brackets::TaxYear;

use crate::Usd;
use crate::forms::{Form, FormType, InputForm};

/// Input fields for IRS Schedule K-1 (Form 1065) 2025 — Partner's Share of Income, Deductions, Credits, etc.
#[derive(Debug, Clone, Default)]
pub struct Input1065ScheduleK1 {
    // -----------------------------------------------------------------------
    // Top-of-form — Final/Amended indicators
    // -----------------------------------------------------------------------
    /// Final K-1 checkbox
    pub final_k1_ind: bool,
    /// Amended K-1 checkbox
    pub amended_k1_ind: bool,

    // -----------------------------------------------------------------------
    // Part I — Information About the Partnership
    // -----------------------------------------------------------------------
    /// Item A: Partnership's employer identification number
    pub business_name_line1_txt: String,
    /// Item A (continued): Partnership's name line 2
    pub business_name_line2_txt: String,
    /// Item B: Partnership's name, address, city, state, and ZIP code
    pub partner_us_address: String,
    /// Item C: IRS center where partnership filed return
    pub service_center_where_ret_filed_cd: String,
    /// Item D: Check if this is a publicly traded partnership (PTP)
    pub publicly_traded_partnership_ind: bool,

    // -----------------------------------------------------------------------
    // Part II — Information About the Partner
    // -----------------------------------------------------------------------
    /// Item E: Partner's SSN or TIN (Do not use TIN of a disregarded entity. See instructions.)
    pub partner_ssn: String,
    /// Item E: Partner's EIN (if applicable)
    pub partner_ein: String,
    /// Item E: Missing SSN/EIN reason code
    pub missing_ssnein_reason_cd: String,
    /// Item E: Missing EIN reason code
    pub missing_ein_reason_cd: String,
    /// Item F: Partner's name, address, city, state, and ZIP code
    pub partner_foreign_address: String,
    /// Item F: Beneficial owner / person name
    pub beneficial_owner_person_nm: String,
    /// Item G: General partner or LLC member-manager checkbox
    pub general_partner_or_llc_mem_mgr_ind: bool,
    /// Item G: Limited partner or other LLC member checkbox
    pub limited_partner_or_other_llc_mem_ind: bool,
    /// Item H1: Domestic partner checkbox
    pub domestic_partner_ind: bool,
    /// Item H1: Foreign partner checkbox
    pub foreign_partner_ind: bool,
    /// Item H2: If the partner is a disregarded entity (DE), enter the partner's EIN
    pub disregarded_entity_ein: String,
    /// Item H2: Disregarded entity SSN
    pub disregarded_entity_ssn: String,
    /// Item H2: Disregarded entity indicator
    pub disregarded_entity_ind: bool,
    /// Item I1: What type of entity is this partner?
    pub entity_partner_type_desc: String,
    /// Item I2: If this partner is a retirement plan (IRA/SEP/Keogh/etc.), check here
    pub partner_retirment_plan_ind: bool,
    /// Item J: Partner's share of profit, loss, and capital — Profit beginning-of-year
    pub partners_interest_in_profits_boy_rt: String,
    /// Item J: Partner's share of profit — Profit end-of-year
    pub partners_interest_in_profits_eoy_rt: String,
    /// Item J: Partner's share of loss — Loss beginning-of-year
    pub partners_losses_boy_rt: String,
    /// Item J: Partner's share of loss — Loss end-of-year
    pub partners_losses_eoy_rt: String,
    /// Item J: Partner's share of capital — Capital beginning-of-year
    pub partners_capital_boy_rt: String,
    /// Item J: Partner's share of capital — Capital end-of-year
    pub partners_capital_eoy_rt: String,
    /// Item J: Check if decrease is due to sale of partnership interest
    pub decr_sale_prtnr_prtshp_int_ind: bool,
    /// Item J: Check if decrease is due to exchange of partnership interest
    pub decr_exch_prtnr_prtshp_int_ind: bool,
    /// Item K1: Partner's share of liabilities — Nonrecourse beginning-of-year
    pub nonrecourse_boy_amt: Usd,
    /// Item K1: Partner's share of liabilities — Nonrecourse end-of-year
    pub nonrecourse_eoy_amt: Usd,
    /// Item K1: Partner's share of liabilities — Qualified nonrecourse financing beginning-of-year
    pub qlfy_non_rcrs_financing_boy_amt: Usd,
    /// Item K1: Partner's share of liabilities — Qualified nonrecourse financing end-of-year
    pub qlfy_non_rcrs_financing_eoy_amt: Usd,
    /// Item K1: Partner's share of liabilities — Recourse beginning-of-year
    pub recourse_boy_amt: Usd,
    /// Item K1: Partner's share of liabilities — Recourse end-of-year
    pub recourse_eoy_amt: Usd,
    /// Item K2: Check this box if item K1 includes liability amounts from lower-tier partnerships
    pub lower_tier_partnership_liab_amt_ind: bool,
    /// Item K3: Check if any of the above liability is subject to guarantees or other payment
    /// obligations by the partner
    pub liab_subj_gurnt_arrngm_prtnr_ind: bool,
    /// Item L: Partner's Capital Account Analysis — Beginning capital account
    pub capital_account_boy_amt: Usd,
    /// Item L: Capital contributed during the year
    pub capital_contributed_during_yr_amt: Usd,
    /// Item L: Current year net income (loss)
    pub current_year_net_income_or_loss_amt: Usd,
    /// Item L: Other increase (decrease) (attach explanation)
    pub other_increase_decrease_amt: Usd,
    /// Item L: Withdrawals and distributions
    pub withdrawals_and_dstrbtns_amt: Usd,
    /// Item L: Ending capital account
    pub capital_account_eoy_amt: Usd,
    /// Item M: Did the partner contribute property with a built-in gain (loss)?
    pub prop_contri_built_in_gain_loss_ind: bool,
    /// Item N: Partner's Share of Net Unrecognized Section 704(c) Gain or (Loss) — Beginning
    pub net_unrcgnzd704c_gain_loss_boy_amt: Usd,
    /// Item N: Partner's Share of Net Unrecognized Section 704(c) Gain or (Loss) — Ending
    pub net_unrcgnzd704c_gain_loss_eoy_amt: Usd,

    // -----------------------------------------------------------------------
    // Part III — Partner's Share of Current Year Income, Deductions, Credits, and Other Items
    // -----------------------------------------------------------------------
    /// Line 1: Ordinary business income (loss)
    pub ordinary_income_loss_amt: Usd,
    /// Line 2: Net rental real estate income (loss)
    pub real_estate_net_income_loss_amt: Usd,
    /// Line 3: Other net rental income (loss)
    pub other_rental_income_amt: Usd,
    /// Line 4a: Guaranteed payments for services
    pub guaranteed_payments_services_amt: Usd,
    /// Line 4b: Guaranteed payments for capital
    pub guaranteed_payments_capital_amt: Usd,
    /// Line 4c: Total guaranteed payments
    pub guaranteed_pymts_to_partner_amt: Usd,
    /// Line 5: Interest income
    pub interest_income_amt: Usd,
    /// Line 6a: Ordinary dividends
    pub ordinary_dividends_amt: Usd,
    /// Line 6b: Qualified dividends
    pub qualified_dividends_amt: Usd,
    /// Line 6c: Dividend equivalents
    pub dividend_equivalents_amt: Usd,
    /// Line 7: Royalties
    pub portfolio_income_loss_rylts_amt: Usd,
    /// Line 8: Net short-term capital gain (loss)
    pub net_st_capital_gain_or_loss_amt: Usd,
    /// Line 9a: Net long-term capital gain (loss)
    pub net_lt_capital_gain_or_loss_amt: Usd,
    /// Line 9b: Collectibles (28%) gain (loss)
    pub collectibles_gain_loss_amt: Usd,
    /// Line 9c: Unrecaptured section 1250 gain
    pub unrecaptured_section1250_gain_amt: Usd,
    /// Line 10: Net section 1231 gain (loss)
    pub net_section1231_gain_loss_amt: Usd,
    /// Line 11: Other income (loss)
    pub country_or_possession_cd: String,
    /// Line 12: Section 179 deduction
    pub section179_expense_deduction_amt: Usd,

    // Line 13: Other deductions
    // (see attached statement for codes and amounts)

    // Line 14: Self-employment earnings (loss)
    // (see attached statement for codes and amounts)

    // Line 15: Credits
    // (see attached statement for codes and amounts)
    /// Line 16: Schedule K-3 is attached if checked
    pub schedule_k3_attached_ind: bool,

    // Line 17: Alternative minimum tax (AMT) items
    // (see attached statement for codes and amounts)

    // Line 18: Tax-exempt income and nondeductible expenses
    // (see attached statement for codes and amounts)

    // Line 19: Distributions
    // (see attached statement for codes and amounts)

    // Line 20: Other information
    // (see attached statement for codes and amounts)
    /// Line 21: Foreign taxes paid or accrued
    pub total_foreign_taxes_paid_or_accr_amt: Usd,
    /// Line 22: More than one activity for at-risk purposes
    pub sect465_at_risk_aggregated_acty_ind: bool,
    /// Line 23: More than one activity for passive activity purposes
    pub sect469_pal_grouped_acty_ind: bool,
    /// Section 1377(a)(2) indicator
    pub sect1377a2_indicator: bool,
}

impl Form for Input1065ScheduleK1 {
    fn name() -> &'static str {
        "Schedule K-1 (Form 1065)"
    }

    fn year(&self) -> TaxYear {
        TaxYear::Y2025
    }

    fn form_type() -> FormType {
        FormType::Input
    }
}

impl InputForm for Input1065ScheduleK1 {}
