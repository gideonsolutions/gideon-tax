use crate::Usd;

/// Output fields for IRS Schedule NEC (Form 1040-NR) 2025 — Tax on Income Not Effectively Connected With a U.S. Trade or Business.
#[derive(Debug, Clone, Default)]
pub struct OutputScheduleNec {
    // -----------------------------------------------------------------------
    // Nature of income, tax rate, and amount of income
    // (columns a through d represent different tax rates: 10%, 15%, 30%, Other)
    // -----------------------------------------------------------------------
    /// Line 7: Capital gains from line 9 of the Capital Gains and Losses section
    pub net_capital_gain_or_loss_amt: Usd,

    // -----------------------------------------------------------------------
    // Capital Gains and Losses From Sales or Exchanges of Property
    // -----------------------------------------------------------------------
    /// Total capital gains (property held less than 1 year and 1 year or more)
    pub total_capital_gain_amt: Usd,
    /// Total capital losses (property held less than 1 year and 1 year or more)
    pub total_capital_loss_amt: Usd,

    // -----------------------------------------------------------------------
    // Gambling — Canada Residents
    // -----------------------------------------------------------------------
    /// Line 10: Gambling winnings — residents of Canada only
    pub gmbl_win_canada_residents_amt: Usd,
    /// Line 11: Gambling losses — residents of Canada only (cannot exceed line 10)
    pub gmbl_loss_canada_residents_amt: Usd,
    /// Line 12: Net gambling income for residents of Canada (line 10 minus line 11)
    pub net_gmbl_canada_residents_amt: Usd,

    // -----------------------------------------------------------------------
    // Gambling — Non-Canada Residents
    // -----------------------------------------------------------------------
    /// Gambling winnings — not residents of Canada
    pub gmbl_win_not_canada_residents_amt: Usd,

    // -----------------------------------------------------------------------
    // Other income and tax
    // -----------------------------------------------------------------------
    /// Other income type description
    pub other_income_typ: String,
    /// Tax rate applied to income not effectively connected
    pub tax_rt: String,
    /// Line 15: Tax on income not effectively connected with a U.S. trade or business
    /// (enter on Form 1040-NR, line 23a)
    pub income_not_us_business_tax_amt: Usd,
}
