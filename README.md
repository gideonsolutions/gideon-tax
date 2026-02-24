# GideonTax

Deterministic US federal income tax calculator written in Rust.

## Overview

GideonTax aims to be a fully deterministic, auditable tax engine for US federal individual income tax. The project is built as a Rust workspace with modular crates:

- **gideon-tax-core** - Core tax calculation logic and types (filing status, tax brackets, etc.)

IRS form schemas live in the [irs-form-schema](https://github.com/gideonsolutions/irs-form-schema) submodule.

## Project Structure

```
gideon-tax/
├── crates/
│   └── gideon-tax-core/    # Core tax engine
├── irs-form-schema/         # IRS form schemas (submodule)
├── prompts/                 # Research prompts for IRS rule extraction
└── Cargo.toml               # Workspace configuration
```

## Tax Year Coverage

- **2025** - Federal individual income tax forms (24 source forms, 32 return forms)

## Building

```sh
cargo build
```

## License

Licensed under the [Gideon Christian Open Source License (GCOSL) v1.0](LICENSE).
