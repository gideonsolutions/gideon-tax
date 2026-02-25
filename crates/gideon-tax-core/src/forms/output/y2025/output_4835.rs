use crate::Usd;

/// Output fields for IRS Form 4835 (2025) — Farm Rental Income and Expenses.
#[derive(Debug, Clone, Default)]
pub struct Output4835 {
    // -----------------------------------------------------------------------
    // Top-of-form
    // -----------------------------------------------------------------------
    /// Employer ID number (EIN), if any
    pub ein: String,
    /// Line A: Did you actively participate in the operation of this farm during 2025?
    pub actively_participated_ind: bool,
    /// Passive activity loss literal code
    pub passive_activity_loss_literal_cd: String,
    /// Attachment indicator code
    pub attachment_indicator_cd: String,
    /// Section 263A indicator code
    pub section263_a_indicator_cd: String,

    // -----------------------------------------------------------------------
    // Part I — Gross Farm Rental Income — Based on Production
    // -----------------------------------------------------------------------
    /// Line 1: Income from production of livestock, produce, grains, and other crops
    pub livestock_and_crop_income_amt: Usd,
    /// Line 2a/2b: Cooperative distributions (Form(s) 1099-PATR) — total amount
    pub cooperative_distributions_amt: Usd,
    /// Line 2b: Cooperative distributions — taxable amount
    pub cooperative_distri_txbl_amt: Usd,
    /// Line 3a/3b: Agricultural program payments — total amount
    pub agricultural_program_pymt_amt: Usd,
    /// Line 3b: Agricultural program payments — taxable amount
    pub agricultural_program_pymt_txbl_amt: Usd,
    /// Line 4a: CCC loans reported under election
    pub ccc_loan_reported_election_amt: Usd,
    /// Line 4b/4c: CCC loans forfeited — total amount
    pub ccc_loans_forfeited_amt: Usd,
    /// Line 4c: CCC loans forfeited — taxable amount
    pub ccc_loans_forfeited_taxable_amt: Usd,
    /// Line 5a/5b: Crop insurance proceeds and federal crop disaster payments — total amount
    pub crop_ins_proc_and_dsstr_pymt_amt: Usd,
    /// Line 5b: Crop insurance proceeds and federal crop disaster payments — taxable amount
    pub crop_ins_proc_and_dsstr_pymt_txbl_amt: Usd,
    /// Line 5c: Election to defer crop insurance proceeds to next year
    pub election_defer_crop_ins_proc_ind: bool,
    /// Line 5d: Amount deferred from previous tax year
    pub crop_ins_proc_defrd_prev_ty_amt: Usd,
    /// Line 6: Other income, including federal and state gasoline or fuel tax credit or refund
    pub oth_incm_including_gas_fuel_tx_cr_amt: Usd,
    /// Line 7: Gross farm rental income. Add amounts in lines 1 through 6. Enter the total here and on Schedule E (Form 1040), line 42
    pub gross_farm_rental_income_amt: Usd,

    // -----------------------------------------------------------------------
    // Part II — Expenses — Farm Rental Property
    // -----------------------------------------------------------------------
    /// Line 8: Car and truck expenses (see Schedule F (Form 1040) instructions). Also, attach Form 4562
    pub car_and_truck_expenses_amt: Usd,
    /// Line 9: Chemicals
    pub chemical_expense_amt: Usd,
    /// Line 10: Conservation expenses (see instructions)
    pub conservation_expense_amt: Usd,
    /// Line 11: Custom hire (machine work)
    pub custom_hire_expense_amt: Usd,
    /// Line 12: Depreciation and section 179 expense deduction not claimed elsewhere
    pub deprec_and_sect179_expns_ded_amt: Usd,
    /// Line 13: Employee benefit programs other than on line 21 (see Schedule F (Form 1040) instructions)
    pub employee_benefit_program_amt: Usd,
    /// Line 14: Feed
    pub feed_purchased_expense_amt: Usd,
    /// Line 15: Fertilizers and lime
    pub fertilizer_and_lime_expense_amt: Usd,
    /// Line 16: Freight and trucking
    pub freight_and_trucking_expense_amt: Usd,
    /// Line 17: Gasoline, fuel, and oil
    pub gasoline_fuel_and_oil_expense_amt: Usd,
    /// Line 18: Insurance (other than health)
    pub insurance_amt: Usd,
    /// Line 19a: Interest — Mortgage (paid to banks, etc.)
    pub mortgage_interest_paid_banks_amt: Usd,
    /// Line 19b: Interest — Other
    pub mortgage_interest_paid_other_amt: Usd,
    /// Line 20: Labor hired (less employment credits) (see Schedule F (Form 1040) instructions)
    pub labor_hired_expense_amt: Usd,
    /// Line 21: Pension and profit-sharing plans
    pub pension_profit_sharing_plans_amt: Usd,
    /// Line 22a: Rent or lease — Vehicles, machinery, and equipment (see instructions)
    pub machinery_and_equipment_rent_amt: Usd,
    /// Line 22b: Rent or lease — Other (land, animals, etc.)
    pub other_business_property_rent_amt: Usd,
    /// Line 23: Repairs and maintenance
    pub repairs_and_maintenance_amt: Usd,
    /// Line 24: Seeds and plants
    pub seed_and_plant_expense_amt: Usd,
    /// Line 25: Storage and warehousing
    pub storage_and_warehousing_expns_amt: Usd,
    /// Line 26: Supplies
    pub supplies_amt: Usd,
    /// Line 27: Taxes
    pub tax_expense_amt: Usd,
    /// Line 28: Utilities
    pub utilities_amt: Usd,
    /// Line 29: Veterinary, breeding, and medicine
    pub vtrnry_breeding_medicine_expns_amt: Usd,
    /// Line 30: Other expenses (specify)
    pub other_expense: String,
    /// Line 31: Total expenses. Add lines 8 through 30g
    pub total_expenses_amt: Usd,
    /// Line 32: Net farm rental income or (loss). Subtract line 31 from line 7
    pub net_farm_rental_income_or_loss_amt: Usd,
    /// Line 34a: All investment is at risk
    pub all_investment_is_at_risk_ind: bool,
    /// Line 34b: Some investment is not at risk
    pub some_investment_is_not_at_risk_ind: bool,
    /// Line 34c: Farm rental deductible loss (from Form 8582)
    pub farm_rental_deductible_loss_amt: Usd,
}
