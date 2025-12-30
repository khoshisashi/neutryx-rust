# Requirements Document

## Project Description (Input)
Phase 2.2: Instrument Definitions - Implement enum dispatch architecture for financial instruments without using trait objects. This includes: (1) Instrument enum with variants for European/American/Asian options, forward contracts, and swaps, (2) PayoffType enum for Call/Put/Digital with smooth payoff functions, (3) InstrumentParams struct with shared parameters (strike, expiry, notional), (4) No Box<dyn Trait> - use enum dispatch for Enzyme AD compatibility, (5) All types generic over T: Float

## Requirements
<!-- Will be generated in /kiro:spec-requirements phase -->

