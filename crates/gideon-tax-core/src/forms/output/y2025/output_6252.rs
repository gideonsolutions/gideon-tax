use crate::Usd;

/// Output fields for IRS Form 6252 (2025) — Installment Sale Income.
#[derive(Debug, Clone, Default)]
pub struct Output6252 {
    // -----------------------------------------------------------------------
    // Top-of-form
    // -----------------------------------------------------------------------
    /// Line 1: Description of property
    pub property_desc: String,
    /// Line 2a: Date acquired (mm/dd/yyyy)
    pub acquired_dt: String,
    /// Line 2b: Date sold (mm/dd/yyyy)
    pub sold_dt: String,
    /// Line 2b (alt): Disposition date
    pub disposition_dt: String,
    /// Line 3: Was the property sold to a related party? (Yes/No)
    pub property_sold_to_related_party_ind: bool,
    /// Line 4: Can the total selling price be determined by the close of the tax year in which
    /// such sale or other disposition occurs? (Yes/No)
    pub tot_sell_prc_ty_sale_or_oth_dispos_ind: bool,
    /// Installment sale property type code
    pub installment_sale_property_type_cd: String,

    // -----------------------------------------------------------------------
    // Part I — Gross Profit and Contract Price
    // -----------------------------------------------------------------------
    /// Line 5: Selling price including mortgages and other debts. Don't include interest
    pub selling_price_including_mortg_amt: Usd,
    /// Line 6: Mortgages, debts, and other liabilities the buyer assumed or took the property
    /// subject to (see instructions)
    pub mortgage_indebtedness_amt: Usd,
    /// Line 7: Subtract line 6 from line 5
    pub selling_price_less_mortg_indbt_amt: Usd,
    /// Line 8: Cost or other basis of property sold
    pub cost_or_other_basis_prop_sold_amt: Usd,
    /// Line 9: Depreciation allowed or allowable
    pub depreciation_allowed_amt: Usd,
    /// Line 10: Adjusted basis. Subtract line 9 from line 8
    pub adjusted_basis_amt: Usd,
    /// Line 11: Commissions and other expenses of sale
    pub commissions_other_expns_of_sale_amt: Usd,
    /// Line 12: Income recapture from Form 4797, Part III (see instructions)
    pub ordinary_incm_und_recapture_rls_amt: Usd,
    /// Line 13: Add lines 10, 11, and 12
    pub sum_of_adj_bss_comm_incm_rcptr_amt: Usd,
    /// Line 14: Subtract line 13 from line 5. If zero or less, don't complete the rest of this
    /// form. See instructions
    pub sum_less_adj_bss_comm_incm_rcptr_amt: Usd,
    /// Line 15: If the property described on line 1 above was your main home, enter the amount
    /// of your excluded gain. See instructions. Otherwise, enter -0-
    pub excluded_gain_amt: Usd,
    /// Line 16: Gross profit. Subtract line 15 from line 14
    pub gross_profit_amt: Usd,
    /// Line 17: Subtract line 13 from line 6. If zero or less, enter -0-
    pub net_adj_basis_comm_incm_rcptr_amt: Usd,
    /// Line 18: Contract price. Add line 7 and line 17
    pub contract_price_amt: Usd,

    // -----------------------------------------------------------------------
    // Part II — Installment Sale Income
    // -----------------------------------------------------------------------
    /// Line 19: Gross profit percentage (expressed as a decimal amount). Divide line 16 by line 18
    pub gross_profit_ratio_pct: String,
    /// Line 20: If this is the year of sale, enter the amount from line 17. Otherwise, enter -0-
    pub year_of_sale_amt: Usd,
    /// Line 21: Payments received during year (see instructions). Don't include interest
    pub payments_received_current_year_amt: Usd,
    /// Line 22: Add lines 20 and 21
    pub sum_year_of_sale_and_pymts_rcvd_amt: Usd,
    /// Line 23: Payments received in prior years (see instructions). Don't include interest
    pub payments_received_prior_years_amt: Usd,
    /// Line 24: Installment sale income. Multiply line 22 by line 19. This amount cannot be less
    /// than zero. See instructions
    pub installment_sale_income_amt: Usd,
    /// Line 25: Enter the part of line 24 that is ordinary income under the recapture rules.
    /// See instructions
    pub ordinary_income_part_amt: Usd,
    /// Line 26: Subtract line 25 from line 24. Enter here and on Schedule D or Form 4797.
    /// See instructions
    pub instal_sale_less_ordnry_incm_amt: Usd,

    // -----------------------------------------------------------------------
    // Part III — Related Party Installment Sale Income
    // -----------------------------------------------------------------------
    /// Line 27: Name, address, and taxpayer identifying number of related party
    pub related_party_ssn: String,
    /// Related party EIN
    pub related_party_ein: String,
    /// Related party missing EIN reason code
    pub missing_ein_reason_cd: String,
    /// Related party business name line 1
    pub business_name_line1_txt: String,
    /// Related party business name line 2
    pub business_name_line2_txt: String,
    /// Related party US address
    pub related_party_us_address: String,
    /// Related party foreign address
    pub related_party_foreign_address: String,
    /// Line 28: Did the related party resell or dispose of the property ("second disposition")
    /// during this tax year? (Yes/No)
    pub second_disposition_ind: bool,
    /// Line 29: Check the box that applies — not to avoid tax
    pub not_to_avoid_tax_ind: bool,
    /// Line 29a: The second disposition was more than 2 years after the first disposition
    /// (other than dispositions of marketable securities)
    pub snd_dispos_more2_yrs_aftr_first_ind: bool,
    /// Line 29b: The first disposition was a sale or exchange of stock to the issuing corporation
    pub first_dispos_sale_exchange_stk_ind: bool,
    /// Line 29c: The second disposition was an involuntary conversion and the threat of conversion
    /// occurred after the first disposition
    pub second_dispos_invlntry_cnvrt_ind: bool,
    /// Line 29d: The second disposition occurred after the death of the original seller or buyer
    pub second_dispos_after_death_sellr_ind: bool,
    /// Line 29e: Property sold to related party with marketable securities indicator
    pub prop_sold_rltd_party_mrktbl_sec_ind: bool,
    /// Line 30: Selling price of property sold by related party (see instructions)
    pub realized_amt: Usd,
    /// Line 31: Enter contract price from line 18 for year of first sale
    pub first_year_contract_price_amt: Usd,
    /// Line 32: Enter the smaller of line 30 or line 31
    pub smllr_realized_or_contract_prc_amt: Usd,
    /// Line 33: Total payments received by the end of this tax year (see instructions)
    pub total_payments_received_amt: Usd,
    /// Line 34: Subtract line 33 from line 32. If zero or less, enter -0-
    pub total_payments_rcvd_less_prc_amt: Usd,
    /// Line 35: Multiply line 34 by the gross profit percentage on line 19 for year of first sale
    pub tot_pymt_prc_times_gro_prft_pct_amt: Usd,
    /// Line 36: Enter the part of line 35 that is ordinary income under the recapture rules.
    /// See instructions
    pub payment_price_less_ordnry_incm_amt: Usd,
    /// Line 37: Subtract line 36 from line 35. Enter here and on Schedule D or Form 4797.
    /// See instructions
    pub total_section_property_amt: Usd,
}
