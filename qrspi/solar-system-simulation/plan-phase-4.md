## Phase 4: Accuracy, Extensibility, and Release Readiness

### Step 1: Add accuracy validation
- [ ] Status marker
- Files: `crates/solar-core/tests/accuracy.rs`, `crates/solar-core/src/validation.rs`, `docs/scientific-contract.md`
- Changes: Add independent fixtures across the selected range, including Mercury and Neptune, and report per-body absolute distance, relative vector, maximum, RMS, and aggregate results.
- Tests: Fail validation when the chosen acceptance tolerance is exceeded.
- Risk: High - the 99.999% target must use a precise, reproducible metric.

### Step 2: Prove future-body compatibility
- [ ] Status marker
- Files: `crates/solar-core/tests/extensibility.rs`, `crates/solar-core/src/catalog.rs`, `crates/solar-core/src/domain.rs`
- Changes: Add synthetic future moon, dwarf planet, or asteroid entries and prove catalog, query, state, and serialization contracts require no schema changes.
- Tests: Verify future-body records coexist with the initial nine bodies.
- Risk: Medium - fixed-field assumptions can be hidden in serialization or queries.

### Step 3: Document and run release checks
- [ ] Status marker
- Files: `docs/scientific-contract.md`, `README.md`, `Cargo.toml`, workspace source and tests as needed
- Changes: Document conventions, limitations, provenance, supported range, reproducibility expectations, and future provider paths. Run all formatting, linting, unit/integration, and validation checks.
- Tests: `cargo fmt --check`, `cargo clippy --all-targets --all-features -- -D warnings`, and `cargo test --workspace`.
- Risk: Low - final integration checkpoint.
