use crate::Usd;

/// Output fields for IRS Schedule OI (Form 1040-NR) 2025 — Other Information.
#[derive(Debug, Clone, Default)]
pub struct OutputScheduleOi {
    // -----------------------------------------------------------------------
    // Item A — Citizenship
    // -----------------------------------------------------------------------
    /// Item A: Of what country or countries were you a citizen or national during the tax year?
    pub citizen_country_cd: String,

    // -----------------------------------------------------------------------
    // Item B — Tax Residence
    // -----------------------------------------------------------------------
    /// Item B: In what country did you claim residence for tax purposes during the tax year?
    pub foreign_tax_residence_country_cd: String,

    // -----------------------------------------------------------------------
    // Item C — Green Card Application
    // -----------------------------------------------------------------------
    /// Item C: Have you ever applied to be a green card holder (lawful permanent resident) of the
    /// United States?
    pub app_lawful_permanent_resident_ind: bool,

    // -----------------------------------------------------------------------
    // Item D — Were you ever:
    // -----------------------------------------------------------------------
    /// Item D1: A U.S. citizen?
    pub us_citizen_ind: bool,
    /// Item D2: A green card holder (lawful permanent resident) of the United States?
    pub lawful_permanent_resident_ind: bool,

    // -----------------------------------------------------------------------
    // Item E — Visa Type
    // -----------------------------------------------------------------------
    /// Item E: If you had a visa on the last day of the tax year, enter your visa type.
    /// If you didn't have a visa, enter your U.S. immigration status on the last day of the
    /// tax year. (Non-immigrant visa type)
    pub non_immigrant_visa_type_cd: String,
    /// Item E: Immigrant visa type
    pub immigrant_visa_type_cd: String,
    /// Item E: U.S. immigration status description
    pub us_immigration_status_desc: String,

    // -----------------------------------------------------------------------
    // Item F — Visa Change
    // -----------------------------------------------------------------------
    /// Item F: Have you ever changed your visa type (nonimmigrant status) or U.S. immigration
    /// status?
    pub us_immigration_stat_visa_typ_chg_ind: bool,
    /// Item F: If "Yes," indicate the date and nature of the change
    pub visa_change_dt: String,
    /// Item F: Description of the visa/immigration status change
    pub visa_change_desc: String,

    // -----------------------------------------------------------------------
    // Item G — Dates Entered and Left the United States
    // -----------------------------------------------------------------------
    /// Item G: Check the box for Canada or Mexico if you are a resident of Canada or Mexico AND
    /// commute to work in the United States at frequent intervals
    pub canada_resident_work_in_us_ind: bool,
    /// Item G: Mexico resident commuting to work in the U.S.
    pub mexico_resident_work_in_us_ind: bool,

    // -----------------------------------------------------------------------
    // Item H — Days Present in the United States
    // -----------------------------------------------------------------------
    /// Item H: Give number of days you were present in the United States during 2025
    pub physically_pres_uspy_day_cnt: u32,
    /// Item H: Give number of days you were present in the United States during 2024
    pub physically_pres_uspy_less1_day_cnt: u32,
    /// Item H: Give number of days you were present in the United States during 2023
    pub physically_pres_uspy_less2_day_cnt: u32,

    // -----------------------------------------------------------------------
    // Item I — Prior Year Return
    // -----------------------------------------------------------------------
    /// Item I: Did you file a U.S. income tax return for any prior year?
    pub us_tax_ret_filed_for_any_py_ind: bool,
    /// Item I: If "Yes," give the latest year and form number you filed
    pub return_filed_yr: u16,
    /// Item I: Form type code filed for prior year
    pub form_type_cd: String,

    // -----------------------------------------------------------------------
    // Item J — Filing Return for a Trust
    // -----------------------------------------------------------------------
    /// Item J: Are you filing a return for a trust?
    pub filing_return_for_trust_ind: bool,
    /// Item J: If "Yes," did the trust have a U.S. or foreign owner under the grantor trust rules,
    /// make a distribution or loan to a U.S. person, or receive a contribution from a U.S. person?
    pub grantor_tr_distri_loan_or_contri_ind: bool,

    // -----------------------------------------------------------------------
    // Item K — Total Compensation
    // -----------------------------------------------------------------------
    /// Item K: Did you receive total compensation of $250,000 or more during the tax year?
    pub total_comp250_k_or_more_ind: bool,
    /// Item K: If "Yes," did you use an alternative method to determine the source of this
    /// compensation?
    pub alt_basis_compensation_source_ind: bool,

    // -----------------------------------------------------------------------
    // Item L — Income Exempt From Tax (Treaty)
    // -----------------------------------------------------------------------
    /// Item L1(d): Amount of treaty tax-exempt U.S. income in current tax year
    pub treaty_tax_exempt_us_income_amt: Usd,
    /// Item L2: Were you subject to tax in a foreign country on any of the income shown in 1(d)?
    pub tx_frgn_cntry_exempt_incm_curr_ty_ind: bool,
    /// Item L3: Are you claiming treaty benefits pursuant to a Competent Authority determination?
    pub competent_authority_determ_ind: bool,

    // -----------------------------------------------------------------------
    // Item M — Section 871(d) Election
    // -----------------------------------------------------------------------
    /// Item M1: This is the first year you are making an election to treat income from real
    /// property located in the United States as effectively connected with a U.S. trade or
    /// business under section 871(d)
    pub section871d_election_ind: bool,
    /// Item M2: You have made an election in a previous year that has not been revoked, to treat
    /// income from real property located in the United States as effectively connected with a
    /// U.S. trade or business under section 871(d)
    pub py_section871d_election_not_rvk_ind: bool,
}
