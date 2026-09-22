//! Deterministic, UI-independent solar-system simulation primitives.

#![forbid(unsafe_code)]

/// Returns the crate version for baseline workspace verification.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(test)]
mod tests {
    use super::VERSION;

    #[test]
    fn exposes_package_version() {
        assert_eq!(VERSION, "0.1.0");
    }
}
