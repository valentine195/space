# Copilot Instructions

## Project Purpose

This project is a web-based space simulation. The initial product is a deterministic Rust simulation core for the Sun and eight planets, with positions validated against an authoritative ephemeris. The simulation is intended to expand later to moons, dwarf planets, asteroids, comets, and other bodies.

The current implementation scope is the Rust simulation layer. Do not add Angular, WebAssembly, browser, rendering, or deployment code unless the user explicitly starts that phase.

## Architecture

- Keep the simulation core independent of Angular, WebAssembly, browser APIs, and rendering.
- Treat Rust as the source of truth for simulation state, time, validation, body metadata, position calculations, errors, and serialization.
- Preserve a provider boundary so an ephemeris-backed model can later coexist with or be replaced by a true n-body model.
- Use collection-based body catalogs and state records. Do not encode the initial nine bodies as fixed fields that would require schema changes for future bodies.
- Use stable body identifiers, categories, display metadata, and optional parent-body relationships.

## Public Contracts

- Preserve stable body identifiers, coordinate semantics, units, time scale, model metadata, serialized fields, and typed error behavior.
- Validate inputs before mutating simulation state. Failed operations must leave the previous valid state unchanged.
- Keep outputs deterministic for identical model data and time inputs.
- Make numerical and serialization behavior explicit and testable.
- Treat public API changes as contract changes: update tests and the relevant QRSPI artifacts before implementation.

## Scientific Conventions

The approved baseline contract uses:

- Heliocentric Cartesian positions
- J2000 ecliptic coordinates
- Astronomical units for public distances
- Julian Date in a documented dynamical time scale, initially JD(TDB)
- Positions for the Sun and eight planets in the first slice

Do not silently change frames, units, time scales, supported date ranges, Sun-position semantics, ephemeris provenance, or accuracy metrics. Document any approved change in `qrspi/solar-system-simulation/spec.md` and the scientific-contract documentation.

The accuracy target is measurable agreement with a named reference ephemeris. Do not describe it as physical truth. Use documented validation epochs and explicit absolute and relative error metrics; a relative-only metric is insufficient near zero-distance configurations.

## QRSPI Workflow

Use the QRSPI artifacts as the planning and scope authority:

- Request: `qrspi/solar-system-simulation/request.md`
- Research questions: `qrspi/solar-system-simulation/queries.md`
- Research findings: `qrspi/solar-system-simulation/research.md`
- Behavioral contract: `qrspi/solar-system-simulation/spec.md`
- Implementation overview: `qrspi/solar-system-simulation/plan.md`
- Phase details: `qrspi/solar-system-simulation/plan-phase-N.md`
- Resumable workflow state: `qrspi/solar-system-simulation/state.json`

Follow the active phase and step in `state.json`. Do not silently skip, reorder, broaden, or rewrite plan steps. If implementation conflicts with the spec or plan, stop and record the blocker or decision in the QRSPI artifacts.

## Validation Commands

Run these commands from the repository root:

```sh
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --workspace
```

Keep tests close to the behavior they protect. Changes to scientific calculations require deterministic fixtures, reference comparisons, supported-range checks, finite-value checks, and regression coverage.
