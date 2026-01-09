# Service Layer Migration Guide

## Overview

This document describes the internal reorganisation of the Neutryx crate structure from "Runtime Layer" (A-I-P-R) to "Service Layer" (A-I-P-S).

## What Changed

### Crate Renaming

| Old Name | New Name |
|----------|----------|
| `runtime_cli` | `service_cli` |
| `runtime_python` | `service_python` |
| `runtime_server` | `service_gateway` |

### Architecture Naming

The architecture has been renamed from **A-I-P-R** to **A-I-P-S**:

- **A**dapter: External data adapters (unchanged)
- **I**nfra: Infrastructure services (unchanged)
- **P**ricer: Pricing engine (unchanged)
- **S**ervice: Execution environments and interfaces (formerly **R**untime)

## What Has NOT Changed

### User-Facing Components

The following remain unchanged and require **no migration action**:

1. **CLI Binary Name**: `neutryx`
   ```bash
   neutryx --help
   neutryx calibrate --input market_data.csv
   neutryx price --portfolio trades.csv
   ```

2. **Server Binary Name**: `neutryx-server`
   ```bash
   neutryx-server --port 8080
   ```

3. **Python Module Name**: `neutryx`
   ```python
   import neutryx
   from neutryx import price_swap
   ```

4. **API Endpoints**: All REST API endpoints remain unchanged
   ```
   POST /api/v1/price
   POST /api/v1/calibrate
   GET  /api/v1/health
   ```

5. **Configuration Files**: All configuration file formats and paths remain unchanged

## Impact Assessment

| Component | Impact | Action Required |
|-----------|--------|-----------------|
| End Users | None | No action |
| API Consumers | None | No action |
| Python Users | None | No action |
| Internal Developers | Low | Update `use` statements if directly importing crates |
| CI/CD Pipelines | Low | Update crate references in scripts |

## For Internal Developers

If you have code that directly imports from the renamed crates:

### Before
```rust
use runtime_cli::Config;
use runtime_server::handlers::price;
use runtime_python::PyPricer;
```

### After
```rust
use service_cli::Config;
use service_gateway::handlers::price;
use service_python::PyPricer;
```

## Dependency Updates

For external projects depending on these crates via Cargo:

### Before
```toml
[dependencies]
runtime_cli = { path = "../neutryx/crates/runtime_cli" }
```

### After
```toml
[dependencies]
service_cli = { path = "../neutryx/crates/service_cli" }
```

## Rationale

The renaming from "Runtime" to "Service" better reflects the architectural role of these crates:

1. **Service** emphasises the crates' role as service providers (CLI service, API gateway service, Python binding service)
2. **Gateway** specifically highlights that `service_gateway` acts as an API gateway/entry point
3. Aligns with industry-standard terminology for system architecture

## Questions?

For questions or issues related to this migration, please open an issue in the repository.
