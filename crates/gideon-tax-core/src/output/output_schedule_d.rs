use crate::Usd;

/// Output fields for IRS Schedule D (Form 1040) — Capital Gains and Losses (2025).
///
/// Fields are ordered by line number as they appear on the form.
/// Covers Part I (Short-Term Capital Gains and Losses), Part II
/// (Long-Term Capital Gains and Losses), and Part III (Summary).
/// Lines 1a--3 and 8a--10 each have sub-columns (d) proceeds,
/// (e) cost, (g) adjustments, and (h) gain or loss, represented
/// with `st_` and `lt_` prefixes for the short-term and long-term
/// groups respectively.
#[derive(Debug, Clone, Default)]
pub struct OutputScheduleD {
    // -----------------------------------------------------------------------
    // Header
    // -----------------------------------------------------------------------
    /// Did you dispose of any investment(s) in a qualified opportunity fund during the tax year?
    pub dispose_investment_qof_ind: bool,

    // -----------------------------------------------------------------------
    // Part I -- Short-Term Capital Gains and Losses
    // -----------------------------------------------------------------------

    // Line 1a: Totals for all short-term transactions reported on Form
    // 1099-B or Form 1099-DA for which basis was reported to the IRS
    // and for which you have no adjustments (see instructions).
    // However, if you choose to report all these transactions on
    // Form 8949, leave this line blank and go to line 1b
    /// Line 1a, column (d): Proceeds (sales price) -- short-term, 1099-B basis reported, no adjustments
    pub st_1099b_bss_rpt_no_adj_proceeds_amt: Usd,
    /// Line 1a, column (e): Cost (or other basis) -- short-term, 1099-B basis reported, no adjustments
    pub st_1099b_bss_rpt_no_adj_cost_amt: Usd,
    /// Line 1a, column (h): Gain or (loss) -- short-term, 1099-B basis reported, no adjustments
    pub st_1099b_bss_rpt_no_adj_gain_or_loss_amt: Usd,

    // Line 1b: Totals for all transactions reported on Form(s) 8949
    // with Box A or Box G checked
    /// Line 1b, column (d): Proceeds (sales price) -- short-term, Form 8949 Box A or Box G
    pub st_1099b_shows_basis_proceeds_amt: Usd,
    /// Line 1b, column (e): Cost (or other basis) -- short-term, Form 8949 Box A or Box G
    pub st_1099b_shows_basis_cost_amt: Usd,
    /// Line 1b, column (g): Adjustments to gain or loss -- short-term, Form 8949 Box A or Box G
    pub st_1099b_shows_basis_adjustments_amt: Usd,
    /// Line 1b, column (h): Gain or (loss) -- short-term, Form 8949 Box A or Box G
    pub st_1099b_shows_basis_gain_or_loss_amt: Usd,

    // Line 2: Totals for all transactions reported on Form(s) 8949
    // with Box B or Box H checked
    /// Line 2, column (d): Proceeds (sales price) -- short-term, Form 8949 Box B or Box H
    pub st_1099b_not_show_basis_proceeds_amt: Usd,
    /// Line 2, column (e): Cost (or other basis) -- short-term, Form 8949 Box B or Box H
    pub st_1099b_not_show_basis_cost_amt: Usd,
    /// Line 2, column (g): Adjustments to gain or loss -- short-term, Form 8949 Box B or Box H
    pub st_1099b_not_show_basis_adjustments_amt: Usd,
    /// Line 2, column (h): Gain or (loss) -- short-term, Form 8949 Box B or Box H
    pub st_1099b_not_show_basis_gain_or_loss_amt: Usd,

    // Line 3: Totals for all transactions reported on Form(s) 8949
    // with Box C or Box I checked
    /// Line 3, column (d): Proceeds (sales price) -- short-term, Form 8949 Box C or Box I
    pub st_1099b_not_received_proceeds_amt: Usd,
    /// Line 3, column (e): Cost (or other basis) -- short-term, Form 8949 Box C or Box I
    pub st_1099b_not_received_cost_amt: Usd,
    /// Line 3, column (g): Adjustments to gain or loss -- short-term, Form 8949 Box C or Box I
    pub st_1099b_not_received_adjustments_amt: Usd,
    /// Line 3, column (h): Gain or (loss) -- short-term, Form 8949 Box C or Box I
    pub st_1099b_not_received_gain_or_loss_amt: Usd,

    /// Line 4: Short-term gain from Form 6252 and short-term gain or (loss) from Forms 4684, 6781, and 8824
    pub st_gain_or_loss_from_forms_amt: Usd,
    /// Line 5: Net short-term gain or (loss) from partnerships, S corporations, estates, and trusts from Schedule(s) K-1
    pub st_net_gain_or_loss_from_sch_k1_amt: Usd,
    /// Line 6: Short-term capital loss carryover. Enter the amount, if any, from line 8 of your Capital Loss Carryover Worksheet in the instructions
    pub st_capital_loss_carryover_amt: Usd,
    /// Line 7: Net short-term capital gain or (loss). Combine lines 1a through 6 in column (h). If you have any long-term capital gains or losses, go to Part II below. Otherwise, go to Part III on the back
    pub net_st_capital_gain_or_loss_amt: Usd,

    // -----------------------------------------------------------------------
    // Part II -- Long-Term Capital Gains and Losses
    // -----------------------------------------------------------------------

