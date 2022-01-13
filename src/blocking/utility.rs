//! Utility endpoints group

/// Languages for translations of API resource information.
pub mod language {
    crate::blocking::endpoint!(crate::model::utility::Language; for "language");
}
