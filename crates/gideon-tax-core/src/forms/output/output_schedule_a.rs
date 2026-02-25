use crate::Usd;

/// Output fields for IRS Form 1040 Schedule A (2025) — Itemized Deductions.
#[derive(Debug, Clone, Default)]
pub struct OutputScheduleA {
    /// Line 1: Medical and dental expenses (see instructions)
    pub medical_and_dental_expenses_amt: Usd,
    /// Line 2: Enter amount from Form 1040 or 1040-SR, line 11b
    pub tax_return_agi_amt: Usd,
    /// Line 3: Multiply line 2 by 7.5% (0.075)
    pub calculated_medical_allowance_amt: Usd,
    /// Line 4: Subtract line 3 from line 1. If line 3 is more than line 1, enter -0-
    pub total_medical_and_dental_expnss_amt: Usd,
    /// Line 5a: State and local income taxes or general sales taxes
    pub state_and_local_tax_amt: Usd,
    /// Line 5a checkbox: If you elect to include general sales taxes instead of income taxes, check this box
    pub state_and_local_sales_tax_ind: bool,
    /// Line 5b: State and local real estate taxes (see instructions)
    pub real_estate_taxes_amt: Usd,
    /// Line 5c: State and local personal property taxes
    pub personal_property_taxes_amt: Usd,
    /// Line 5d: Add lines 5a through 5c
    pub total_state_and_local_tax_amt: Usd,
    /// Line 5e: Enter the smaller of line 5d or $40,000 ($20,000 if married filing
    /// separately). If Form 1040 or 1040-SR, line 11b is more than $500,000
    /// ($250,000 if married filing separately), or if you completed Form 2555,
    /// Form 4563, or excluded income from Puerto Rico, see instructions
    pub state_and_local_tax_limitation_amt: Usd,
    /// Line 6: Other taxes. List type and amount
    pub other_taxes_amt: Usd,
    /// Line 7: Add lines 5e and 6
    pub total_taxes_paid_amt: Usd,
    /// Line 8 checkbox: If you didn't use all of your home mortgage loan(s) to buy, build, or improve your home, check this box
    pub home_mortg_not_used_ind: bool,
    /// Line 8a: Home mortgage interest and points reported to you on Form 1098. See instructions if limited
    pub rpt_home_mortg_int_and_points_amt: Usd,
    /// Line 8b: Home mortgage interest not reported to you on Form 1098. See
    /// instructions if limited. If paid to the person from whom you bought the home,
    /// see instructions and show that person's name, identifying no., and address
    pub form_1098_home_mortg_int_not_rpt_amt: Usd,
    /// Line 8c: Points not reported to you on Form 1098. See instructions for special rules
    pub form_1098_points_not_reported_amt: Usd,
    /// Line 8e: Add lines 8a through 8c
    pub total_home_mortg_int_and_points_amt: Usd,
    /// Line 9: Investment interest. Attach Form 4952 if required. See instructions
    pub investment_interest_amt: Usd,
    /// Line 10: Add lines 8e and 9
    pub total_interest_paid_amt: Usd,
    /// Line 11: Gifts by cash or check. If you made any gift of $250 or more, see instructions
    pub gifts_by_cash_or_check_amt: Usd,
    /// Line 12: Other than by cash or check. If you made any gift of $250 or more,
    /// see instructions. You must attach Form 8283 if over $500
    pub other_than_by_cash_or_check_amt: Usd,
    /// Line 13: Carryover from prior year
    pub carryover_from_prior_year_amt: Usd,
    /// Line 14: Add lines 11 through 13
    pub total_charitable_contri_amt: Usd,
    /// Line 16: Other — from list in instructions. List type and amount
    pub other_miscellaneous_ded_amt: Usd,
    /// Line 17: Add the amounts in the far right column for lines 4 through 16. Also, enter this amount
    /// on Form 1040 or 1040-SR, line 12e
    pub total_itemized_deductions_amt: Usd,
    /// Line 18: If you elect to itemize deductions even though they are less than your standard
    /// deduction, check this box
    pub itmzd_ded_less_than_std_ded_ind: bool,
    /// Calculated AGI minus total net loss amount (used in medical expense calculation)
    pub calc_adj_gro_incm_mns_tot_net_loss_amt: Usd,
    /// Form/schedule number reference
    pub form_schedule_number: String,
    /// Qualified contributions amount (attribute of Line 11)
    pub qualified_contributions_amt: Usd,
}
