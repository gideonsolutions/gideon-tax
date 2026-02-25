use crate::Usd;

/// Output fields for IRS Form 8621 (2025) — Information Return by a Shareholder of a Passive Foreign Investment Company or Qualified Electing Fund.
#[derive(Debug, Clone, Default)]
pub struct Output8621 {
    // -----------------------------------------------------------------------
    // Header — Shareholder Information
    // -----------------------------------------------------------------------
    /// Header: Shareholder name
    pub shareholder_person_nm: String,
    /// Header: Shareholder tax year (calendar year)
    pub shareholder_tax_yr: u16,
    /// Header: Shareholder tax year begin date
    pub shareholder_tax_year_begin_dt: String,
    /// Header: Shareholder tax year end date
    pub shareholder_tax_year_end_dt: String,
    /// Header: Check type — Individual
    pub individual_shareholder_ind: bool,
    /// Header: Check type — Corporation
    pub corporation_shareholder_ind: bool,
    /// Header: Check type — Partnership
    pub partnership_shareholder_ind: bool,
    /// Header: Check type — S Corporation
    pub s_corporation_shareholder_ind: bool,
    /// Header: Check type — Nongrantor Trust
    pub nongrantor_trust_shareholder_ind: bool,
    /// Header: Check type — Estate
    pub estate_shareholder_ind: bool,
    /// Header: Check if any Excepted Specified Foreign Financial Assets are reported
    pub foreign_financial_asset_ind: bool,
    /// Header: Qualifying Insurance Corporation Election
    pub qualified_insurance_corp_elect_ind: bool,

    // -----------------------------------------------------------------------
    // Header — PFIC / QEF Information
    // -----------------------------------------------------------------------
    /// PFIC/QEF: Business name line 1
    pub business_name_line1_txt: String,
    /// PFIC/QEF: Business name line 2
    pub business_name_line2_txt: String,
    /// PFIC/QEF: Address line 1
    pub address_line1_txt: String,
    /// PFIC/QEF: Address line 2
    pub address_line2_txt: String,
    /// PFIC/QEF: City name
    pub city_nm: String,
    /// PFIC/QEF: Province or state name
    pub province_or_state_nm: String,
    /// PFIC/QEF: Country code
    pub country_cd: String,
    /// PFIC/QEF: Foreign postal code
    pub foreign_postal_cd: String,
    /// PFIC/QEF: U.S. address
    pub pfic_or_qefus_address: String,
    /// PFIC/QEF: Employer identification number
    pub pfic_or_qefein: String,
    /// PFIC/QEF: Tax year (calendar year)
    pub tax_yr: u16,
    /// PFIC/QEF: Tax year begin date
    pub tax_year_begin_dt: String,
    /// PFIC/QEF: Tax year end date
    pub tax_year_end_dt: String,

    // -----------------------------------------------------------------------
    // Part I — Summary of Annual Information
    // -----------------------------------------------------------------------
    /// Line 1: Description of each class of shares held by the shareholder
    pub class_of_share_txt: String,
    /// Line 1: Check if shares jointly owned with spouse
    pub jointly_owned_with_spouse_ind: bool,
    /// Line 2: Date shares acquired during the tax year
    pub shares_acquired_dt: String,
    /// Line 3: Number of shares held at the end of the tax year
    pub end_tax_year_shares_cnt: u32,
    /// Line 3: Percentage of shares held at the end of the tax year
    pub end_tax_year_shares_pct: String,
    /// Line 4a: Value of shares $0-50,000
    pub shares_value_range_a_ind: bool,
    /// Line 4b: Value of shares $50,001-100,000
    pub shares_value_range_b_ind: bool,
    /// Line 4c: Value of shares $100,001-150,000
    pub shares_value_range_c_ind: bool,
    /// Line 4d: Value of shares $150,001-200,000
    pub shares_value_range_d_ind: bool,
    /// Line 4e: Value of shares if more than $200,000
    pub shares_value_range_e_amt: Usd,
    /// Line 5a: Section 1291 — excess distribution or gain indicator
    pub section1291_ind: bool,
    /// Line 5a: Section 1291 — amount
    pub section1291_amt: Usd,
    /// Line 5b: Section 1293 (Qualified Electing Fund) indicator
    pub section1293_ind: bool,
    /// Line 5b: Section 1293 — amount
    pub section1293_amt: Usd,
    /// Line 5c: Section 1296 (Mark to Market) indicator
    pub section1296_ind: bool,
    /// Line 5c: Section 1296 — amount
    pub section1296_amt: Usd,

    // -----------------------------------------------------------------------
    // Part II — Elections
    // -----------------------------------------------------------------------
    /// Election A: Election to treat the PFIC as a QEF
    pub election_to_treat_the_pfic_as_qef_ind: bool,
    /// Election B: Election to extend time for payment of tax
    pub elect_to_extnd_tm_for_pymt_of_tx_ind: bool,
    /// Election C: Election to mark-to-market PFIC stock
    pub election_to_mark_to_mrkt_pfic_stk_ind: bool,
    /// Election D: Deemed sale election
    pub deemed_sale_election_ind: bool,
    /// Election E: Deemed dividend election (PFIC first tax year as QEF that is CFC)
    pub deemed_dividend_election_ind: bool,
    /// Election F: Election to recognize gain on deemed sale of PFIC
    pub elect_to_rcgnz_gain_on_pfic_sale_ind: bool,
    /// Election G: Deemed dividend election with respect to a section 1297(e) PFIC
    pub deemed_div_elect_sec1297e_pfic_ind: bool,
    /// Election H: Deemed dividend election with respect to a former PFIC
    pub deemed_div_elect_frmr_pfic_ind: bool,
    /// Election status (combined election status descriptor)
    pub election_status: String,

