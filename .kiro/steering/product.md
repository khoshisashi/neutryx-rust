# Product Overview

A production-grade **XVA (Credit Valuation Adjustment) pricing library** for derivatives portfolios, delivering bank-grade CVA/DVA/FVA calculations with cutting-edge performance through Enzyme automatic differentiation.

## Architecture: A-I-P-S Stream

The workspace enforces a strict unidirectional data flow that mirrors alphabetical order:

1. **A**dapter: Ingestion and normalisation of external data (The Raw Inputs)
2. **I**nfra: System-wide definitions, persistence, and configuration (The Foundation)
3. **P**ricer: Mathematical modelling, optimisation, and risk computation (The Kernel)
4. **S**ervice: Execution environments and interfaces (The Outputs)

## Core Capabilities

- **Credit Valuation Adjustments**: CVA, DVA, FVA calculations for derivatives portfolios
- **Exposure Metrics**: EE, EPE, PFE, EEPE, ENE calculations with parallel computation
- **Multi-Asset Class Instruments**: Equity, Rates (IRS, Swaption, Cap/Floor), Credit (CDS), FX derivatives
- **High-Performance Greeks**: Enzyme LLVM-level AD for C++-competitive differentiation
- **Dual-Mode Verification**: Parallel Enzyme and num-dual backends for correctness validation
- **Monte Carlo Pricing**: Path-dependent options with workspace buffers and checkpointing
- **Thread-Local Buffer Pool**: Allocation-free simulation with RAII buffer management
- **Path-Dependent Options**: Asian (arithmetic/geometric), Barrier (all 8 variants), Lookback (fixed/floating) with streaming statistics
- **Analytical Solutions**: Geometric Asian (Kemna-Vorst), Barrier (Merton/Rubinstein-Reiner) for MC verification
- **Portfolio Analytics**: Parallelised portfolio-level XVA computations with SoA optimisation
- **Market Data Infrastructure**: AD-compatible yield curves and volatility surfaces with interpolation
- **Model Calibration**: Swaption volatility surface calibration with Levenberg-Marquardt (pricer_optimiser)
- **Interest Rate Models**: Hull-White, Cox-Ingersoll-Ross (CIR) with mean reversion
- **Correlated Processes**: Multi-factor correlation via Cholesky decomposition

## Target Use Cases

- **Quantitative Finance**: Pricing and risk management for derivatives trading desks
- **Risk Analytics**: Portfolio-level credit exposure and valuation adjustments
- **Research & Validation**: Dual-mode verification enables academic/production validation
- **Performance Benchmarking**: LLVM-level AD vs traditional finite difference methods

## Value Proposition

- **Unidirectional Data Flow**: A-I-P-S architecture ensures clear separation of concerns
- **Isolation of Experimental Code**: Enzyme confined to pricer_pricing, keeping 75% of codebase production-stable
- **Correctness First**: Built-in verification through dual AD backends (Enzyme + num-dual)
- **Differentiability by Design**: Smooth approximations replace discontinuities throughout
- **Performance Without Compromise**: Static dispatch and LLVM optimisation for zero-cost abstractions

---
_Created: 2025-12-29_
_Updated: 2026-01-09_ — Migrated to A-I-P-S architecture (v2.1)
_Focus on patterns and purpose, not exhaustive feature lists_
