use crate::Usd;

/// Output fields for IRS Form 8824 (2025) — Like-Kind Exchanges.
#[derive(Debug, Clone, Default)]
pub struct Output8824 {
    // -----------------------------------------------------------------------
    // Top-of-form — Taxpayer information
    // -----------------------------------------------------------------------
    /// Name shown on tax return
    pub person_nm: String,
    /// Business name line 1
    pub business_name_line1_txt: String,
    /// Business name line 2
    pub business_name_line2_txt: String,
    /// Identifying number (SSN)
    pub ssn: String,
    /// Identifying number (EIN)
    pub ein: String,
    /// Missing EIN reason code
    pub missing_ein_reason_cd: String,

    // -----------------------------------------------------------------------
    // Part I — Information on the Like-Kind Exchange
    // -----------------------------------------------------------------------
    /// Line 1: Description of like-kind property given up
    pub like_kind_property_given_up_dsc: String,
    /// Line 2: Description of like-kind property received
    pub like_kind_property_received_dsc: String,
    /// Line 3: Date like-kind property given up was originally acquired (month, day, year)
    pub property_given_up_acquired_dt: String,
    /// Line 4: Date you actually transferred your property to the other party (month, day, year)
    pub property_transferred_dt: String,
    /// Line 5: Date like-kind property you received was identified by written notice to another party (month, day, year)
    pub written_notice_of_property_rcvd_dt: String,
    /// Line 6: Date you actually received the like-kind property from other party (month, day, year)
    pub property_actually_received_dt: String,
    /// Line 7: Was the exchange of the property given up or received made with a related party?
    pub exchange_made_with_related_prty_ind: bool,

    // -----------------------------------------------------------------------
    // Part II — Related Party Exchange Information
    // -----------------------------------------------------------------------
    /// Line 8: Relationship to you
    pub relationship_description_txt: String,
    /// Line 8: Related party's U.S. address
    pub us_address: String,
    /// Line 8: Related party's foreign address
    pub foreign_address: String,
    /// Line 9: During this tax year, did the related party sell or dispose of any part of the like-kind property received from you?
    pub related_party_sold_prop_rcvd_ind: bool,
    /// Line 10: During this tax year, did you sell or dispose of any part of the like-kind property you received?
    pub you_sold_property_received_ind: bool,
    /// Line 11a: The disposition was after the death of either of the related parties
    pub dispos_was_aftr_dth_rltd_partys_ind: bool,
    /// Line 11b: The disposition was an involuntary conversion, and the threat of conversion occurred after the exchange
    pub disposition_was_invlntry_cnvrt_ind: bool,
    /// Line 11c: You can establish that neither the exchange nor the disposition had tax avoidance as one of its principal purposes
    pub exch_dispos_not_tax_avoidance_ind: bool,

