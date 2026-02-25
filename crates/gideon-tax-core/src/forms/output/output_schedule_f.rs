use crate::Usd;

/// Output fields for IRS Schedule F (Form 1040) 2025 — Profit or Loss From Farming.
#[derive(Debug, Clone, Default)]
pub struct OutputScheduleF {
    // -----------------------------------------------------------------------
    // Header
    // -----------------------------------------------------------------------
    /// Line A: Principal crop or activity
    pub principal_product_desc: String,
    /// Line B: Enter code from Part IV (agricultural activity code)
    pub agricultural_activity_cd: String,
    /// Line C: Accounting method — Cash
    pub method_of_accounting_cash_ind: bool,
    /// Line C: Accounting method — Accrual
    pub method_of_accounting_accrual_ind: bool,
    /// Line D: Employer ID number (EIN)
    pub ein: String,
    /// Line E: Did you "materially participate" in the operation of this business during 2025? If "No," see instructions for limit on passive losses
    pub materially_participated_ind: bool,
    /// Line F: Did you make any payments in 2025 that would require you to file Form(s) 1099? See instructions
    pub required_to_file_forms_1099_ind: bool,
    /// Line G: If "Yes," did you or will you file required Form(s) 1099?
    pub required_forms_1099_filed_ind: bool,
    /// Farm proprietor name, line 1
    pub business_name_line_1_txt: String,
    /// Farm proprietor name, line 2
    pub business_name_line_2_txt: String,

    // -----------------------------------------------------------------------
    // Part I — Farm Income—Cash Method
    // -----------------------------------------------------------------------
    /// Line 1a: Sales of purchased livestock and other resale items (see instructions)
    pub cash_sales_of_lvstck_bght_for_resale_amt: Usd,
    /// Line 1b: Cost or other basis of purchased livestock or other items reported on line 1a
    pub cash_cost_of_lvstck_bght_for_resale_amt: Usd,
    /// Line 1c: Subtract line 1b from line 1a
    pub cash_purchased_profit_amt: Usd,
    /// Line 2: Sales of livestock, produce, grains, and other products you raised
    pub cash_sale_of_products_raised_amt: Usd,
    /// Line 3a: Cooperative distributions (Form(s) 1099-PATR)
    pub cash_cooperative_distributions_amt: Usd,
    /// Line 3b: Taxable amount
    pub cash_cooperative_distri_txbl_amt: Usd,
    /// Line 4a: Agricultural program payments (see instructions)
    pub cash_agricultural_program_pymt_amt: Usd,
    /// Line 4b: Taxable amount
    pub cash_agricultural_program_pymt_txbl_amt: Usd,
    /// Line 5a: Commodity Credit Corporation (CCC) loans reported under election
    pub cash_ccc_loan_reported_election_amt: Usd,
    /// Line 5b: CCC loans forfeited
    pub cash_ccc_loans_forfeited_amt: Usd,
    /// Line 5c: Taxable amount
    pub cash_ccc_loans_forfeited_taxable_amt: Usd,
    /// Line 6: Crop insurance proceeds and federal crop disaster payments (see instructions)
    pub cash_crop_ins_proc_and_dsstr_pymt_amt: Usd,
    /// Line 6a: Amount received in 2025
    pub cash_crop_ins_proc_and_dsstr_pymt_txbl_amt: Usd,
    /// Line 6c: If election to defer to 2026 is attached, check here
    pub cash_election_defer_crop_ins_proc_ind: bool,
    /// Line 6d: Amount deferred from 2024
    pub cash_crop_ins_proc_defrd_prev_ty_amt: Usd,
    /// Line 7: Custom hire (machine work) income
    pub cash_custom_hire_income_amt: Usd,
    /// Line 8: Other income, including federal and state gasoline or fuel tax credit or refund (see instructions)
    pub cash_other_income_amt: Usd,
    /// Line 9: Gross income. Add amounts in the right column (lines 1c, 2, 3b, 4b, 5a, 5c, 6b, 6d, 7, and 8). If you use the
    /// accrual method, enter the amount from Part III, line 50. See instructions
    pub cash_gross_income_amt: Usd,