    // -----------------------------------------------------------------------
    // Part III — Income From a Qualified Electing Fund (QEF)
    // -----------------------------------------------------------------------
    /// Line 6a: Pro rata share of the ordinary earnings of the QEF
    pub pro_rata_share_of_qef_ordnry_earn_amt: Usd,
    /// Line 6b: Portion of line 6a included in income under section 951 or excludable under section 1293(g)
    pub portion_of_pro_rate_ordnry_earn_amt: Usd,
    /// Line 6c: Subtract line 6b from line 6a (ordinary income)
    pub ordinary_income_from_qef_amt: Usd,
    /// Line 7a: Pro rata share of the total net capital gain of the QEF
    pub pro_rata_share_of_tot_net_cap_gain_amt: Usd,
    /// Line 7b: Portion of line 7a included in income under section 951 or excludable under section 1293(g)
    pub income_portion_of_net_cap_gain_amt: Usd,
    /// Line 7c: Subtract line 7b from line 7a (net long-term capital gain)
    pub net_long_term_capital_gain_amt: Usd,
    /// Line 8a: Add lines 6c and 7c (dividend income and net LTCG)
    pub dividend_income_and_net_ltcg_amt: Usd,
    /// Line 8b: Total amount of cash and FMV of other property distributed or deemed distributed
    pub total_cash_and_distributions_amt: Usd,
    /// Line 8c: Portion of line 8a attributable to shares disposed of, pledged, or transferred
    pub total_cash_and_prtn_of_pro_rata_amt: Usd,
    /// Line 8e: Subtract line 8d from line 8a (undistributed earnings)
    pub undistributed_earnings_amt: Usd,
    /// Line 9a: Total tax for the tax year
    pub total_tax_for_tax_year_amt: Usd,
    /// Line 9b: Total tax determined without regard to the amount on line 8e
    pub tot_tx_without_pro_rata_less_cash_amt: Usd,
    /// Line 9c: Deferred tax (subtract line 9b from line 9a)
    pub deferred_tax_amt: Usd,

    // -----------------------------------------------------------------------
    // Part IV — Gain or (Loss) From Mark-to-Market Election
    // -----------------------------------------------------------------------
    /// Line 10a: Fair market value of PFIC stock at end of tax year
    pub fair_market_value_of_pfic_stk_amt: Usd,
    /// Line 10b: Adjusted basis in stock at end of tax year
    pub adjusted_basis_in_stock_end_of_ty_amt: Usd,
    /// Line 10c: Subtract line 10b from line 10a (gain or loss)
    pub ordinary_income_from_pfic_stk_amt: Usd,
    /// Line 11: Unreversed inclusions (as defined in section 1296(d))
    pub unreversed_inclusions_amt: Usd,
    /// Line 12: Loss from line 10c, limited to unreversed inclusions on line 11
    pub income_portion_of_ordinary_earn_amt: Usd,
    /// Line 13a: Fair market value of stock on date of sale or disposition
    pub fmv_stk_on_dt_sale_or_dispos_amt: Usd,
    /// Line 13b: Adjusted basis of stock on date of sale or disposition
    pub adj_basis_stk_on_dt_sale_or_dispos_amt: Usd,
    /// Line 13c: Subtract line 13b from line 13a (gain or loss on disposition)
    pub pro_rata_less_cash_and_portion_amt: Usd,
    /// Line 14a: Unreversed inclusions on disposition (section 1296(d))
    pub stk_sale_unreversed_inclusions_amt: Usd,
    /// Line 14b: Loss from line 13c, limited to unreversed inclusions on line 14a
    pub loss_limited_by_ordinary_income_amt: Usd,
    /// Line 14c: Loss on line 13c exceeding unreversed inclusions on line 14a
    pub loss_excess_of_unrvrsd_inclsn_amt: Usd,

    // -----------------------------------------------------------------------
    // Part V — Distributions From and Dispositions of Stock of a Section 1291 Fund
    // -----------------------------------------------------------------------
    /// Line 15a: Total distributions from the section 1291 fund during the current tax year
    pub distri_and_dispos_of_stock_typ: String,
    /// Line 15e(2): Excess distribution (in U.S. dollars)
    pub excess_amt: Usd,
    /// Line 15f: Gain or loss from disposition of stock of a section 1291 fund
    pub excess_or_unreserved_inclsn_amt: Usd,
    /// Line 16b: Amounts allocable to current tax year and pre-PFIC years (other income)
    pub earnings_distributed_dur_the_ty_amt: Usd,
    /// Line 16c: Aggregate increases in tax for each tax year in holding period
    pub accrued_interest_due_this_ret_amt: Usd,
    /// Line 16e: Subtract line 16d from line 16c ("additional tax")
    pub interest_accrued_on_defrd_tax_amt: Usd,

    // -----------------------------------------------------------------------
    // Part VI — Status of Prior Year Section 1294 Elections and Termination
    // -----------------------------------------------------------------------
    /// Line 17: Tax year of outstanding election
    pub outstanding_election_tax_yr: u16,
    /// Line 18: Undistributed earnings to which the election relates
    pub deferred_tax_after_partial_term_amt: Usd,
    /// Line 21: Event terminating election
    pub event_terminating_election_txt: String,
    /// Line 22: Earnings distributed or deemed distributed during the tax year
    pub deferred_tax_due_with_this_ret_amt: Usd,
    /// Line 24: Accrued interest due with this return
    pub interest_accr_aftr_partl_term_amt: Usd,

    // -----------------------------------------------------------------------
    // Additional / EIN
    // -----------------------------------------------------------------------
    /// EIN missing reason code
    pub ein_missing_reason_cd: String,
}
