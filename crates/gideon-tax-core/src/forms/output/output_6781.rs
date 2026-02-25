use crate::Usd;

/// Output fields for IRS Form 6781 (2025) — Gains and Losses From Section 1256 Contracts and Straddles.
#[derive(Debug, Clone, Default)]
pub struct Output6781 {
    // -----------------------------------------------------------------------
    // Top-of-form — Check all applicable boxes
    // -----------------------------------------------------------------------
    /// Box A: Mixed straddle election
    pub mixed_straddle_election_ind: bool,
    /// Box B: Straddle-by-straddle identification election
    pub straddle_by_straddle_ind: bool,
    /// Box C: Mixed straddle account election
    pub mixed_straddle_account_ind: bool,
    /// Box D: Net section 1256 contracts loss election
    pub net_section1256_election_ind: bool,

    // -----------------------------------------------------------------------
    // Part I — Section 1256 Contracts Marked to Market
    // -----------------------------------------------------------------------
    /// Line 1: Identification of account — gains and losses from section 1256 contracts
    /// (reported in columns (b) Loss and (c) Gain on the form)
    pub total_section1256_cntrcts_gain_amt: Usd,
    /// Line 1: Total section 1256 contracts loss amount
    pub total_section1256_cntrcts_loss_amt: Usd,
    /// Line 3: Net gain or (loss). Combine line 2, columns (b) and (c)
    pub net_gain_amt: Usd,
    /// Line 4: Form 1099-B adjustments. See instructions and attach statement
    pub form1099_b_adjustments_amt: Usd,
    /// Line 5: Combine lines 3 and 4
    pub net_gain_and1099_b_adjustments_amt: Usd,
    /// Line 6: If you have a net section 1256 contracts loss and checked box D above, enter the
    /// amount of loss to be carried back. Enter the loss as a positive number
    pub section1256_carried_back_amt: Usd,
    /// Line 7: Combine lines 5 and 6
    pub net_gain_and_adj_plus_carryback_amt: Usd,
    /// Line 8: Short-term capital gain or (loss). Multiply line 7 by 40% (0.40). Enter here and
    /// include on line 4 of Schedule D or on Form 8949. See instructions
    pub short_term_capital_gain_amt: Usd,
    /// Line 9: Long-term capital gain or (loss). Multiply line 7 by 60% (0.60). Enter here and
    /// include on line 11 of Schedule D or on Form 8949. See instructions
    pub long_term_capital_gain_amt: Usd,

    // -----------------------------------------------------------------------
    // Part II — Gains and Losses From Straddles
    // -----------------------------------------------------------------------

    // -- Section A — Losses From Straddles --
    /// Line 11a: Enter the short-term portion of losses from line 10, column (h), here and
    /// include on line 4 of Schedule D or on Form 8949. See instructions
    pub short_term_portion_rcgnz_loss_amt: Usd,
    /// Line 11b: Enter the long-term portion of losses from line 10, column (h), here and include
    /// on line 11 of Schedule D or on Form 8949. See instructions
    pub long_term_portion_rcgnz_loss_amt: Usd,

    // -- Section B — Gains From Straddles --
    /// Line 13a: Enter the short-term portion of gains from line 12, column (f), here and include
    /// on line 4 of Schedule D or on Form 8949. See instructions
    pub short_term_portion_of_gain_amt: Usd,
    /// Line 13b: Enter the long-term portion of gains from line 12, column (f), here and include
    /// on line 11 of Schedule D or on Form 8949. See instructions
    pub long_term_portion_of_gain_amt: Usd,
}
