## Phase 1: Workspace and Domain Contract

### Step 1: Bootstrap the Rust workspace
- [x] Status marker
- Files: `Cargo.toml`, `rust-toolchain.toml`, `rustfmt.toml`, `clippy.toml`, `crates/solar-core/Cargo.toml`, `crates/solar-core/src/lib.rs`
- Changes: Create a Cargo workspace with a `solar-core` library crate, pin the Rust toolchain, add formatting/lint configuration, and keep the crate independent of Angular, WebAssembly, browser APIs, and rendering.
- Tests: Run `cargo fmt --check`, `cargo clippy --all-targets --all-features -- -D warnings`, and `cargo test --workspace`.
- Risk: Low - establishes the build foundation without scientific behavior.

### Step 2: Define the domain contract
- [ ] Status marker
- Files: `crates/solar-core/src/lib.rs`, `crates/solar-core/src/catalog.rs`, `crates/solar-core/src/domain.rs`, `crates/solar-core/tests/`
- Changes: Define stable body identifiers, categories, metadata, optional parent relationships, scientific metadata, position vectors, and collection-based state records. Register exactly the Sun and eight planets initially.
- Tests: Verify catalog membership, stable identifiers, metadata, collection output, and serialization shape.
- Risk: Medium - this is the compatibility boundary for future bodies and later WebAssembly use.

### Step 3: Add typed validation errors
- [ ] Status marker
- Files: `crates/solar-core/src/error.rs`, `crates/solar-core/src/domain.rs`, `crates/solar-core/src/catalog.rs`, `crates/solar-core/tests/`
- Changes: Define typed errors and validation rules for non-finite values, unsupported dates, unavailable model data, and invalid body identifiers. Ensure rejected operations cannot mutate valid state.
- Tests: Verify each error category and rollback behavior.
- Risk: Medium - error semantics become part of the public API.
