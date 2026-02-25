use crate::Usd;

/// Output fields for IRS Form 8805 (2025) — Foreign Partner's Information Statement of Section 1446 Withholding Tax.
#[derive(Debug, Clone, Default)]
pub struct Output8805 {
    // -----------------------------------------------------------------------
    // Top-of-form — Calendar/tax year information
    // -----------------------------------------------------------------------
    /// Tax period beginning date
    pub tax_period_begin_dt: String,
    /// Tax period ending date
    pub tax_period_end_dt: String,

    // -----------------------------------------------------------------------
    // Lines 1a-1c — Foreign partner's information
    // -----------------------------------------------------------------------
    /// Line 1a: Foreign partner's name
    pub partner_person_nm: String,
    /// Line 1b: Foreign partner's U.S. identifying number (SSN)
    pub partner_ssn: String,
    /// Line 1b: Foreign partner's U.S. identifying number (EIN)
    pub partner_ein: String,
    /// Missing EIN reason code for foreign partner
    pub missing_ein_reason_cd: String,
    /// Line 1c: Foreign partner's U.S. address
    pub partner_us_address: String,
    /// Line 1c: Foreign partner's foreign address
    pub partner_foreign_address: String,

    // -----------------------------------------------------------------------
    // Line 2 — Account number
    // -----------------------------------------------------------------------
    /// Line 2: Account number assigned by partnership (if any)
    pub account_num: String,

    // -----------------------------------------------------------------------
    // Lines 3-4 — Partner type and country
    // -----------------------------------------------------------------------
    /// Line 3: Type of partner (specify)
    pub partner_type_cd: String,
    /// Line 4: Country code of partner (enter two-letter code)
    pub foreign_country_or_us_possession_cd: String,

    // -----------------------------------------------------------------------
    // Lines 5a-5c — Partnership information
    // -----------------------------------------------------------------------
    /// Line 5a: Name of partnership (line 1)
    pub business_name_line1_txt: String,
    /// Line 5a: Name of partnership (line 2)
    pub business_name_line2_txt: String,
    /// Line 5b: U.S. Employer Identification Number (EIN) of partnership
    pub partnership_ein: String,
    /// Missing EIN reason code for partnership
    pub partnership_missing_ein_reason_cd: String,
    /// Line 5c: Partnership U.S. address
    pub partnership_us_address: String,
    /// Line 5c: Partnership foreign address
    pub partnership_foreign_address: String,

    // -----------------------------------------------------------------------
    // Lines 6-7 — Withholding agent information
    // -----------------------------------------------------------------------
    /// Line 6: Withholding agent's name (enter "SAME" if partnership is also the withholding agent)
    pub beneficiary_person_nm: String,
    /// Line 6: Same as above code
    pub same_as_above_cd: String,
    /// Line 6c: Withholding agent's U.S. address
    pub us_address: String,
    /// Line 6c: Withholding agent's foreign address
    pub foreign_address: String,
    /// Line 7: Withholding agent's U.S. EIN
    pub ein: String,
    /// Missing SSN/EIN reason code for withholding agent
    pub missing_ssnein_reason_cd: String,
    /// Withholding agent's SSN
    pub ssn: String,

    // -----------------------------------------------------------------------
    // Lines 8a-8b — Partnership ownership and ECTI exemption
    // -----------------------------------------------------------------------
    /// Line 8a: Check if the partnership identified on line 5a owns an interest in one or more partnerships
    pub own_int_greater_one_partnership_ind: bool,
    /// Line 8b: Check if any of the partnership's ECTI is exempt from U.S. tax for the partner identified on line 1a
    pub ecti_exempt_ind: bool,

    // -----------------------------------------------------------------------
    // Lines 9-10 — ECTI and tax credit amounts
    // -----------------------------------------------------------------------
    /// Line 9: Partnership's ECTI allocable to partner for the tax year
    pub allocable_ecti_amt: Usd,
    /// Line 10: Total tax credit allowed to partner under section 1446
    pub total_allowed_tax_credit_amt: Usd,

    // -----------------------------------------------------------------------
    // Schedule T — Beneficiary Information
    // -----------------------------------------------------------------------
    /// Line 12: Amount of ECTI on line 9 to be included in the beneficiary's gross income
    pub allocable_ecti_incld_gross_incm_amt: Usd,
}