    // -----------------------------------------------------------------------
    // Part II — Farm Expenses—Cash and Accrual Method
    // -----------------------------------------------------------------------
    /// Line 10: Car and truck expenses (see instructions). Also attach Form 4562
    pub car_and_truck_expenses_amt: Usd,
    /// Line 11: Chemicals
    pub chemical_expense_amt: Usd,
    /// Line 12: Conservation expenses (see instructions)
    pub conservation_expense_amt: Usd,
    /// Line 13: Custom hire (machine work)
    pub custom_hire_expense_amt: Usd,
    /// Line 14: Depreciation and section 179 expense (see instructions)
    pub deprec_and_sect_179_expns_ded_amt: Usd,
    /// Line 15: Employee benefit programs other than on line 23
    pub employee_benefit_program_amt: Usd,
    /// Line 16: Feed
    pub feed_purchased_expense_amt: Usd,
    /// Line 17: Fertilizers and lime
    pub fertilizer_and_lime_expense_amt: Usd,
    /// Line 18: Freight and trucking
    pub freight_and_trucking_expense_amt: Usd,
    /// Line 19: Gasoline, fuel, and oil
    pub gasoline_fuel_and_oil_expense_amt: Usd,
    /// Line 20: Insurance (other than health)
    pub insurance_amt: Usd,
    /// Line 21a: Mortgage (paid to banks, etc.)
    pub mortgage_interest_paid_banks_amt: Usd,
    /// Line 21b: Other
    pub mortgage_interest_paid_other_amt: Usd,
    /// Line 22: Labor hired (less employment credits)
    pub labor_hired_expense_amt: Usd,
    /// Line 23: Pension and profit-sharing plans
    pub pension_profit_sharing_plans_amt: Usd,
    /// Line 24a: Vehicles, machinery, equipment
    pub machinery_and_equipment_rent_amt: Usd,
    /// Line 24b: Other (land, animals, etc.)
    pub other_business_property_rent_amt: Usd,
    /// Line 25: Repairs and maintenance
    pub repairs_and_maintenance_amt: Usd,
    /// Line 26: Seeds and plants
    pub seed_and_plant_expense_amt: Usd,
    /// Line 27: Storage and warehousing
    pub storage_and_warehousing_expns_amt: Usd,
    /// Line 28: Supplies
    pub supplies_amt: Usd,
    /// Line 29: Taxes
    pub tax_expense_amt: Usd,
    /// Line 30: Utilities
    pub utilities_amt: Usd,
    /// Line 31: Veterinary, breeding, and medicine
    pub vtrnry_breeding_medicine_expns_amt: Usd,
    /// Line 32: Other expenses (specify)
    pub total_preproductive_prd_expns_amt: Usd,
    /// Line 33: Total expenses. Add lines 10 through 32f. If line 32f is negative, see instructions
    pub total_expenses_amt: Usd,
    /// Line 34: Net farm profit or (loss). Subtract line 33 from line 9
    pub net_farm_profit_loss_amt: Usd,
    /// Line 36a: All investment is at risk
    pub all_investment_is_at_risk_ind: bool,
    /// Line 36b: Some investment is not at risk
    pub some_investment_is_not_at_risk_ind: bool,

    // -----------------------------------------------------------------------
    // Part III — Farm Income—Accrual Method (see instructions)
    // -----------------------------------------------------------------------
    /// Line 37: Sales of livestock, produce, grains, and other products (see instructions)
    pub accrual_sales_livestock_produce_prod_amt: Usd,
    /// Line 38a: Cooperative distributions (Form(s) 1099-PATR)
    pub accrual_cooperative_distributions_amt: Usd,
    /// Line 38b: Taxable amount
    pub accrual_cooperative_distri_txbl_amt: Usd,
    /// Line 39a: Agricultural program payments
    pub accrual_agricultural_program_pymt_amt: Usd,
    /// Line 39b: Taxable amount
    pub accrual_agricultural_program_pymt_txbl_amt: Usd,
    /// Line 40a: CCC loans reported under election
    pub accrual_ccc_loan_reported_election_amt: Usd,
    /// Line 40b: CCC loans forfeited
    pub accrual_ccc_loans_forfeited_amt: Usd,
    /// Line 40c: Taxable amount
    pub accrual_ccc_loans_forfeited_taxable_amt: Usd,
    /// Line 41: Crop insurance proceeds
    pub accrual_crop_ins_proc_and_dsstr_pymt_amt: Usd,
    /// Line 42: Custom hire (machine work) income
    pub accrual_custom_hire_income_amt: Usd,
    /// Line 43: Other income (see instructions)
    pub accrual_other_income_amt: Usd,
    /// Line 44: Add amounts in the right column for lines 37 through 43 (lines 37, 38b, 39b, 40a, 40c, 41, 42, and 43)
    pub accrual_total_income_amt: Usd,
    /// Line 45: Inventory of livestock, produce, grains, and other products at beginning of the year. Do
    /// not include sales reported on Form 4797
    pub accrual_inventory_of_products_at_boy_amt: Usd,
    /// Line 46: Cost of livestock, produce, grains, and other products purchased during the year
    pub accrual_cost_of_products_prchsd_during_yr_amt: Usd,
    /// Line 47: Add lines 45 and 46
    pub accrual_invntry_at_boy_plus_cost_of_prchs_amt: Usd,
    /// Line 48: Inventory of livestock, produce, grains, and other products at end of year
    pub accrual_inventory_of_products_at_eoy_amt: Usd,
    /// Line 49: Cost of livestock, produce, grains, and other products sold. Subtract line 48 from line 47
    pub accrual_cost_of_products_sold_amt: Usd,
    /// Line 50: Gross income. Subtract line 49 from line 44. Enter the result here and on Part I, line 9
    pub accrual_gross_income_amt: Usd,
}
