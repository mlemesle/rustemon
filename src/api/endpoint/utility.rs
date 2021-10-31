//! Utility endpoints group

/// Languages for translations of API resource information.
pub mod language {
    crate::api::endpoint::endpoint::endpoint!(crate::api::model::utility::Language; for "language");
}
