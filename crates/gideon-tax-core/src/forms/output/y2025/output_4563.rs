use us_tax_brackets::TaxYear;

use crate::forms::{DynForm, Form, FormType, OutputForm};
use crate::{GideonTaxError, Usd};

// =========================================================================
// Line 4b occupant
// =========================================================================

/// An occupant of a home maintained outside American Samoa (Form 4563, Line 4b).
#[derive(Debug, Clone)]
pub struct F4563Occupant {
    /// Name of occupant
    pub occupant_nm: String,
    /// Relationship of occupant to filer
    pub occupant_relationship: String,
}

// =========================================================================
// Line 4b row
// =========================================================================

/// Per-entry row from Form 4563, Line 4b.
///
/// Each row describes a home maintained outside American Samoa during the
/// tax year: its address, whether it was rented, and the occupants.
#[derive(Debug, Clone)]
pub struct F4563Line4b {
    /// Address of home maintained outside American Samoa
    pub address: String,
    /// Whether the home was rented
    pub rented_ind: bool,
    /// Occupants of the home
    pub occupants: Vec<F4563Occupant>,
}

// =========================================================================
// Line 6 row
// =========================================================================

/// Per-entry row from Form 4563, Line 6 (columns a–d).
///
/// Each row records one period of absence from American Samoa during the
/// tax year.
#[derive(Debug, Clone)]
pub struct F4563Line6 {
    /// Column (a): Date left American Samoa
    pub date_left: String,
    /// Column (b): Date returned to American Samoa
    pub date_returned: String,
    /// Column (c): Number of days absent from American Samoa
    pub days_absent: u32,
    /// Column (d): Reason for absence
    pub reason_for_absence: String,
}

// =========================================================================
// Input
// =========================================================================

/// All information needed to complete Form 4563.
///
/// Lines 7–14 are income exclusion amounts supplied by the filer;
/// Line 15 (the total) is computed in [`OutputForm::try_new`].
#[derive(Debug, Clone)]
pub struct F4563Input {
    // -----------------------------------------------------------------------
    // Top-of-form
    // -----------------------------------------------------------------------
    /// Name as shown on Form 1040 or 1040-SR
    pub name_line1_txt: String,
    /// Your social security number
    pub ssn: String,

    // -----------------------------------------------------------------------
    // Part I — General Information
    // -----------------------------------------------------------------------
    /// Line 1: Date bona fide residence began
    pub bona_fide_residence_begin_dt: String,
    /// Line 1: Date bona fide residence ended
    pub bona_fide_residence_end_dt: String,
    /// Line 2: Type of living quarters — Rented room
    pub rented_room_ind: bool,
    /// Line 2: Type of living quarters — Rented house or apartment
    pub rented_house_ind: bool,
    /// Line 2: Type of living quarters — Quarters furnished by employer
    pub employer_furnished_quarters_ind: bool,
    /// Line 2: Type of living quarters — Purchased home
    pub purchased_house_ind: bool,
    /// Line 3a: Did any of your family live with you in American Samoa during any part of the tax year?
    pub family_living_with_you_ind: bool,
    /// Line 3b: If "Yes," who and for what period?
    pub family_who_and_period_txt: String,
    /// Line 4a: Did you maintain any home(s) outside American Samoa?
    pub hm_maint_outsd_american_samoa_ind: bool,
    /// Line 4b: If "Yes," details about homes maintained outside American Samoa
    pub homes_outside_american_samoa: Vec<F4563Line4b>,
    /// Line 5: Name and address of employer (state if self-employed)
    pub employer_name_and_address: String,
    /// Line 6: Days absent from American Samoa during the tax year
    pub days_absent: Vec<F4563Line6>,

    // -----------------------------------------------------------------------
    // Eligibility
    // -----------------------------------------------------------------------
    /// Whether the filer is a bona fide resident of American Samoa
    pub is_bona_fide_resident_of_american_samoa: bool,