    // Line 8a: Totals for all long-term transactions reported on Form
    // 1099-B or Form 1099-DA for which basis was reported to the IRS
    // and for which you have no adjustments (see instructions).
    // However, if you choose to report all these transactions on
    // Form 8949, leave this line blank and go to line 8b
    /// Line 8a, column (d): Proceeds (sales price) -- long-term, 1099-B basis reported, no adjustments
    pub lt_1099b_bss_rpt_no_adj_proceeds_amt: Usd,
    /// Line 8a, column (e): Cost (or other basis) -- long-term, 1099-B basis reported, no adjustments
    pub lt_1099b_bss_rpt_no_adj_cost_amt: Usd,
    /// Line 8a, column (h): Gain or (loss) -- long-term, 1099-B basis reported, no adjustments
    pub lt_1099b_bss_rpt_no_adj_gain_or_loss_amt: Usd,

    // Line 8b: Totals for all transactions reported on Form(s) 8949
    // with Box D or Box J checked
    /// Line 8b, column (d): Proceeds (sales price) -- long-term, Form 8949 Box D or Box J
    pub lt_1099b_shows_basis_proceeds_amt: Usd,
    /// Line 8b, column (e): Cost (or other basis) -- long-term, Form 8949 Box D or Box J
    pub lt_1099b_shows_basis_cost_amt: Usd,
    /// Line 8b, column (g): Adjustments to gain or loss -- long-term, Form 8949 Box D or Box J
    pub lt_1099b_shows_basis_adjustments_amt: Usd,
    /// Line 8b, column (h): Gain or (loss) -- long-term, Form 8949 Box D or Box J
    pub lt_1099b_shows_basis_gain_or_loss_amt: Usd,

    // Line 9: Totals for all transactions reported on Form(s) 8949
    // with Box E or Box K checked
    /// Line 9, column (d): Proceeds (sales price) -- long-term, Form 8949 Box E or Box K
    pub lt_1099b_not_show_basis_proceeds_amt: Usd,
    /// Line 9, column (e): Cost (or other basis) -- long-term, Form 8949 Box E or Box K
    pub lt_1099b_not_show_basis_cost_amt: Usd,
    /// Line 9, column (g): Adjustments to gain or loss -- long-term, Form 8949 Box E or Box K
    pub lt_1099b_not_show_basis_adjustments_amt: Usd,
    /// Line 9, column (h): Gain or (loss) -- long-term, Form 8949 Box E or Box K
    pub lt_1099b_not_show_basis_gain_or_loss_amt: Usd,

    // Line 10: Totals for all transactions reported on Form(s) 8949
    // with Box F or Box L checked
    /// Line 10, column (d): Proceeds (sales price) -- long-term, Form 8949 Box F or Box L
    pub lt_1099b_not_received_proceeds_amt: Usd,
    /// Line 10, column (e): Cost (or other basis) -- long-term, Form 8949 Box F or Box L
    pub lt_1099b_not_received_cost_amt: Usd,
    /// Line 10, column (g): Adjustments to gain or loss -- long-term, Form 8949 Box F or Box L
    pub lt_1099b_not_received_adjustments_amt: Usd,
    /// Line 10, column (h): Gain or (loss) -- long-term, Form 8949 Box F or Box L
    pub lt_1099b_not_received_gain_or_loss_amt: Usd,

    /// Line 11: Gain from Form 4797, Part I; long-term gain from Forms 2439 and 6252; and long-term gain or (loss) from Forms 4684, 6781, and 8824
    pub lt_gain_or_loss_from_forms_amt: Usd,
    /// Line 12: Net long-term gain or (loss) from partnerships, S corporations, estates, and trusts from Schedule(s) K-1
    pub lt_net_gain_or_loss_from_sch_k1_amt: Usd,
    /// Line 13: Capital gain distributions. See the instructions
    pub capital_gain_distributions_amt: Usd,
    /// Line 13: Form 8814 amount
    pub form_8814_amt: Usd,
    /// Line 13: Form 8814 literal code
    pub form_8814_literal_cd: String,
    /// Line 14: Long-term capital loss carryover. Enter the amount, if any, from line 13 of your Capital Loss Carryover Worksheet in the instructions
    pub lt_capital_loss_carryover_amt: Usd,
    /// Line 15: Net long-term capital gain or (loss). Combine lines 8a through 14 in column (h). Then, go to Part III on the back
    pub net_lt_capital_gain_or_loss_amt: Usd,

    // -----------------------------------------------------------------------
    // Part III -- Summary
    // -----------------------------------------------------------------------
    /// Line 16: Combine lines 7 and 15 and enter the result
    pub net_st_and_lt_capital_gain_or_loss_amt: Usd,
    /// Line 17: Are lines 15 and 16 both gains?
    pub st_and_lt_gain_ind: bool,
    /// Line 18: If you are required to complete the 28% Rate Gain Worksheet (see instructions), enter the amount, if any, from line 7 of that worksheet
    pub rate_gain_wksht_amt: Usd,
    /// Line 19: If you are required to complete the Unrecaptured Section 1250 Gain Worksheet (see instructions), enter the amount, if any, from line 18 of that worksheet
    pub unrcptr_sect_1250_gain_wksht_amt: Usd,
    /// Line 20: Are lines 18 and 19 both zero or blank and you are not filing Form 4952?
    pub zero_or_blank_ind: bool,
    /// Line 21: If line 16 is a loss, enter here and on Form 1040, 1040-SR, or 1040-NR, line 7a, the smaller of: the loss on line 16; or ($3,000), or if married filing separately, ($1,500)
    pub allowable_loss_amt: Usd,
    /// Line 22: Do you have qualified dividends on Form 1040, 1040-SR, or 1040-NR, line 3a?
    pub qualified_dividends_ind: bool,
}
