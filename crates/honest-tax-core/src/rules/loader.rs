//! Tax rules loader - loads rules from JSON data files.

use crate::error::{TaxError, TaxResult};
use crate::rules::Rules2025;
use crate::traits::TaxRules;
use crate::types::{TaxYear, MAX_SUPPORTED_YEAR, MIN_SUPPORTED_YEAR};
use std::sync::Arc;

/// Loader for tax rules by year.
///
/// Provides access to year-specific tax rules, loading from
/// embedded data or external JSON files.
#[derive(Debug, Default)]
pub struct RulesLoader {
    // Could cache loaded rules here in the future
}

impl RulesLoader {
    /// Creates a new rules loader.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the tax rules for the given year.
    pub fn load(&self, year: TaxYear) -> TaxResult<Arc<dyn TaxRules>> {
        match year {
            2025 => Ok(Arc::new(Rules2025::new())),
            _ => Err(TaxError::UnsupportedTaxYear(
                year,
                MIN_SUPPORTED_YEAR,
                MAX_SUPPORTED_YEAR,
            )),
        }
    }

    /// Returns true if the given year is supported.
    pub fn is_supported(&self, year: TaxYear) -> bool {
        (MIN_SUPPORTED_YEAR..=MAX_SUPPORTED_YEAR).contains(&year)
    }

    /// Returns the range of supported years.
    pub fn supported_years(&self) -> std::ops::RangeInclusive<TaxYear> {
        MIN_SUPPORTED_YEAR..=MAX_SUPPORTED_YEAR
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_2025() {
        let loader = RulesLoader::new();
        let rules = loader.load(2025).unwrap();
        assert_eq!(rules.year(), 2025);
    }

    #[test]
    fn test_unsupported_year() {
        let loader = RulesLoader::new();
        assert!(loader.load(2020).is_err());
    }

    #[test]
    fn test_is_supported() {
        let loader = RulesLoader::new();
        assert!(loader.is_supported(2025));
        assert!(!loader.is_supported(2020));
    }
}
