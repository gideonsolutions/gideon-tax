/// Output fields for IRS Schedule EIC (Form 1040) 2025 — Earned Income Credit.
#[derive(Debug, Clone, Default)]
pub struct OutputScheduleEic {
    // -----------------------------------------------------------------------
    // Qualifying Child Information
    // -----------------------------------------------------------------------
    // Line 1: Child's name (first and last)
    // (This is not a separate field; the child is identified by qualifying_child_ssn)

    /// Line 2: Child's SSN (The child must have an SSN as defined in the instructions for
    /// Form 1040, line 27a, unless the child was born and died in 2025)
    pub qualifying_child_ssn: String,
    /// Line 2: "Died" literal code if the child was born and died in 2025 and did not have an SSN
    pub died_literal_cd: String,
    /// Line 3: Child's year of birth
    pub child_birth_yr: u16,
    /// Line 4a: Was the child under age 24 at the end of 2025, a student, and younger than
    /// you (or your spouse if filing jointly)?
    pub child_is_a_student_under24_ind: bool,
    /// Line 4b: Was the child permanently and totally disabled during any part of 2025?
    pub child_permanently_disabled_ind: bool,
    /// Line 5: Child's relationship to you (e.g., son, daughter, grandchild, niece, nephew,
    /// eligible foster child, etc.)
    pub child_relationship_cd: String,
    /// Line 6: Number of months child lived with you in the United States during 2025
    pub months_child_lived_with_you_cnt: u32,
    /// Kidnapped child code (if applicable, see instructions)
    pub kidnapped_child_cd: String,
}
