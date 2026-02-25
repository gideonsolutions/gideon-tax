use crate::Usd;

/// Output fields for IRS Form 4797 (2025) — Sales of Business Property.
#[derive(Debug, Clone, Default)]
pub struct Output4797 {
    // -----------------------------------------------------------------------
    // Top-of-form
    // -----------------------------------------------------------------------
    /// Name(s) shown on return
    pub business_name_line1_txt: String,
    /// Name(s) shown on return (line 2)
    pub business_name_line2_txt: String,
    /// Identifying number (EIN)
    pub ein: String,
    /// Missing EIN reason code
    pub missing_ein_reason_cd: String,
    /// Not applicable code
    pub not_applicable_cd: String,
    /// Passive activity loss literal code
    pub passive_activity_loss_literal_cd: String,
    /// Line 1a: Enter the gross proceeds from sales or exchanges reported on Form(s) 1099-B or 1099-S
    pub current_year_gross_proceeds_amt: Usd,
    /// Line 1b: Enter the total amount of gain from partial dispositions of MACRS assets
    pub partl_dispos_macrs_ast_tot_gain_amt: Usd,
    /// Line 1c: Enter the total amount of loss from partial dispositions of MACRS assets
    pub partl_dispos_macrs_ast_tot_loss_amt: Usd,

    // -----------------------------------------------------------------------
    // Part I — Sales or Exchanges of Property Used in a Trade or Business and
    // Involuntary Conversions From Other Than Casualty or Theft — Most
    // Property Held More Than 1 Year
    // -----------------------------------------------------------------------
    /// Line 2: Property sale or exchange descriptions (description, date acquired, date sold, gross sales price, depreciation, cost/basis, gain/loss)
    pub property_sale_or_exchange: String,
    /// Line 3: Gain, if any, from Form 4684, line 39
    pub gain_form4684_amt: Usd,
    /// Line 4: Section 1231 gain from installment sales from Form 6252, line 26 or 37
    pub gain_installment_sales_frm6252_amt: Usd,
    /// Line 5: Section 1231 gain or (loss) from like-kind exchanges from Form 8824
    pub gain_loss_form8824_amt: Usd,
    /// Line 6: Gain, if any, from line 32, from other than casualty or theft
    pub gain_oth_than_casualty_or_theft_amt: Usd,
    /// Line 7: Combine lines 2 through 6. Enter the gain or (loss) here
    pub total_property_gain_loss_amt: Usd,
    /// Line 8: Nonrecaptured net section 1231 losses from prior years
    pub nonrecaptured_net1231_losses_amt: Usd,
    /// Line 9: Subtract line 8 from line 7. If zero or less, enter the gain from line 7 on line 12 below
    pub prop_gain_nonrecaptured_loss_amt: Usd,

    // -----------------------------------------------------------------------
    // Part II — Ordinary Gains and Losses
    // -----------------------------------------------------------------------
    /// Line 10: Ordinary gains and losses not included on lines 11 through 16 (include property held 1 year or less)
    pub ordinary_gain_loss: String,
    /// Line 11: Loss, if any, from line 7
    pub ordinary_loss_amt: Usd,
    /// Line 12: Gain, if any, from line 7 or amount from line 8, if applicable
    pub net_gain_amt: Usd,
    /// Line 13: Gain, if any, from line 31
    pub net_gain_loss_form4684_amt: Usd,
    /// Line 14: Net gain or (loss) from Form 4684, lines 31 and 38a
    pub form4684_loss_amt: Usd,
    /// Line 15: Ordinary gain from installment sales from Form 6252, line 25 or 36
    pub ordnry_gain_instal_sale_frm6252_amt: Usd,
    /// Line 16: Ordinary gain or (loss) from like-kind exchanges from Form 8824
    pub ordinary_gain_loss_form8824_amt: Usd,
    /// Line 17: Combine lines 10 through 16
    pub total_ordinary_gain_loss_amt: Usd,
    /// Line 18a: For individual returns — loss from income-producing property on Schedule A (Form 1040), line 16
    pub other_gain_loss_amt: Usd,
    /// Line 18b: Redetermine the gain or (loss) on line 17 excluding the loss, if any, on line 18a
    pub total_gain_loss_amt: Usd,

