use crate::Usd;

/// Output fields for IRS Form 8815 (2025) — Exclusion of Interest From Series EE and I U.S. Savings Bonds Issued After 1989.
#[derive(Debug, Clone, Default)]
pub struct Output8815 {
    // -----------------------------------------------------------------------
    // Line 1 — Eligible persons and institutions
    // -----------------------------------------------------------------------
    /// Line 1(a): Name of person who was enrolled at or attended an eligible educational institution
    pub eligible_person_nm: String,
    /// Line 1(b): Name of eligible educational institution
    pub eligible_institution_nm: String,
    /// Line 1(b): Address line 1 of eligible educational institution
    pub address_line1_txt: String,
    /// Line 1(b): Address line 2 of eligible educational institution
    pub address_line2_txt: String,
    /// Line 1(b): City of eligible educational institution
    pub city_nm: String,
    /// Line 1(b): State abbreviation of eligible educational institution
    pub state_abbreviation_cd: String,
    /// Line 1(b): ZIP code of eligible educational institution
    pub zip_cd: String,
    /// Line 1(b): Country code (foreign address)
    pub country_cd: String,
    /// Line 1(b): Province or state name (foreign address)
    pub province_or_state_nm: String,
    /// Line 1(b): Foreign postal code
    pub foreign_postal_cd: String,
    /// Coverdell educational savings account code
    pub coverdell_educational_sav_acct_cd: String,
    /// Qualified tuition program code
    pub qualified_tuition_program_cd: String,

    // -----------------------------------------------------------------------
    // Lines 2-14 — Exclusion computation
    // -----------------------------------------------------------------------
    /// Line 2: Enter the total qualified higher education expenses you paid in 2025
    pub excl_bond_int_tot_qlfy_educ_expns_amt: Usd,
    /// Line 3: Enter the total of any nontaxable educational benefits received for 2025
    pub excl_bond_int_tot_non_tx_educ_bnft_amt: Usd,
    /// Line 4: Subtract line 3 from line 2. If zero or less, stop
    pub excl_bond_int_txbl_educ_benefit_amt: Usd,
    /// Line 5: Enter the total proceeds (principal and interest) from all series EE and I U.S. savings bonds issued after 1989 that you cashed during 2025
    pub excl_bond_tot_py_bond_proc_amt: Usd,
    /// Line 6: Enter the interest included on line 5
    pub excl_bond_int_tot_py_bond_int_amt: Usd,
    /// Line 7: If line 4 is equal to or more than line 5, enter "1.000". If line 4 is less than line 5, divide line 4 by line 5 (decimal)
    pub excl_bond_int_txbl_expns_bond_proc_rt: String,
    /// Line 8: Multiply line 6 by line 7
    pub excl_bond_int_tentative_bond_int_amt: Usd,
    /// Line 9: Enter your modified adjusted gross income
    pub excl_bond_int_modified_agi_amt: Usd,
    /// Line 10: Enter $99,500 if single, head of household, or qualifying surviving spouse; or $149,250 if married filing jointly
    pub excl_bond_int_filing_status_lmt_amt: Usd,
    /// Line 11: Subtract line 10 from line 9. If zero or less, skip line 12, enter -0- on line 13, and go to line 14
    pub excl_bond_int_excess_agi_amt: Usd,
    /// Line 12: Divide line 11 by $15,000 if single, head of household, or qualifying surviving spouse; or $30,000 if married filing jointly (decimal)
    pub excl_bond_int_excess_agi_rt: String,
    /// Line 13: Multiply line 8 by line 12
    pub excl_bond_int_offset_amt: Usd,
    /// Line 14: Excludable savings bond interest. Subtract line 13 from line 8. Enter the result here and on Schedule B (Form 1040), line 3
    pub excludable_savings_bond_int_amt: Usd,
}
