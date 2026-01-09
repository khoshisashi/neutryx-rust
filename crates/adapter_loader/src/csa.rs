//! CSA (Credit Support Annex) terms and netting set configuration.

use pricer_core::types::Currency;

/// Credit Support Annex terms.
///
/// Defines the collateral agreement between counterparties.
#[derive(Debug, Clone)]
pub struct CsaTerms {
    /// CSA identifier
    pub csa_id: String,
    /// Threshold amount (exposure below which no collateral is required)
    pub threshold: f64,
    /// Minimum transfer amount
    pub minimum_transfer_amount: f64,
    /// Independent amount (initial margin)
    pub independent_amount: f64,
    /// Collateral currency
    pub currency: Currency,
    /// Margin period of risk (in days)
    pub margin_period_of_risk: u32,
}

impl Default for CsaTerms {
    fn default() -> Self {
        Self {
            csa_id: String::new(),
            threshold: 0.0,
            minimum_transfer_amount: 0.0,
            independent_amount: 0.0,
            currency: Currency::USD,
            margin_period_of_risk: 10,
        }
    }
}

/// Netting set configuration.
///
/// Defines how trades are grouped for netting purposes.
#[derive(Debug, Clone)]
pub struct NettingSetConfig {
    /// Netting set identifier
    pub netting_set_id: String,
    /// Counterparty identifier
    pub counterparty_id: String,
    /// Associated CSA terms (if any)
    pub csa_terms: Option<CsaTerms>,
    /// Whether close-out netting applies
    pub closeout_netting: bool,
}

impl NettingSetConfig {
    /// Create a new netting set configuration.
    pub fn new(netting_set_id: impl Into<String>, counterparty_id: impl Into<String>) -> Self {
        Self {
            netting_set_id: netting_set_id.into(),
            counterparty_id: counterparty_id.into(),
            csa_terms: None,
            closeout_netting: true,
        }
    }

    /// Set CSA terms for this netting set.
    pub fn with_csa(mut self, csa: CsaTerms) -> Self {
        self.csa_terms = Some(csa);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_netting_set_config() {
        let config = NettingSetConfig::new("NS001", "CP001");
        assert_eq!(config.netting_set_id, "NS001");
        assert_eq!(config.counterparty_id, "CP001");
        assert!(config.closeout_netting);
    }
}