    // -----------------------------------------------------------------------
    // Part II — Figure Your Exclusion (lines 7-14)
    // -----------------------------------------------------------------------
    /// Line 7: Wages, salaries, tips, etc.
    pub wages_exclusion_amt: Usd,
    /// Line 8: Taxable interest
    pub taxable_interest_exclusion_amt: Usd,
    /// Line 9: Ordinary dividends
    pub ordinary_dividends_exclusion_amt: Usd,
    /// Line 10: Business income
    pub business_income_exclusion_amt: Usd,
    /// Line 11: Capital gain
    pub capital_gain_exclusion_amt: Usd,
    /// Line 12: Rental real estate, royalties, etc.
    pub rental_real_estate_income_excl_amt: Usd,
    /// Line 13: Farm income
    pub farm_income_exclusion_amt: Usd,
    /// Line 14: Other income. List type and amount
    pub total_other_income_exclusion_amt: Usd,
}

// =========================================================================
// Output
// =========================================================================

/// Output fields for IRS Form 4563 (2025) — Exclusion of Income for Bona Fide Residents of American Samoa.
#[derive(Debug, Clone)]
pub struct Output4563 {
    // -----------------------------------------------------------------------
    // Top-of-form
    // -----------------------------------------------------------------------
    /// Name as shown on Form 1040 or 1040-SR
    pub name_line1_txt: String,
    /// Your social security number
    pub ssn: String,

    // -----------------------------------------------------------------------
    // Part I — General Information
    // -----------------------------------------------------------------------
    /// Line 1: Date bona fide residence began
    pub bona_fide_residence_begin_dt: String,
    /// Line 1: Date bona fide residence ended
    pub bona_fide_residence_end_dt: String,
    /// Line 2: Type of living quarters — Rented room
    pub rented_room_ind: bool,
    /// Line 2: Type of living quarters — Rented house or apartment
    pub rented_house_ind: bool,
    /// Line 2: Type of living quarters — Quarters furnished by employer
    pub employer_furnished_quarters_ind: bool,
    /// Line 2: Type of living quarters — Purchased home
    pub purchased_house_ind: bool,
    /// Line 3a: Did any of your family live with you in American Samoa during any part of the tax year?
    pub family_living_with_you_ind: bool,
    /// Line 3b: If "Yes," who and for what period?
    pub family_who_and_period_txt: String,
    /// Line 4a: Did you maintain any home(s) outside American Samoa?
    pub hm_maint_outsd_american_samoa_ind: bool,
    /// Line 4b: If "Yes," details about homes maintained outside American Samoa
    pub homes_outside_american_samoa: Vec<F4563Line4b>,
    /// Line 5: Name and address of employer (state if self-employed)
    pub employer_name_and_address: String,
    /// Line 6: Days absent from American Samoa during the tax year
    pub days_absent: Vec<F4563Line6>,

    // -----------------------------------------------------------------------
    // Part II — Figure Your Exclusion
    // -----------------------------------------------------------------------
    /// Line 7: Wages, salaries, tips, etc.
    pub wages_exclusion_amt: Usd,
    /// Line 8: Taxable interest
    pub taxable_interest_exclusion_amt: Usd,
    /// Line 9: Ordinary dividends
    pub ordinary_dividends_exclusion_amt: Usd,
    /// Line 10: Business income
    pub business_income_exclusion_amt: Usd,
    /// Line 11: Capital gain
    pub capital_gain_exclusion_amt: Usd,
    /// Line 12: Rental real estate, royalties, etc.
    pub rental_real_estate_income_excl_amt: Usd,
    /// Line 13: Farm income
    pub farm_income_exclusion_amt: Usd,
    /// Line 14: Other income. List type and amount
    pub total_other_income_exclusion_amt: Usd,
    /// Line 15: Add lines 7 through 14. This is the amount you may exclude from your gross income this tax year
    pub gross_income_exclusion_amt: Usd,
}

// =========================================================================
// Trait impls
// =========================================================================

impl Form for Output4563 {
    fn name() -> &'static str {
        "Form 4563"
    }

    fn year(&self) -> TaxYear {
        TaxYear::Y2025
    }

    fn form_type() -> FormType {
        FormType::Output
    }
}

impl OutputForm for Output4563 {
    type Input = F4563Input;

    fn must_file(input: &Self::Input) -> bool {
        let amounts = [
            input.wages_exclusion_amt,
            input.taxable_interest_exclusion_amt,
            input.ordinary_dividends_exclusion_amt,
            input.business_income_exclusion_amt,
            input.capital_gain_exclusion_amt,
            input.rental_real_estate_income_excl_amt,
            input.farm_income_exclusion_amt,
            input.total_other_income_exclusion_amt,
        ];
        input.is_bona_fide_resident_of_american_samoa && amounts.iter().any(|&a| a > Usd::ZERO)
    }

