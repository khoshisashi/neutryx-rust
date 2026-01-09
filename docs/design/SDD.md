# System Design Description (Rust Edition)

## Overview

Neutryx is a production-grade **derivatives pricing library** for Tier-1 banks, featuring:
- **Multi-Asset Class Coverage**: Rates, FX, Equity, Credit, Commodity derivatives
- **High-Performance AD**: Enzyme LLVM-level automatic differentiation
- **Risk Analytics**: XVA (CVA/DVA/FVA), exposure metrics, portfolio aggregation
- **A-I-P-S Architecture**: Adapter → Infra → Pricer → Service unidirectional data flow

## TOE Design

### Subsystems

#### L1: PricerCore (Foundation)

* **Description**: Safe abstractions for math, types, traits, and market data.
* **Modules**: `DualNumber`, `DayCount`, `Smoothing`, `Priceable`, `YieldCurve`, `VolatilitySurface`

#### L2: PricerModels (Business Logic)

* **Description**: Financial instruments and stochastic models across all asset classes.
* **Instruments**:
  * `instruments/equity/` - Vanilla options, barriers, Asians, lookbacks
  * `instruments/rates/` - IRS, Swaptions, Cap/Floor, OIS, Basis swaps
  * `instruments/credit/` - CDS, CDS Index, Credit-linked notes
  * `instruments/fx/` - FX options, forwards, NDFs, barriers
  * `instruments/commodity/` - Forwards, options, spread options
* **Models**:
  * `models/equity/` - GBM, Local Vol, Heston, SABR
  * `models/rates/` - Hull-White, CIR, LMM (planned)
  * `models/hybrid/` - Correlated multi-factor models

#### L3: PricerPricing (Computation)

* **Description**: Pricing engines with Enzyme AD integration.
* **Modules**: `MonteCarloEngine`, `PDEEngine` (planned), `AnalyticalSolutions`, `EnzymeContext`

#### L4: PricerRisk (Application)

* **Description**: Portfolio analytics, XVA, and risk metrics.
* **Modules**: `XVACalculator`, `ExposureCalculator`, `RegulatoryMetrics` (planned), `NettingSet`
