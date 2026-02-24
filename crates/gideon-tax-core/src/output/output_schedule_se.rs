use crate::Usd;

/// Output fields for IRS Schedule SE (Form 1040) — Self-Employment Tax (2025).
#[derive(Debug, Clone, Default)]
pub struct OutputScheduleSe {
    // -----------------------------------------------------------------------
    // Header
    // -----------------------------------------------------------------------
    /// Name of person with self-employment income (as shown on Form 1040,
    /// 1040-SR, 1040-SS, or 1040-NR)
    pub person_nm: String,
    /// Social security number of person with self-employment income
    pub ssn: String,

    // -----------------------------------------------------------------------
    // Part I — Self-Employment Tax
    // -----------------------------------------------------------------------
    /// Line A: If you are a minister, member of a religious order, or Christian
    /// Science practitioner and you filed Form 4361, but you had $400 or more
    /// of other net earnings from self-employment, check here and continue with
    /// Part I
    pub exempt_form_4361_ind: bool,
    /// Line 1a: Net farm profit or (loss) from Schedule F, line 34, and farm
    /// partnerships, Schedule K-1 (Form 1065), box 14, code A
    pub net_farm_profit_loss_amt: Usd,
    /// Line 1b: If you received social security retirement or disability
    /// benefits, enter the amount of Conservation Reserve Program payments
    /// included on Schedule F, line 4b, or listed on Schedule K-1 (Form 1065),
    /// box 20, code AQ
    pub conservation_reserve_prog_pymt_amt: Usd,
    /// Line 2: Net profit or (loss) from Schedule C, line 31; and Schedule K-1
    /// (Form 1065), box 14, code A (other than farming). See instructions for
    /// other income to report or if you are a minister or member of a religious
    /// order
    pub net_non_farm_profit_loss_amt: Usd,
    /// Line 3: Combine lines 1a, 1b, and 2
    pub se_total_net_earnings_or_loss_amt: Usd,
    /// Line 3 attribute: Additional income or loss amount
    pub additional_income_or_loss_amt: Usd,
    /// Line 3 attribute: Additional income or loss code
    pub additional_income_or_loss_cd: String,
    /// Line 3 attribute: Chapter 11 bankruptcy income amount
    pub chap_11_bankruptcy_income_amt: Usd,
    /// Line 3 attribute: Chapter 11 bankruptcy income code
    pub chap_11_bankruptcy_income_cd: String,
    /// Line 3 attribute: Community income taxed to spouse amount
    pub community_incm_taxed_to_spouse_amt: Usd,
    /// Line 3 attribute: Community income taxed to spouse code
    pub community_incm_taxed_to_spouse_cd: String,
    /// Line 3 attribute: Exempt community income amount
    pub exempt_community_income_amt: Usd,
    /// Line 3 attribute: Exempt community income code
    pub exempt_community_income_cd: String,
    /// Line 3 attribute: Self-employment tax exempt amount
    pub self_employment_tax_exempt_amt: Usd,
    /// Line 3 attribute: Self-employment tax exempt code
    pub self_employment_tax_exempt_cd: String,
    /// Short Schedule SE, line 3 attribute: Additional income or loss amount
    pub short_additional_income_or_loss_amt: Usd,
    /// Short Schedule SE, line 3 attribute: Additional income or loss code
    pub short_additional_income_or_loss_cd: String,
    /// Short Schedule SE, line 3 attribute: Chapter 11 bankruptcy income amount
    pub short_chap_11_bankruptcy_income_amt: Usd,
    /// Short Schedule SE, line 3 attribute: Chapter 11 bankruptcy income code
    pub short_chap_11_bankruptcy_income_cd: String,
    /// Short Schedule SE, line 3 attribute: Community income taxed to spouse
    /// amount
    pub short_community_incm_taxed_to_spouse_amt: Usd,
    /// Short Schedule SE, line 3 attribute: Community income taxed to spouse
    /// code
    pub short_community_incm_taxed_to_spouse_cd: String,
    /// Short Schedule SE, line 3 attribute: Exempt community income amount
    pub short_exempt_community_income_amt: Usd,
    /// Short Schedule SE, line 3 attribute: Exempt community income code
    pub short_exempt_community_income_cd: String,
    /// Short Schedule SE, line 3 attribute: Self-employment tax exempt amount
    pub short_self_employment_tax_exempt_amt: Usd,
    /// Short Schedule SE, line 3 attribute: Self-employment tax exempt code
    pub short_self_employment_tax_exempt_cd: String,
    /// Line 4a: If line 3 is more than zero, multiply line 3 by 92.35%
    /// (0.9235). Otherwise, enter amount from line 3
    pub minimum_profit_for_se_tax_amt: Usd,
    /// Line 4b: If you elect one or both of the optional methods, enter the
    /// total of lines 15 and 17 here
    pub optional_method_amt: Usd,
    /// Line 4c: Combine lines 4a and 4b. If less than $400, stop; you don't
    /// owe self-employment tax. Exception: If less than $400 and you had church
    /// employee income, enter -0- and continue
    pub combined_se_amt: Usd,
    /// Line 5a: Enter your church employee income from Form W-2. See
    /// instructions for definition of church employee income
    pub w2_wages_from_churches_amt: Usd,
    /// Line 5b: Multiply line 5a by 92.35% (0.9235). If less than $100,
    /// enter -0-
    pub min_allowable_church_wages_amt: Usd,
    /// Line 6: Add lines 4c and 5b
    pub combined_se_and_church_wages_amt: Usd,
    /// Line 7: Maximum amount of combined wages and self-employment earnings
    /// subject to social security tax or the 6.2% portion of the 7.65%
    /// railroad retirement (tier 1) tax for 2025
    pub se_base_amt: Usd,
    /// Line 8a: Total social security wages and tips (total of boxes 3 and 7
    /// on Form(s) W-2) and railroad retirement (tier 1) compensation. If
    /// $176,100 or more, skip lines 8b through 10, and go to line 11
    pub sst_wages_rrt_comp_amt: Usd,
    /// Line 8b: Unreported tips subject to social security tax from Form 4137,
    /// line 10
    pub unreported_tips_amt: Usd,
    /// Line 8c: Wages subject to social security tax from Form 8919, line 10
    pub wages_subject_to_sst_amt: Usd,
    /// Line 8d: Add lines 8a, 8b, and 8c
    pub total_wages_and_unreported_tips_amt: Usd,
    /// Line 9: Subtract line 8d from line 7. If zero or less, enter -0- here
    /// and on line 10 and go to line 11
    pub tax_base_amt: Usd,
    /// Line 10: Multiply the smaller of line 6 or line 9 by 12.4% (0.124)
    pub allowable_se_amt: Usd,
    /// Line 12: Self-employment tax. Add lines 10 and 11. Enter here and on
    /// Schedule 2 (Form 1040), line 4, or Form 1040-SS, Part I, line 3
    pub self_employment_tax_amt: Usd,
    /// Line 13: Deduction for one-half of self-employment tax. Multiply
    /// line 12 by 50% (0.50). Enter here and on Schedule 1 (Form 1040),
    /// line 15
    pub deductible_self_employment_tax_amt: Usd,

    // -----------------------------------------------------------------------
    // Part II — Optional Methods To Figure Net Earnings
    // -----------------------------------------------------------------------
    /// Line 15: Enter the smaller of: two-thirds (2/3) of gross farm income
    /// (not less than zero) or $7,240. Also, include this amount on line 4b
    /// above
    pub se_tax_farm_optional_method_amt: Usd,
    /// Line 16: Subtract line 15 from line 14
    pub se_tax_non_farm_optional_base_amt: Usd,
    /// Line 17: Enter the smaller of: two-thirds (2/3) of gross nonfarm income
    /// (not less than zero) or the amount on line 16. Also, include this amount
    /// on line 4b above
    pub se_tax_non_farm_optional_method_amt: Usd,
}
