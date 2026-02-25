use crate::Usd;

/// Output fields for IRS Form 8936 Schedule A (2025) — Clean Vehicle Credit Amount.
#[derive(Debug, Clone, Default)]
pub struct Output8936ScheduleA {
    // -----------------------------------------------------------------------
    // Part I — Vehicle Details
    // -----------------------------------------------------------------------
    /// Line 1a: Year
    pub vehicle_model_yr: u16,
    /// Line 1b: Make
    pub vehicle_make_name_txt: String,
    /// Line 1c: Model
    pub vehicle_model_name_txt: String,
    /// Line 2: Vehicle identification number (VIN)
    pub vin: String,
    /// Line 3: Enter date vehicle was placed in service (MM/DD/YYYY)
    pub vehicle_placed_in_service_dt: String,
    /// Line 4a: Did you transfer the credit to the dealer at the time of sale? (Yes indicator)
    pub cr_trnsfr_dlr_sale_ind: bool,
    /// Line 4a: Transferred amount shown on the seller's report
    pub cr_trnsfr_dlr_sale_amt: Usd,
    /// Line 5: Does the VIN belong to a new clean vehicle? (Yes indicator)
    pub new_cln_veh_service_ty_yes_ind: bool,
    /// Line 5: Does the VIN belong to a new clean vehicle? (No indicator)
    pub new_cln_veh_service_ty_no_ind: bool,
    /// Line 6: Does the VIN belong to a previously owned clean vehicle? (Yes — see Part IV)
    pub prev_own_cln_veh_service_ty_yes_ind: bool,
    /// Line 6: Does the VIN belong to a previously owned clean vehicle? (No indicator)
    pub prev_own_cln_veh_service_ty_no_ind: bool,
    /// Line 7: Does the VIN belong to a qualified commercial clean vehicle? (Yes — see Part V)
    pub qlfy_cmrcl_cln_veh_srvc_ty_yes_ind: bool,
    /// Line 7: VIN belongs to a qualified commercial clean vehicle placed in service during the tax year (indicator)
    pub vin_qlfy_cmrcl_cln_veh_in_srvc_ty_ind: bool,

    // -----------------------------------------------------------------------
    // Part II — Credit Amount for Business/Investment Use Part of New Clean Vehicle
    // -----------------------------------------------------------------------
    /// Line 8a: Did you resell the vehicle within 30 days of the placed-in-service date? (Yes indicator)
    pub resell_cln_veh30_days_ind: bool,
    /// Line 8b: Are you filing this form with an individual income tax return? (Yes indicator)
    pub filing_form_iitr_ind: bool,
    /// Line 8c: Is line 2 more than the "Part II/III limits" amount for your 2025 filing status? (Yes indicator)
    pub amt_grtr_than_cyfs_limit_ind: bool,
    /// Line 8d: Is line 4 more than the "Part II/III limits" amount for your 2024 filing status? (Yes indicator)
    pub amt_grtr_than_pyfs_limit_ind: bool,
    /// Line 8e: Did you acquire the vehicle for use or to lease to others, and not for resale? (Yes indicator)
    pub acq_veh_use_or_lease_not_resale_ind: bool,
    /// Line 9: Tentative credit amount (see instructions)
    pub tentative_credit_amt: Usd,
    /// Line 10: Business/investment use percentage (see instructions)
    pub business_investment_use_pct: String,
    /// Line 11: Multiply line 9 by line 10. Business/investment use credit amount
    pub business_investment_use_amt: Usd,

    // -----------------------------------------------------------------------
    // Part III — Credit Amount for Personal Use Part of New Clean Vehicle
    // -----------------------------------------------------------------------
    /// Line 12: Subtract line 11 from line 9 in Part II. Personal use new clean vehicle credit amount
    pub prsnl_use_new_clean_vehicle_cr_amt: Usd,

    // -----------------------------------------------------------------------
    // Part IV — Credit Amount for Previously Owned Clean Vehicle
    // -----------------------------------------------------------------------
    /// Line 13a: Did you resell the vehicle within 30 days? (Yes indicator)
    pub prev_own_resell_cln_veh30_days_ind: bool,
    /// Line 13b: Is line 2 more than the "Part IV limits" amount for your 2025 filing status? (Yes indicator)
    pub prev_own_amt_grtr_than_cyfs_limit_ind: bool,
    /// Line 13c: Is line 4 more than the "Part IV limits" amount for your 2024 filing status? (Yes indicator)
    pub prev_own_amt_grtr_than_pyfs_limit_ind: bool,
    /// Line 13d: Have you claimed a previously owned clean vehicle credit for another vehicle in the 3-year period? (Yes indicator)
    pub not_allowed_claim_cln_veh_cr_ind: bool,
    /// Line 13e: Is the sales price of the vehicle more than $25,000? (Yes indicator)
    pub cln_veh_sale_price_more_spcfd_amt_ind: bool,
    /// Line 13f: Did you acquire the vehicle for use and not for resale? (Yes indicator)
    pub acq_prev_own_veh_use_not_resale_ind: bool,
    /// Line 13g: Can you be claimed as a dependent on another person's tax return? (Yes indicator)
    pub claimed_as_dependent_ind: bool,
    /// Line 14: Enter the sales price of the vehicle
    pub sale_price_amt: Usd,
    /// Line 15: Multiply line 14 by 30% (0.30)
    pub sale_price_by_specified_pct_amt: Usd,
    /// Line 16: Maximum vehicle credit amount ($4,000)
    pub max_qlfy_cmrcl_clean_veh_cr_amt: Usd,
    /// Line 17: Enter the smaller of line 15 or line 16. Previously owned clean vehicle credit amount
    pub prev_owned_clean_veh_credit_amt: Usd,

    // -----------------------------------------------------------------------
    // Part V — Credit Amount for Qualified Commercial Clean Vehicle
    // -----------------------------------------------------------------------
    /// Part V: Did you acquire the commercial clean vehicle for use and not for resale? (Yes indicator)
    pub acq_cmrcl_cln_veh_use_not_resale_ind: bool,
    /// Part V: Gross vehicle weight rating
    pub gross_vehicle_weight_rating_num: String,
    /// Part V: IRS-issued registration number
    pub irs_issued_registration_num: String,
    /// Part V: Is the vehicle of a character subject to the allowance for depreciation? (Yes indicator)
    pub veh_of_char_subj_to_allwnc_deprec_ind: bool,
    /// Part V: Is the vehicle powered by a gasoline or diesel internal combustion engine? (Yes indicator)
    pub vehicle_powered_by_gas_or_diesel_ind: bool,
    /// Part V: Vehicle cost or other basis amount
    pub vehicle_cost_or_other_basis_amt: Usd,
    /// Part V: Vehicle incremental cost amount
    pub vehicle_incremental_cost_amt: Usd,
    /// Part V: Tentative qualified commercial clean vehicle credit amount
    pub tent_qlfy_cmrcl_clean_vehicle_cr_amt: Usd,
    /// Part V: Section 179 expense deduction amount
    pub section179_expense_deduction_amt: Usd,
    /// Part V: Net section 179 expense deduction amount
    pub net_sect179_expense_ded_amt: Usd,
    /// Part V: Net section 179 expense deduction percentage amount
    pub net_sect179_expense_ded_pct_amt: Usd,
    /// Part V: Qualified commercial clean vehicle credit amount
    pub qlfy_cmrcl_clean_vehicle_cr_amt: Usd,
}