    // -----------------------------------------------------------------------
    // Part III — Gain From Disposition of Property Under Sections 1245,
    // 1250, 1252, 1254, and 1255
    // -----------------------------------------------------------------------
    /// Line 19: Description of section 1245, 1250, 1252, 1254, or 1255 property
    pub property_desc: String,
    /// Line 19: Property disposition gain detail (descriptions, dates, amounts)
    pub property_disposition_gain: String,
    /// Line 19: Date acquired
    pub acquired_dt: String,
    /// Line 19: Date acquired — Inherited code
    pub date_acquired_inherited_cd: String,
    /// Line 19: Various code
    pub various_cd: String,
    /// Line 19: Date sold
    pub sold_dt: String,
    /// Line 20: Gross sales price (Note: See line 1a before completing)
    pub gross_sales_price_amt: Usd,
    /// Line 21: Cost or other basis plus expense of sale
    pub cost_or_other_basis_expense_sale_amt: Usd,
    /// Line 22: Depreciation (or depletion) allowed or allowable
    pub depreciation_depletion_allw_amt: Usd,
    /// Line 23: Adjusted basis. Subtract line 22 from line 21
    pub adjusted_basis_amt: Usd,
    /// Line 24: Total gain. Subtract line 23 from line 20
    pub total_gain_amt: Usd,
    /// Line 25a: Depreciation allowed or allowable from line 22 (if section 1245 property)
    pub section1245_depreciation_allw_amt: Usd,
    /// Line 25b: Enter the smaller of line 24 or 25a (section 1245 property amount)
    pub section1245_property_amt: Usd,
    /// Line 26a: Additional depreciation after 1975 (if section 1250 property)
    pub addnl_depreciation_after1975_amt: Usd,
    /// Line 26b: Applicable percentage multiplied by the smaller of line 24 or line 26a
    pub applicable_percentage_amt: Usd,
    /// Line 26c: Subtract line 26a from line 24. If residential rental property or line 24 isn't more than line 26a, skip lines 26d and 26e
    pub gain_less_deprec_after1975_amt: Usd,
    /// Line 26d: Additional depreciation after 1969 and before 1976
    pub addnl_depreciation1969_to1976_amt: Usd,
    /// Line 26e: Enter the smaller of line 26c or 26d
    pub smllr_reduced_gain_addnl_deprec_amt: Usd,
    /// Line 26f: Section 291 amount (corporations only)
    pub section291_amt: Usd,
    /// Line 26g: Add lines 26b, 26e, and 26f (section 1250 property amount)
    pub section1250_property_amt: Usd,
    /// Line 27a: Soil, water, and land clearing expenses (if section 1252 property)
    pub soil_water_land_clear_expense_amt: Usd,
    /// Line 27b: Line 27a multiplied by applicable percentage
    pub applcbl_pct_soil_wtr_clear_expn_amt: Usd,
    /// Line 27c: Enter the smaller of line 24 or 27b (section 1252 property amount)
    pub section1252_property_amt: Usd,
    /// Line 28a: Intangible drilling and development costs (if section 1254 property)
    pub intangible_drilling_dev_cost_amt: Usd,
    /// Line 28b: Enter the smaller of line 24 or 28a (section 1254 property amount)
    pub section1254_property_amt: Usd,
    /// Line 29a: Applicable percentage of payments excluded from income under section 126
    pub applcbl_pct_payment_excluded_amt: Usd,
    /// Line 29b: Enter the smaller of line 24 or 29a (section 1255 property amount)
    pub section1255_property_amt: Usd,

    // -----------------------------------------------------------------------
    // Summary of Part III Gains
    // -----------------------------------------------------------------------
    /// Line 30: Total gains for all properties. Add property columns A through D, line 24
    pub total_gains_for_all_properties_amt: Usd,
    /// Line 31: Add property columns A through D, lines 25b, 26g, 27c, 28b, and 29b. Enter here and on line 13
    pub total_section_property_amt: Usd,
    // -----------------------------------------------------------------------
    // Part IV — Recapture Amounts Under Sections 179 and 280F(b)(2) When
    // Business Use Drops to 50% or Less
    // -----------------------------------------------------------------------
    /// Line 33 (a): Section 179 expense deduction or depreciation allowable in prior years
    pub sect179_ded_depreciation_py_amt: Usd,
    /// Line 33 (b): Section 280F(b)(2) expense deduction or depreciation allowable in prior years
    pub sect280_ded_depreciation_py_amt: Usd,
    /// Line 34 (a): Recomputed depreciation — Section 179
    pub sect179_rcmpt_depreciation_amt: Usd,
    /// Line 34 (b): Recomputed depreciation — Section 280F(b)(2)
    pub sect280_rcmpt_depreciation_amt: Usd,
    /// Line 35 (a): Recapture amount — Section 179. Subtract line 34 from line 33
    pub section179_recapture_amt: Usd,
    /// Line 35 (b): Recapture amount — Section 280F(b)(2). Subtract line 34 from line 33
    pub sect280_recapture_amt: Usd,
}
