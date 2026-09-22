# Feature Request

A new personal side project that is a web-based space simulation showing the real location of celestial bodies. It should start with the solar system and may expand beyond that in the future.

The core of the simulation should be written in Rust. A small WebAssembly shim should allow it to be used by an Angular UI, but the initial focus is the Rust simulation layer.

1. Initial bodies: Sun and eight planets only, or include moons and dwarf planets?
Sun and eight planets.

2. Position source: fixed orbital elements, an established ephemeris dataset, or an n-body simulation?
Could start with ephemeris dataset but ideally simulated with true orbital mechanics.

3. Accuracy target and supported date range?
5 9s.

4. Coordinate system, reference frame, and time scale?
No opinion, whatever makes the most sense. Give suggestions.

5. Rust layer scope: state and position calculations only, or also time control and serialization?
Rust layer should do everything, with the UI being display only.

6. Should the WebAssembly shim be included in this first slice, or deferred until the Rust API is stable?
First slice is just the Rust API