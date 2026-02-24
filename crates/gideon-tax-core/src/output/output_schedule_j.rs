use crate::Usd;

/// Output fields for IRS Schedule J (Form 1040) — Income Averaging for
/// Individuals With Income From Farming or Fishing (2025).
#[derive(Debug, Clone, Default)]
pub struct OutputScheduleJ {
    /// Line 1: Enter the taxable income from your 2025 Form 1040, 1040-SR, or
    /// 1040-NR, line 15
    pub taxable_income_amt: Usd,
    /// Line 2a: Enter your elected farm income. See instructions. Do not enter
    /// more than the amount on line 1
    pub elected_farm_income_amt: Usd,
    /// Line 2b: Excess, if any, of net long-term capital gain over net
    /// short-term capital loss
    pub excess_net_long_term_cap_gain_amt: Usd,
    /// Line 2c: Unrecaptured section 1250 gain
    pub unrecaptured_property_gain_amt: Usd,
    /// Line 3: Subtract line 2a from line 1
    pub net_income_amt: Usd,
    /// Line 4: Figure the tax on the amount on line 3 using the 2025 tax
    /// rates. See instructions
    pub current_tax_amt: Usd,
    /// Line 5: Taxable income from 2022 return (or prior Schedule J line, as
    /// applicable). If zero or less, see instructions
    pub third_py_taxable_income_amt: Usd,
    /// Line 6: Divide the amount on line 2a by 3.0
    pub third_py_average_income_amt: Usd,
    /// Line 7: Combine lines 5 and 6. If zero or less, enter -0-
    pub third_py_net_income_amt: Usd,
    /// Line 8: Figure the tax on the amount on line 7 using the 2022 tax
    /// rates. See instructions
    pub third_py_tax_table_amt: Usd,
    /// Line 9: Taxable income from 2023 return (or prior Schedule J line, as
    /// applicable). If zero or less, see instructions
    pub second_py_taxable_income_amt: Usd,
    /// Line 11: Combine lines 9 and 10. If less than zero, enter as a negative
    /// amount
    pub second_py_net_income_amt: Usd,
    /// Line 12: Figure the tax on the amount on line 11 using the 2023 tax
    /// rates. See instructions
    pub second_py_tax_table_amt: Usd,
    /// Line 13: Taxable income from 2024 return (or prior Schedule J line, as
    /// applicable). If zero or less, see instructions
    pub first_py_taxable_income_amt: Usd,
    /// Line 15: Combine lines 13 and 14. If less than zero, enter as a
    /// negative amount
    pub first_py_net_income_amt: Usd,
    /// Line 16: Figure the tax on the amount on line 15 using the 2024 tax
    /// rates. See instructions
    pub first_py_tax_table_amt: Usd,
    /// Line 17: Add lines 4, 8, 12, and 16
    pub total_tax_table_amt: Usd,
    /// Line 18: Amount from line 17
    pub gross_farm_income_tax_amt: Usd,
    /// Line 19: Tax from 2022 return (or prior Schedule J line, as applicable)
    pub tentative_tax_3rd_py_rtn_amt: Usd,
    /// Line 20: Tax from 2023 return (or prior Schedule J line, as applicable)
    pub tentative_tax_2nd_py_rtn_amt: Usd,
    /// Line 21: Tax from 2024 return (or prior Schedule J line, as applicable)
    pub tentative_tax_1st_py_rtn_amt: Usd,
    /// Line 22: Add lines 19 through 21
    pub average_farm_income_tax_amt: Usd,
}
