## Phase 2: Time and Provider Boundaries

### Step 1: Implement simulation time and state transitions
- [ ] Status marker
- Files: `crates/solar-core/src/simulation.rs`, `crates/solar-core/src/error.rs`, `crates/solar-core/tests/`
- Changes: Implement initialization, current time, set-time, signed advancement, pause, forward/reverse progression, and supported-range boundaries.
- Tests: Cover successful transitions, pause no-op, reverse time, large valid steps, boundary dates, and invalid-input rollback.
- Risk: Medium - state mutation must remain atomic when validation fails.

### Step 2: Define the model/provider boundary
- [ ] Status marker
- Files: `crates/solar-core/src/provider.rs`, `crates/solar-core/src/simulation.rs`, `crates/solar-core/src/lib.rs`, `crates/solar-core/tests/`
- Changes: Define a provider contract accepting documented dynamical Julian Date values and returning collection-compatible body states, independent of ephemeris lookup or future n-body integration.
- Tests: Use a deterministic test provider to verify orchestration and query behavior.
- Risk: Medium - provider abstraction must not leak implementation-specific data shapes.

### Step 3: Finalize scientific conventions
- [ ] Status marker
- Files: `docs/scientific-contract.md`, `crates/solar-core/src/domain.rs`, `crates/solar-core/src/provider.rs`
- Changes: Choose DE440 or DE441, exact supported range, JD(TDB) input, heliocentric J2000 ecliptic realization, Sun-position semantics, source-to-public transformation, and positions-only first-slice behavior.
- Tests: Verify documented constants and metadata agree with the selected conventions.
- Risk: High - incorrect choices invalidate accuracy validation and future integrations.