    // -----------------------------------------------------------------------
    // Part III — Realized Gain or (Loss), Recognized Gain, and Basis of Like-Kind Property Received
    // -----------------------------------------------------------------------
    /// Line 12: Fair market value (FMV) of other property given up. See instructions
    pub fmv_of_other_property_given_up_amt: Usd,
    /// Line 12a: Description of other property given up
    pub other_property_given_up_desc: String,
    /// Line 13: Adjusted basis of other property given up
    pub adjusted_basis_of_oth_prop_gvn_up_amt: Usd,
    /// Line 14: Gain or (loss) recognized on other property given up. Subtract line 13 from line 12
    pub gain_loss_on_other_prop_gvn_up_amt: Usd,
    /// Line 15: Cash received, FMV of other property received, plus net liabilities assumed by other party, reduced (but not below zero) by any exchange expenses you incurred
    pub cash_fmv_net_liab_red_by_expnss_amt: Usd,
    /// Line 15a: Description of other property received
    pub other_property_received_desc: String,
    /// Line 16: FMV of like-kind property you received
    pub fmv_of_like_kind_property_rcvd_amt: Usd,
    /// Line 17: Add lines 15 and 16
    pub realized_amt: Usd,
    /// Line 18: Adjusted basis of like-kind property you gave up, net amounts paid to other party, plus any exchange expenses not used on line 15. See instructions
    pub adj_bss_of_like_kind_prop_gvn_up_amt: Usd,
    /// Line 19: Realized gain or (loss). Subtract line 18 from line 17
    pub realized_gain_or_loss_amt: Usd,
    /// Line 20: Enter the smaller of line 15 or line 19, but not less than zero
    pub smaller_gain_or_loss_amt: Usd,
    /// Line 21: Ordinary income under recapture rules. Enter here and on Form 4797, line 16. See instructions
    pub ordinary_incm_und_recapture_rls_amt: Usd,
    /// Line 22: Subtract line 21 from line 20. If zero or less, enter -0-. If more than zero, enter here and on Schedule D or Form 4797
    pub smllr_gain_loss_less_ordnry_incm_amt: Usd,
    /// Line 23: Recognized gain. Add lines 21 and 22
    pub recognized_gain_amt: Usd,
    /// Line 24: Deferred gain or (loss). Subtract line 23 from line 19. If a related party exchange, see instructions
    pub deferred_gain_or_loss_amt: Usd,
    /// Line 25: Basis of like-kind property received. Subtract line 15 from the sum of lines 18 and 23. See instructions
    pub basis_of_like_kind_property_rcvd_amt: Usd,
    /// Line 25a: Basis of like-kind section 1250 property received
    pub bss_like_kind_sect1250_prop_rcvd_amt: Usd,
    /// Line 25b: Basis of like-kind section 1245, 1252, 1254, and 1255 property received
    pub bss_like_kind_sect_prop_rcvd_amt: Usd,
    /// Line 25c: Basis of like-kind intangible property received
    pub bss_like_kind_intngbl_prop_rcvd_amt: Usd,
    /// Gain or (loss) description text
    pub gain_or_loss_desc: String,
    /// Gain or (loss) amount
    pub gain_or_loss_amt: Usd,
    /// Statement indicator for reporting gain in multi-asset exchanges
    pub gain_in_multi_asset_exch_stmt_ind: bool,

    // -----------------------------------------------------------------------
    // Part IV — Deferral of Gain From Section 1043 Conflict-of-Interest Sales
    // -----------------------------------------------------------------------
    /// Line 26: Certificate of divestiture number
    pub deferral_of_gain_cert_of_dvsttr_num: String,
    /// Line 27: Description of divested property
    pub deferral_of_gain_dvstd_property: String,
    /// Line 28: Description of replacement property
    pub deferral_of_gain_desc_of_rplc_prop: String,
    /// Line 29: Date divested property was sold (month, day, year)
    pub deferral_of_gain_dvstd_prop_sold_dt: String,
    /// Line 30: Sales price of divested property. See instructions
    pub deferral_of_gain_dvstd_prop_sale_amt: Usd,
    /// Line 31: Basis of divested property
    pub deferral_of_gain_dvstd_prop_bss_amt: Usd,
    /// Line 32: Realized gain. Subtract line 31 from line 30
    pub deferral_of_gain_realized_gain_amt: Usd,
    /// Line 33: Cost of replacement property purchased within 60 days after date of sale
    pub deferral_of_gain_rplc_cost_aftr_amt: Usd,
    /// Line 34: Subtract line 33 from line 30. If zero or less, enter -0-
    pub deferral_of_gain_rcgnz_less_loss_amt: Usd,
    /// Line 35: Ordinary income under recapture rules. Enter here and on Form 4797, line 10. See instructions
    pub deferral_of_gain_rcgnz_gain_amt: Usd,
    /// Line 37: Deferred gain. Subtract the sum of lines 35 and 36 from line 32
    pub deferral_of_gain_amt: Usd,
    /// Line 38: Basis of replacement property. Subtract line 37 from line 33
    pub deferral_of_gain_bss_of_rplc_prop_amt: Usd,
}