    fn try_new(input: Self::Input) -> Result<Self, GideonTaxError> {
        // Line 15: sum of lines 7–14
        let line15 = input.wages_exclusion_amt
            + input.taxable_interest_exclusion_amt
            + input.ordinary_dividends_exclusion_amt
            + input.business_income_exclusion_amt
            + input.capital_gain_exclusion_amt
            + input.rental_real_estate_income_excl_amt
            + input.farm_income_exclusion_amt
            + input.total_other_income_exclusion_amt;

        Ok(Output4563 {
            // Top-of-form
            name_line1_txt: input.name_line1_txt,
            ssn: input.ssn,

            // Part I
            bona_fide_residence_begin_dt: input.bona_fide_residence_begin_dt,
            bona_fide_residence_end_dt: input.bona_fide_residence_end_dt,
            rented_room_ind: input.rented_room_ind,
            rented_house_ind: input.rented_house_ind,
            employer_furnished_quarters_ind: input.employer_furnished_quarters_ind,
            purchased_house_ind: input.purchased_house_ind,
            family_living_with_you_ind: input.family_living_with_you_ind,
            family_who_and_period_txt: input.family_who_and_period_txt,
            hm_maint_outsd_american_samoa_ind: input.hm_maint_outsd_american_samoa_ind,
            homes_outside_american_samoa: input.homes_outside_american_samoa,
            employer_name_and_address: input.employer_name_and_address,
            days_absent: input.days_absent,

            // Part II
            wages_exclusion_amt: input.wages_exclusion_amt,
            taxable_interest_exclusion_amt: input.taxable_interest_exclusion_amt,
            ordinary_dividends_exclusion_amt: input.ordinary_dividends_exclusion_amt,
            business_income_exclusion_amt: input.business_income_exclusion_amt,
            capital_gain_exclusion_amt: input.capital_gain_exclusion_amt,
            rental_real_estate_income_excl_amt: input.rental_real_estate_income_excl_amt,
            farm_income_exclusion_amt: input.farm_income_exclusion_amt,
            total_other_income_exclusion_amt: input.total_other_income_exclusion_amt,
            gross_income_exclusion_amt: line15,
        })
    }

    fn dependencies() -> &'static [DynForm] {
        &[]
    }

    fn is_valid(&self) -> bool {
        let line15 = self.wages_exclusion_amt
            + self.taxable_interest_exclusion_amt
            + self.ordinary_dividends_exclusion_amt
            + self.business_income_exclusion_amt
            + self.capital_gain_exclusion_amt
            + self.rental_real_estate_income_excl_amt
            + self.farm_income_exclusion_amt
            + self.total_other_income_exclusion_amt;

        self.gross_income_exclusion_amt == line15
    }
}

