## Phase 3: Ephemeris-Backed Solar-System Slice

### Step 1: Pin ephemeris provenance and assets
- [ ] Status marker
- Files: `data/`, `docs/scientific-contract.md`, `Cargo.toml`, `crates/solar-core/Cargo.toml`
- Changes: Pin the chosen ephemeris provenance, version, coverage, license/source metadata, checksum, and reproducible asset-generation or packaging process. Avoid runtime network downloads in the core.
- Tests: Verify asset metadata and reproducible loading.
- Risk: High - data licensing, size, and reproducibility affect the project foundation.

### Step 2: Implement the ephemeris provider
- [ ] Status marker
- Files: `crates/solar-core/src/ephemeris.rs`, `crates/solar-core/src/provider.rs`, `crates/solar-core/src/error.rs`, `crates/solar-core/tests/`
- Changes: Implement source-coordinate conversion into heliocentric J2000 ecliptic coordinates in AU, with explicit metadata and typed failures for unsupported coverage or malformed data.
- Tests: Validate known epochs, finite output, frame metadata, and supported-range rejection.
- Risk: High - frame and time conversion errors can produce plausible but incorrect positions.

### Step 3: Connect provider queries and serialization
- [ ] Status marker
- Files: `crates/solar-core/src/simulation.rs`, `crates/solar-core/src/ephemeris.rs`, `crates/solar-core/tests/`
- Changes: Connect the provider to single-body and all-body queries with deterministic collection serialization.
- Tests: Add repeated-query, finite-output, boundary-date, and serialization tests.
- Risk: Medium - query ordering and serialized stability must remain deterministic.
