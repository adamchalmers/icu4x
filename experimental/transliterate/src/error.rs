// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use core::fmt::Debug;
use displaydoc::Display;
use icu_provider::DataError;

#[cfg(feature = "std")]
impl std::error::Error for TransliteratorError {}

/// A list of error outcomes for various operations in this module.
///
/// Re-exported as [`Error`](crate::Error).
#[derive(Display, Debug, Copy, Clone, PartialEq)]
#[non_exhaustive]
pub enum TransliteratorError {
    /// An error originating inside of the [data provider](icu_provider).
    #[displaydoc("{0}")]
    Data(DataError),
    /// The requested transliterator is marked as internal-only.
    InternalOnly,
}

impl From<DataError> for TransliteratorError {
    fn from(e: DataError) -> Self {
        Self::Data(e)
    }
}