// =========================================================================
// Tests
// =========================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn basic_input() -> F4563Input {
        F4563Input {
            name_line1_txt: "John Doe".to_string(),
            ssn: "123-45-6789".to_string(),
            bona_fide_residence_begin_dt: "01/15/2020".to_string(),
            bona_fide_residence_end_dt: "12/31/2025".to_string(),
            rented_room_ind: false,
            rented_house_ind: true,
            employer_furnished_quarters_ind: false,
            purchased_house_ind: false,
            family_living_with_you_ind: true,
            family_who_and_period_txt: "Spouse, entire year".to_string(),
            hm_maint_outsd_american_samoa_ind: false,
            homes_outside_american_samoa: vec![],
            employer_name_and_address: "Samoa Corp, 100 Main St, Pago Pago, AS 96799".to_string(),
            days_absent: vec![],
            is_bona_fide_resident_of_american_samoa: true,
            wages_exclusion_amt: Usd::from_dollars(50_000),
            taxable_interest_exclusion_amt: Usd::from_dollars(1_000),
            ordinary_dividends_exclusion_amt: Usd::from_dollars(500),
            business_income_exclusion_amt: Usd::ZERO,
            capital_gain_exclusion_amt: Usd::ZERO,
            rental_real_estate_income_excl_amt: Usd::ZERO,
            farm_income_exclusion_amt: Usd::ZERO,
            total_other_income_exclusion_amt: Usd::ZERO,
        }
    }

    #[test]
    fn must_file_with_wages() {
        let input = basic_input();
        assert!(Output4563::must_file(&input));
    }

    #[test]
    fn must_file_no_income() {
        let mut input = basic_input();
        input.wages_exclusion_amt = Usd::ZERO;
        input.taxable_interest_exclusion_amt = Usd::ZERO;
        input.ordinary_dividends_exclusion_amt = Usd::ZERO;
        assert!(!Output4563::must_file(&input));
    }

    #[test]
    fn must_file_only_other_income() {
        let mut input = basic_input();
        input.wages_exclusion_amt = Usd::ZERO;
        input.taxable_interest_exclusion_amt = Usd::ZERO;
        input.ordinary_dividends_exclusion_amt = Usd::ZERO;
        input.total_other_income_exclusion_amt = Usd::from_dollars(200);
        assert!(Output4563::must_file(&input));
    }

    #[test]
    fn must_file_not_bona_fide_resident() {
        let mut input = basic_input();
        input.is_bona_fide_resident_of_american_samoa = false;
        assert!(!Output4563::must_file(&input));
    }

    #[test]
    fn line15_sums_lines_7_through_14() {
        let form = Output4563::try_new(basic_input()).unwrap();
        // 50,000 + 1,000 + 500 = 51,500
        assert_eq!(form.gross_income_exclusion_amt, Usd::from_dollars(51_500));
        assert!(form.is_valid());
    }

    #[test]
    fn all_income_types() {
        let mut input = basic_input();
        input.wages_exclusion_amt = Usd::from_dollars(40_000);
        input.taxable_interest_exclusion_amt = Usd::from_dollars(2_000);
        input.ordinary_dividends_exclusion_amt = Usd::from_dollars(1_500);
        input.business_income_exclusion_amt = Usd::from_dollars(10_000);
        input.capital_gain_exclusion_amt = Usd::from_dollars(3_000);
        input.rental_real_estate_income_excl_amt = Usd::from_dollars(5_000);
        input.farm_income_exclusion_amt = Usd::from_dollars(8_000);
        input.total_other_income_exclusion_amt = Usd::from_dollars(500);
        let form = Output4563::try_new(input).unwrap();
        // 40,000 + 2,000 + 1,500 + 10,000 + 3,000 + 5,000 + 8,000 + 500 = 70,000
        assert_eq!(form.gross_income_exclusion_amt, Usd::from_dollars(70_000));
        assert!(form.is_valid());
    }

    #[test]
    fn zero_income_line15_is_zero() {
        let mut input = basic_input();
        input.wages_exclusion_amt = Usd::ZERO;
        input.taxable_interest_exclusion_amt = Usd::ZERO;
        input.ordinary_dividends_exclusion_amt = Usd::ZERO;
        let form = Output4563::try_new(input).unwrap();
        assert_eq!(form.gross_income_exclusion_amt, Usd::ZERO);
        assert!(form.is_valid());
    }

    #[test]
    fn passthrough_fields_preserved() {
        let input = basic_input();
        let form = Output4563::try_new(input).unwrap();
        assert_eq!(form.name_line1_txt, "John Doe");
        assert_eq!(form.ssn, "123-45-6789");
        assert_eq!(form.bona_fide_residence_begin_dt, "01/15/2020");
        assert_eq!(form.bona_fide_residence_end_dt, "12/31/2025");
        assert!(form.rented_house_ind);
        assert!(!form.rented_room_ind);
        assert!(form.family_living_with_you_ind);
        assert_eq!(form.family_who_and_period_txt, "Spouse, entire year");
        assert!(!form.hm_maint_outsd_american_samoa_ind);
        assert_eq!(
            form.employer_name_and_address,
            "Samoa Corp, 100 Main St, Pago Pago, AS 96799"
        );
    }

    #[test]
    fn line4b_homes_preserved() {
        let mut input = basic_input();
        input.hm_maint_outsd_american_samoa_ind = true;
        input.homes_outside_american_samoa = vec![
            F4563Line4b {
                address: "456 Oak Ave, Honolulu, HI 96801".to_string(),
                rented_ind: true,
                occupants: vec![
                    F4563Occupant {
                        occupant_nm: "Jane Doe".to_string(),
                        occupant_relationship: "Spouse".to_string(),
                    },
                    F4563Occupant {
                        occupant_nm: "Jimmy Doe".to_string(),
                        occupant_relationship: "Son".to_string(),
                    },
                ],
            },
            F4563Line4b {
                address: "789 Pine St, Los Angeles, CA 90001".to_string(),
                rented_ind: false,
                occupants: vec![F4563Occupant {
                    occupant_nm: "Bob Doe".to_string(),
                    occupant_relationship: "Son".to_string(),
                }],
            },
        ];
        let form = Output4563::try_new(input).unwrap();
        assert_eq!(form.homes_outside_american_samoa.len(), 2);
        assert_eq!(
            form.homes_outside_american_samoa[0].address,
            "456 Oak Ave, Honolulu, HI 96801"
        );
        assert!(form.homes_outside_american_samoa[0].rented_ind);
        assert_eq!(form.homes_outside_american_samoa[0].occupants.len(), 2);
        assert_eq!(
            form.homes_outside_american_samoa[0].occupants[0].occupant_nm,
            "Jane Doe"
        );
        assert_eq!(
            form.homes_outside_american_samoa[0].occupants[0].occupant_relationship,
            "Spouse"
        );
        assert_eq!(
            form.homes_outside_american_samoa[0].occupants[1].occupant_nm,
            "Jimmy Doe"
        );
        assert!(!form.homes_outside_american_samoa[1].rented_ind);
        assert_eq!(form.homes_outside_american_samoa[1].occupants.len(), 1);
        assert_eq!(
            form.homes_outside_american_samoa[1].occupants[0].occupant_relationship,
            "Son"
        );
        assert!(form.is_valid());
    }

    #[test]
    fn line6_days_absent_preserved() {
        let mut input = basic_input();
        input.days_absent = vec![
            F4563Line6 {
                date_left: "03/01/2025".to_string(),
                date_returned: "03/15/2025".to_string(),
                days_absent: 14,
                reason_for_absence: "Business trip to Hawaii".to_string(),
            },
            F4563Line6 {
                date_left: "07/04/2025".to_string(),
                date_returned: "07/18/2025".to_string(),
                days_absent: 14,
                reason_for_absence: "Family vacation".to_string(),
            },
        ];
        let form = Output4563::try_new(input).unwrap();
        assert_eq!(form.days_absent.len(), 2);
        assert_eq!(form.days_absent[0].date_left, "03/01/2025");
        assert_eq!(form.days_absent[0].date_returned, "03/15/2025");
        assert_eq!(form.days_absent[0].days_absent, 14);
        assert_eq!(
            form.days_absent[0].reason_for_absence,
            "Business trip to Hawaii"
        );
        assert_eq!(form.days_absent[1].date_left, "07/04/2025");
        assert_eq!(form.days_absent[1].reason_for_absence, "Family vacation");
        assert!(form.is_valid());
    }

    #[test]
    fn form_name_and_year() {
        assert_eq!(Output4563::name(), "Form 4563");
        let form = Output4563::try_new(basic_input()).unwrap();
        assert_eq!(form.year(), TaxYear::Y2025);
        assert_eq!(<Output4563 as Form>::form_type(), FormType::Output);
    }

    #[test]
    fn no_dependencies() {
        assert!(Output4563::dependencies().is_empty());
    }

    #[test]
    fn cents_precision_line15() {
        let mut input = basic_input();
        input.wages_exclusion_amt = Usd::from_cents(123_456);
        input.taxable_interest_exclusion_amt = Usd::from_cents(78_901);
        input.ordinary_dividends_exclusion_amt = Usd::from_cents(23_456);
        input.business_income_exclusion_amt = Usd::ZERO;
        input.capital_gain_exclusion_amt = Usd::ZERO;
        input.rental_real_estate_income_excl_amt = Usd::ZERO;
        input.farm_income_exclusion_amt = Usd::ZERO;
        input.total_other_income_exclusion_amt = Usd::ZERO;
        let form = Output4563::try_new(input).unwrap();
        // 123,456 + 78,901 + 23,456 = 225,813 cents
        assert_eq!(form.gross_income_exclusion_amt, Usd::from_cents(225_813));
        assert!(form.is_valid());
    }
}
