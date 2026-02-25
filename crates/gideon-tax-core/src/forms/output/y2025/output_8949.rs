/// Output fields for IRS Form 8949 (2025) — Sales and Other Dispositions of Capital Assets.
///
/// Note: Form 8949 is a tabular form where individual transactions are listed in Parts I and II.
/// The struct is intentionally empty because transaction-level detail is handled elsewhere;
/// totals flow directly to Schedule D.
#[derive(Debug, Clone, Default)]
pub struct Output8949 {
    // -----------------------------------------------------------------------
    // Part I — Short-Term (transactions involving capital assets held 1 year or less)
    // -----------------------------------------------------------------------
    // Line 1: Individual short-term transactions (columns a-h)
    // Line 2: Totals for columns (d), (e), (g), and (h)

    // -----------------------------------------------------------------------
    // Part II — Long-Term (transactions involving capital assets held more than 1 year)
    // -----------------------------------------------------------------------
    // Line 1: Individual long-term transactions (columns a-h)
    // Line 2: Totals for columns (d), (e), (g), and (h)
}
