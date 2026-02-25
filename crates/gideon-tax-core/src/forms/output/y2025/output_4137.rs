use crate::Usd;

/// Output fields for IRS Form 4137 (2025) — Social Security and Medicare Tax on Unreported Tip Income.
#[derive(Debug, Clone, Default)]
pub struct Output4137 {
    // -----------------------------------------------------------------------
    // Top-of-form
    // -----------------------------------------------------------------------
    /// Name of person who received tips
    pub person_nm: String,
    /// Social security number
    pub ssn: String,

    // -----------------------------------------------------------------------
    // Line 1 — Employer-level detail (per-employer rows A through E)
    // -----------------------------------------------------------------------
    /// Line 1: Unreported tip income per employer (table rows A-E with columns a-d)
    pub unreported_tip_income_per_employer: String,

    // -----------------------------------------------------------------------
    // Lines 2-13
    // -----------------------------------------------------------------------
    /// Line 2: Total cash and charge tips you received in 2025 (sum of line 1, column (c))
    pub total_tips_received_amt: Usd,
    /// Line 3: Total cash and charge tips you reported to your employer(s) in 2025 (sum of
    /// line 1, column (d))
    pub total_tips_reported_amt: Usd,
    /// Line 4: Subtract line 3 from line 2. Include as income on Form 1040, 1040-SR, or
    /// 1040-NR, line 1c
    pub total_tips_received_minus_rpt_amt: Usd,
    /// Line 5: Cash and charge tips you received but did not report to your employer because
    /// the total was less than $20 in a calendar month
    pub incidental_cash_and_tips_amt: Usd,
    /// Line 6: Unreported tips subject to Medicare tax. Subtract line 5 from line 4
    pub net_unreported_minus_incdntl_amt: Usd,
    /// Line 8: Total social security wages and social security tips (total of Form(s) W-2,
    /// boxes 3 and 7) and railroad retirement (RRTA) compensation (subject to 6.2% rate)
    pub social_security_wages_and_tips_amt: Usd,
    /// Line 9: Subtract line 8 from line 7. If line 8 is more than line 7, enter -0-
    pub net_wage_subject_to_soc_sec_tax_amt: Usd,
    /// Line 10: Unreported tips subject to social security tax. Enter the smaller of line 6
    /// or line 9
    pub unreported_tips_subj_to_soc_sec_amt: Usd,
    /// Line 10: If you received tips as a federal, state, or local government employee, code
    /// and amount for "1.45% tips"
    pub government_employee_tip_cd: String,
    /// Line 10: Government employee tip amount (tips subject only to 1.45% Medicare tax)
    pub government_employee_tip_amt: Usd,
    /// Line 11: Multiply line 10 by 0.062 (social security tax rate)
    pub social_security_tax_tip_amt: Usd,
    /// Line 12: Multiply line 6 by 0.0145 (Medicare tax rate)
    pub medicare_tax_tips_amt: Usd,
    /// Line 13: Add lines 11 and 12. Include as tax on Schedule 2 (Form 1040), line 5
    pub soc_sec_medicare_tax_unrptd_tip_amt: Usd,
}
