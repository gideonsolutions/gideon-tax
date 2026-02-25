use crate::Usd;

/// Output fields for IRS Form 1042-S (2026) — Foreign Person's U.S. Source Income Subject to Withholding.
#[derive(Debug, Clone, Default)]
pub struct Output1042S {
    // -----------------------------------------------------------------------
    // Top-of-form — Amendment / Unique Form Identifier
    // -----------------------------------------------------------------------
    /// AMENDED checkbox indicator
    pub amended_ind: bool,
    /// AMENDMENT NO.
    pub amendment_num: String,

    // -----------------------------------------------------------------------
    // Income and Withholding Information (Boxes 1–11)
    // -----------------------------------------------------------------------
    /// Box 3a/4a: Exemption code
    pub exemption_cd: String,
    /// Box 3b/4b: Tax rate
    pub tax_rt: String,
    /// Box 3: Chapter indicator — Enter "3" or "4"
    pub chapter_type_cd: String,
    /// Box 7a: Federal income tax withheld
    pub federal_income_tax_withheld_amt: Usd,
    /// UNIQUE FORM IDENTIFIER
    pub form_id: String,
    /// Box 2: Gross income
    pub gross_income_amt: Usd,
    /// Box 1: Income code
    pub income_type_cd: String,

    // -----------------------------------------------------------------------
    // Withholding Agent (Boxes 12a–12m)
    // -----------------------------------------------------------------------
    /// Box 12d: Withholding agent's name (line 1)
    pub business_name_line1_txt: String,
    /// Box 12d: Withholding agent's name (line 2)
    pub business_name_line2_txt: String,
    /// Box 12b: Ch. 3 status code
    pub chapter3_status_cd: String,
    /// Box 12c: Ch. 4 status code
    pub chapter4_status_cd: String,
    /// Box 12f: Country code
    pub country_cd: String,
    /// Box 12a: Withholding agent's EIN
    pub ein: String,
    /// Box 12g: FTIN, if any
    pub ftin: String,
    /// Box 12h: Address (number and street)
    pub address_line1_txt: String,
    /// Box 12h: Address (line 2)
    pub address_line2_txt: String,
    /// Box 12j: City or town
    pub city_nm: String,
    /// Box 12m: ZIP or foreign postal code
    pub foreign_postal_cd: String,
    /// Box 12k: State or province
    pub province_or_state_nm: String,
    /// Box 12e: Withholding agent's global intermediary identification number (GIIN)
    pub giin: String,

    // -----------------------------------------------------------------------
    // Recipient (Boxes 13a–13p)
    // -----------------------------------------------------------------------
    /// Box 13a: Recipient's name
    pub person_nm: String,
    /// Box 13i: Recipient's U.S. TIN, if any
    pub tin: String,
    /// Box 13c–13h: Recipient's U.S. address
    pub us_address: String,
    /// Box 6: Net income
    pub net_income_amt: Usd,
    /// Box 8: Tax withheld by other agents
    pub other_agents_tax_withheld_amt: Usd,

    // -----------------------------------------------------------------------
    // State Tax Information (Box 17)
    // -----------------------------------------------------------------------
    /// Box 17b: Payer's state tax no.
    pub payer_state_id_num: String,

    // -----------------------------------------------------------------------
    // Reporting Indicators (Boxes 7b–7d, 15)
    // -----------------------------------------------------------------------
    /// Box 15: Check if pro-rata basis reporting
    pub pro_rata_basis_ind: bool,
    /// Box 7d: Check if qualified intermediary, withholding foreign partnership, or withholding foreign trust revising its reporting on Form 1042-S
    pub qi_or_wp_or_wt_revising_reporting_ind: bool,
    /// Box 13o: Recipient's account number
    pub account_num: String,
    /// Box 13p: Recipient's date of birth (YYYYMMDD)
    pub birth_dt: String,
    /// Box 13n: LOB code
    pub lob_cd: String,
    /// Box 9: Overwithheld tax repaid to recipient pursuant to adjustment procedures
    pub recipient_repaid_amt: Usd,
    /// Box 17c: Name of state
    pub state_cd: String,
    /// Box 17a: State income tax withheld
    pub state_tax_withheld_amt: Usd,
    /// Box 7c: Check if withholding occurred in subsequent year with respect to a partnership interest
    pub subsq_yr_withholding_prtshp_int_ind: bool,
    /// Box 7b: Check if federal tax withheld was not deposited with the IRS because escrow procedures were applied
    pub tax_not_deposit_per_escrow_proc_ind: bool,
    /// Box 10: Total withholding credit (combine boxes 7a, 8, and 9)
    pub total_tax_withholding_credit_amt: Usd,
    /// Box 11: Tax paid by withholding agent (amounts not withheld)
    pub withholding_agent_tax_paid_amt: Usd,
    /// Box 5: Withholding allowance
    pub withholding_allowances_amt: Usd,
}
