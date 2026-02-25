use crate::Usd;

/// Output fields for IRS Form 1040 Schedule B (2025) — Interest and Ordinary Dividends.
#[derive(Debug, Clone, Default)]
pub struct OutputScheduleB {
    // -----------------------------------------------------------------------
    // Part I — Interest
    // -----------------------------------------------------------------------
    /// Line 1: List name of payer (seller-financed mortgage interest amount)
    pub total_seller_financed_mortg_int_amt: Usd,
    /// Line 2: Add the amounts on line 1
    pub interest_subtotal_amt: Usd,
    /// Line 3: Excludable interest on series EE and I U.S. savings bonds issued after 1989.
    /// Attach Form 8815
    pub excludable_savings_bond_int_amt: Usd,
    /// Line 4: Subtract line 3 from line 2. Enter the result here and on Form 1040 or 1040-SR, line 2b
    pub calculated_total_taxable_int_amt: Usd,
    /// Line 2 subtotal literal code (adjustment type)
    pub interest_subtotal_literal_cd: String,
    /// Nominee interest amount (adjustment to Line 2)
    pub nominee_interest_amt: Usd,
    /// Nominee interest literal code
    pub nominee_interest_literal_cd: String,
    /// Accrued interest amount (adjustment to Line 2)
    pub accrued_interest_amt: Usd,
    /// Accrued interest literal code
    pub accrued_interest_literal_cd: String,
    /// Accrued market discount amount (adjustment to Line 2)
    pub accrued_market_discount_amt: Usd,
    /// Accrued market discount literal code
    pub accrued_market_discount_literal_cd: String,
    /// Original issue discount adjustment amount (adjustment to Line 2)
    pub original_issue_discount_adj_amt: Usd,
    /// Original issue discount adjustment literal code
    pub original_issue_discount_adj_lit_cd: String,
    /// Amortizable bond premium adjustment amount (adjustment to Line 2)
    pub amortizable_bond_prem_adj_amt: Usd,
    /// Amortizable bond premium adjustment literal code
    pub amortizable_bond_premium_adj_lit_cd: String,
    /// Taxable interest subtotal (intermediate calculation)
    pub taxable_interest_subtotal_amt: Usd,
    /// Form 8814 literal code (election to report child's interest)
    pub form_8814_literal_cd: String,

    // -----------------------------------------------------------------------
    // Part II — Ordinary Dividends
    // -----------------------------------------------------------------------
    /// Line 5: List name of payer (ordinary dividend subtotal)
    pub ordinary_dividend_subtotal_amt: Usd,
    /// Line 5 subtotal literal code
    pub dividend_subtotal_literal_cd: String,
    /// Line 6: Add the amounts on line 5. Enter the total here and on Form 1040 or 1040-SR, line 3b
    pub total_ordinary_dividends_amt: Usd,
    /// Nominee dividend amount (adjustment to Line 6)
    pub nominee_dividend_amt: Usd,
    /// Nominee dividend literal code
    pub nominee_dividend_literal_cd: String,
    /// Restricted stock dividend amount (adjustment to Line 6)
    pub restricted_stock_dividend_amt: Usd,
    /// Restricted stock dividend literal code
    pub restricted_stock_dividend_literal_cd: String,

    // -----------------------------------------------------------------------
    // Part III — Foreign Accounts and Trusts
    // -----------------------------------------------------------------------
    /// Line 7a: At any time during 2025, did you have a financial interest in or signature authority
    /// over a financial account (such as a bank account, securities account, or brokerage account)
    /// located in a foreign country? See instructions
    pub foreign_accounts_question_ind: bool,
    /// Line 7a follow-up: If "Yes," are you required to file FinCEN Form 114, Report of Foreign Bank
    /// and Financial Accounts (FBAR), to report that financial interest or signature authority?
    /// See FinCEN Form 114 and its instructions for filing requirements and exceptions to those requirements
    pub fin_cen_form_114_ind: bool,
    /// Line 7b: If you are required to file FinCEN Form 114, list the name(s) of the foreign
    /// country(-ies) where the financial account(s) is (are) located
    pub foreign_country_cd: String,
    /// Line 8: During 2025, did you receive a distribution from, or were you the grantor of, or
    /// transferor to, a foreign trust? If "Yes," you may have to file Form 3520. See instructions
    pub foreign_trust_question_ind: bool,
    /// Trust form literal code (Form 3520 reference)
    pub trust_form_literal_cd: String,
}
