use crate::Usd;

/// Output fields for IRS Form 8919 (2025) — Uncollected Social Security and Medicare Tax on Wages.
#[derive(Debug, Clone, Default)]
pub struct Output8919 {
    // -----------------------------------------------------------------------
    // Top-of-form
    // -----------------------------------------------------------------------
    /// Name of person who must file this form
    pub person_nm: String,
    /// Social security number
    pub ssn: String,

    // -----------------------------------------------------------------------
    // Firm-level detail (Lines 1-5, columns a-f)
    // -----------------------------------------------------------------------
    /// Lines 1-5: Per-firm detail (name, EIN, reason code, date, 1099-MISC/NEC indicator, wages)
    pub uncollected_soc_sec_med_tax_per_firm: String,

    // -----------------------------------------------------------------------
    // Totals (Lines 6-13)
    // -----------------------------------------------------------------------
    /// Line 6: Total wages (combine lines 1 through 5 in column f)
    pub total_wages_with_no_withholding_amt: Usd,
    /// Line 7: Maximum amount of wages subject to social security tax
    pub total_wages_and_unreported_tips_amt: Usd,
    /// Line 8: Total social security wages and social security tips (from Forms W-2, RRTA compensation, and unreported tips)
    pub net_wages_subject_to_soc_sec_tax_amt: Usd,
    /// Line 9: Subtract line 8 from line 7 (if line 8 is more than line 7, enter -0-)
    pub wages_subject_to_sst_amt: Usd,
    /// Line 10: Wages subject to social security tax (smaller of line 6 or line 9)
    /// (also entered on Form 8959, line 3)
    pub uncollected_soc_sec_tax_amt: Usd,
    /// Line 11: Multiply line 10 by 0.062 (social security tax rate)
    pub uncollected_medicare_tax_amt: Usd,
    /// Line 12: Multiply line 6 by 0.0145 (Medicare tax rate)
    pub uncollected_soc_sec_med_tax_amt: Usd,
    // Line 13: Add lines 11 and 12 (include as tax on Schedule 2, line 6)
    // (The line 13 total equals uncollected_soc_sec_med_tax_amt)
}
