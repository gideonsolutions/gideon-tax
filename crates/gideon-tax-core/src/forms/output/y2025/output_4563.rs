use crate::Usd;

/// Output fields for IRS Form 4563 (2025) — Exclusion of Income for Bona Fide Residents of American Samoa.
#[derive(Debug, Clone, Default)]
pub struct Output4563 {
    // -----------------------------------------------------------------------
    // Top-of-form
    // -----------------------------------------------------------------------
    /// Name as shown on Form 1040 or 1040-SR
    pub name_line1_txt: String,
    /// Your social security number
    pub ssn: String,

    // -----------------------------------------------------------------------
    // Part I — General Information
    // -----------------------------------------------------------------------
    /// Line 1: Date bona fide residence began
    pub bona_fide_residence_begin_dt: String,
    /// Line 1: Date bona fide residence ended
    pub bona_fide_residence_end_dt: String,
    /// Line 2: Type of living quarters — Rented room
    pub rented_room_ind: bool,
    /// Line 2: Type of living quarters — Rented house or apartment
    pub rented_house_ind: bool,
    /// Line 2: Type of living quarters — Quarters furnished by employer
    pub employer_furnished_quarters_ind: bool,
    /// Line 2: Type of living quarters — Purchased home
    pub purchased_house_ind: bool,
    /// Line 3a: Did any of your family live with you in American Samoa during any part of the tax year?
    pub family_living_with_you_ind: bool,
    /// Line 4a: Did you maintain any home(s) outside American Samoa?
    pub hm_maint_outsd_american_samoa_ind: bool,
    /// Line 5: Name and address of employer (state if self-employed)
    pub employer_foreign_address: String,
    /// Line 5: Other employer foreign address
    pub employer_other_foreign_address: String,
    /// Line 5: Self-employed code
    pub self_employed_cd: String,
    /// Line 5: Business name line 1
    pub business_name_line1_txt: String,
    /// Line 5: Business name line 2
    pub business_name_line2_txt: String,
    /// Line 6: Continuation literal code for days absent table
    pub continue_literal_cd: String,

    // -----------------------------------------------------------------------
    // Part II — Figure Your Exclusion
    // -----------------------------------------------------------------------
    /// Line 7: Wages, salaries, tips, etc.
    pub wages_exclusion_amt: Usd,
    /// Line 8: Taxable interest
    pub taxable_interest_exclusion_amt: Usd,
    /// Line 9: Ordinary dividends
    pub ordinary_dividends_exclusion_amt: Usd,
    /// Line 10: Business income
    pub business_income_exclusion_amt: Usd,
    /// Line 11: Capital gain
    pub capital_gain_exclusion_amt: Usd,
    /// Line 12: Rental real estate, royalties, etc.
    pub rental_real_estate_income_excl_amt: Usd,
    /// Line 13: Farm income
    pub farm_income_exclusion_amt: Usd,
    /// Line 14: Other income. List type and amount
    pub total_other_income_exclusion_amt: Usd,
    /// Line 15: Add lines 7 through 14. This is the amount you may exclude from your gross income this tax year
    pub gross_income_exclusion_amt: Usd,
}
