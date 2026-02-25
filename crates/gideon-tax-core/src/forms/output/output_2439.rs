use crate::Usd;

/// Output fields for IRS Form 2439 (Rev. 11-2021) — Notice to Shareholder of Undistributed Long-Term Capital Gains.
#[derive(Debug, Clone, Default)]
pub struct Output2439 {
    // -----------------------------------------------------------------------
    // Boxes 1a–1d — Undistributed Capital Gains
    // -----------------------------------------------------------------------
    /// Box 1c: Section 1202 gain
    pub capital_gain_sect1202_amt: Usd,
    /// Box 1d: Collectibles (28%) gain
    pub collectibles28_percent_gain_amt: Usd,

    // -----------------------------------------------------------------------
    // Top-of-form — VOID / CORRECTED
    // -----------------------------------------------------------------------
    /// CORRECTED checkbox indicator
    pub corrected_ind: bool,

    // -----------------------------------------------------------------------
    // RIC or REIT Information
    // -----------------------------------------------------------------------
    /// Missing EIN reason code (for RIC or REIT)
    pub missing_ein_reason_cd: String,
    /// Identification number of RIC or REIT
    pub ric_or_reitein: String,
    /// Address (number and street) — RIC or REIT
    pub address_line1_txt: String,
    /// Address line 2 — RIC or REIT
    pub address_line2_txt: String,
    /// City — RIC or REIT
    pub city_nm: String,
    /// Country code — RIC or REIT
    pub country_cd: String,
    /// Foreign postal code — RIC or REIT
    pub foreign_postal_cd: String,
    /// Province or state — RIC or REIT
    pub province_or_state_nm: String,
    /// State abbreviation code — RIC or REIT
    pub state_abbreviation_cd: String,
    /// ZIP code — RIC or REIT
    pub zip_cd: String,
    /// Name of RIC or REIT (line 1)
    pub business_name_line1_txt: String,
    /// Name of RIC or REIT (line 2)
    pub business_name_line2_txt: String,

    // -----------------------------------------------------------------------
    // Shareholder Information
    // -----------------------------------------------------------------------
    /// Shareholder's identifying number (EIN)
    pub shareholder_ein: String,
    /// Shareholder's missing EIN reason code
    pub shareholder_missing_ein_reason_cd: String,
    /// Shareholder's name
    pub shareholder_person_nm: String,
    /// Shareholder's identifying number (SSN)
    pub shareholder_ssn: String,

    // -----------------------------------------------------------------------
    // Box 2 — Tax Paid
    // -----------------------------------------------------------------------
    /// Box 2: Tax paid by the RIC or REIT on the box 1a gains
    pub tax_paid_by_ric_or_reit_amt: Usd,

    // -----------------------------------------------------------------------
    // Tax Period
    // -----------------------------------------------------------------------
    /// Tax period beginning date
    pub tax_period_begin_dt: String,
    /// Tax period ending date
    pub tax_period_end_dt: String,

    // -----------------------------------------------------------------------
    // Box 1a — Total Undistributed Long-Term Capital Gains
    // -----------------------------------------------------------------------
    /// Box 1a: Total undistributed long-term capital gains
    pub total_undistributed_lt_cap_gain_amt: Usd,
    /// Box 1b: Unrecaptured section 1250 gain
    pub unrecaptured_section1250_gain_amt: Usd,

    // -----------------------------------------------------------------------
    // VOID indicator
    // -----------------------------------------------------------------------
    /// VOID checkbox indicator
    pub void_ind: bool,
}
