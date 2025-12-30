# Security Design Description (Rust Edition)

## TOE Design

### Subsystems

#### L1: PricerCore (Foundation)

* **Description**: Safe abstractions for math, types, and traits.
* **Modules**: `DualNumber`, `DayCount`, `Smoothing`, `Priceable`

#### L2: PricerModels (Business Logic)

* **Description**: Financial instruments and stochastic models.
* **Modules**: `VanillaOption`, `InterestRateSwap`, `BlackScholes`, `HestonModel` (Planned), `SABRModel` (Planned)

#### L3: PricerKernel (Computation)

* **Description**: Unsafe AD bindings and Monte Carlo engine.
* **Modules**: `EnzymeContext`, `MonteCarloEngine`, `PathGenerator`

#### L4: PricerXVA (Application)

* **Description**: Portfolio aggregation and risk metrics.
* **Modules**: `CVAEngine`, `ExposureCalculator`, `NettingSet`
