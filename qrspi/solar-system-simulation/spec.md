# Solar System Simulation Rust Core

## Behavioral Change

### Before

The workspace has no simulation behavior, celestial-body data, Rust API, or web integration.

### After

The project will define a Rust simulation core for the Sun and eight planets. Given a simulation time, the core will produce the modeled state of all registered bodies, including their positions and the physical quantities required to describe those positions consistently. The initial registry contains exactly nine bodies.

The initial behavioral target is a deterministic solar-system model whose results are validated against an authoritative ephemeris. The target interpretation of “5 9s” is agreement of at least 99.999% with the reference position for every supported body and validation epoch, using a documented relative-position error metric.

The initial supported time range will be the range covered by the selected reference ephemeris and will be recorded as part of the simulation's public data contract. Requests outside that range will return a typed error rather than an extrapolated result.

## Coordinate and Time Contract

The recommended baseline contract is:

- Positions are heliocentric Cartesian vectors.
- The reference frame is J2000 ecliptic coordinates.
- Distances are expressed in astronomical units for the public simulation state.
- Simulation time is represented as Julian Date in a documented dynamical time scale.
- Each body is identified by a stable identifier for the Sun, Mercury, Venus, Earth, Mars, Jupiter, Saturn, Uranus, and Neptune.

These choices are part of the observable contract. They may be revised before implementation if the selected reference data requires a different compatible convention, but the final frame, units, and time scale must be singular and documented.

## Rust API Contract

The Rust core will provide public operations equivalent to:

1. Create or initialize a simulation using its documented model and supported time range.
2. Set or advance the simulation time, including forward and reverse movement.
3. Pause time advancement without changing the current state.
4. Query the current state of one body or all nine bodies.
5. Query the catalog of registered bodies and their metadata.
6. Serialize the current state into a stable, UI-neutral representation for the future WebAssembly boundary.

An input representing a valid epoch, such as the configured reference epoch, returns one body record per registered body with stable identifiers, the requested timestamp, position vectors, units, and frame metadata. An input outside the supported range or with a non-finite timestamp returns a typed validation error and does not mutate the current state.

The API must be deterministic: identical initialization, model data, and time inputs produce identical results regardless of call order outside the documented state-changing operations.

## Data and State Behavior

- The model includes exactly the Sun and eight planets in the first slice.
- The initial body catalog includes the Sun, Mercury, Venus, Earth, Mars, Jupiter, Saturn, Uranus, and Neptune.
- Every body record has a stable identifier, category, display metadata, and optional parent-body relationship. The initial nine bodies use the same record shape as future moons, dwarf planets, asteroids, and comets.
- Body discovery and state queries operate on registered-body identifiers rather than a fixed nine-body field layout.
- Adding a supported body category must add catalog entries and model data without changing the meaning or shape of existing body records, timestamps, coordinates, units, errors, or serialization fields.
- Moons, dwarf planets, asteroids, comets, and bodies outside the solar system are not included in the first slice.
- The state includes enough metadata to prevent consumers from confusing units, frame, time scale, or model version.
- Time advancement accepts signed deltas and supports pause, forward, reverse, and large-step requests subject to the documented validity range.
- Invalid inputs leave the previous valid state unchanged.
- Numerical failures, unavailable model data, and unsupported dates are reported as typed errors.

## Integration Boundary

The first slice exposes only the Rust API. No Angular application, browser runtime, WebAssembly package, JavaScript interop, rendering surface, animation loop, or deployment behavior is added by this specification.

The Rust API's serialized state is UI-neutral so that a later WebAssembly shim can expose it without changing the simulation's scientific contract.

## Compatibility

There is no existing application behavior or public API to preserve. Future UI and WebAssembly layers must consume the Rust contract without changing the meaning of body identifiers, coordinates, units, timestamps, errors, or accuracy validation.

## Acceptance Criteria

1. A clean Rust workspace exposes the documented simulation behavior for the nine initial bodies.
2. Valid epoch queries return deterministic, finite positions for every initial body with explicit frame, unit, time-scale, and model metadata.
3. Time can be set, advanced forward, advanced backward, and paused without violating state invariants.
4. Invalid timestamps and unsupported dates return typed errors and preserve the previous valid state.
5. Automated validation compares returned positions with the selected authoritative ephemeris over the supported date range and demonstrates the agreed 99.999% accuracy target.
6. Body catalog and state serialization are collection-based and do not require a schema change when a future supported body is added.
7. A compatibility test demonstrates that a future body entry can be represented with the same catalog, state, query, and serialization contracts as the initial nine bodies.
8. Rust tests cover initialization, deterministic queries, time transitions, range validation, invalid input handling, serialization, body catalog behavior, and reference-ephemeris accuracy.
9. The first slice has no Angular, WebAssembly, browser, rendering, or deployment requirement.

## Out of Scope

- Angular UI implementation
- WebAssembly shim implementation
- Rendering or visual design
- Interactive browser controls
- Adding and modeling moons, dwarf planets, asteroids, comets, or other additional bodies
- Gravitational perturbations from excluded bodies
- Interstellar or galactic-scale simulation
- Deployment and hosting