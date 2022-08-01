// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Formatting date and time.
//!
//! This module is published as its own crate ([`icu_datetime`](https://docs.rs/icu_datetime/latest/icu_datetime/))
//! and as part of the [`icu`](https://docs.rs/icu/latest/icu/) crate. See the latter for more details on the ICU4X project.
//!
//! [`TypedDateTimeFormatter`] and [`DateTimeFormatter`] are the main types of the component. They accepts a set of arguments which
//! allow it to collect necessary data from the [data provider], and once instantiated, can be
//! used to quickly format any date and time provided.
//!
//! # Examples
//!
//! ```
//! use icu::calendar::Gregorian;
//! use icu::datetime::{
//!     mock::parse_gregorian_from_str, options::length, DateTimeFormatter, TypedDateTimeFormatter, DateTimeFormatterOptions,
//! };
//! use icu::locid::{Locale, locale};
//! use std::str::FromStr;
//!
//! let provider = icu_testdata::get_provider();
//!
//! // See the next code example for a more ergonomic example with .into().
//! let options = DateTimeFormatterOptions::Length(length::Bag::from_date_time_style(
//!     length::Date::Medium,
//!     length::Time::Short,
//! ));
//!
//! // You can work with a formatter that can select the calendar at runtime:
//! let locale = Locale::from_str("en-u-ca-gregory").unwrap();
//! let dtf = DateTimeFormatter::try_new_with_buffer_provider(&provider, &locale.into(), &options)
//!     .expect("Failed to create DateTimeFormatter instance.");
//!
//! // Or one that selects a calendar at compile time:
//! let typed_dtf = TypedDateTimeFormatter::<Gregorian>::try_new(&provider, &locale!("en").into(), &options)
//!     .expect("Failed to create TypedDateTimeFormatter instance.");
//!
//! let typed_date = parse_gregorian_from_str("2020-09-12T12:35:00").expect("Failed to parse date.");
//! let date = typed_date.to_any();
//!
//! let formatted_date = dtf.format(&date).expect("Formatting should succeed");
//! let typed_formatted_date = typed_dtf.format(&typed_date);
//!
//! assert_eq!(formatted_date.to_string(), "Sep 12, 2020, 12:35 PM");
//! assert_eq!(typed_formatted_date.to_string(), "Sep 12, 2020, 12:35 PM");
//! ```
//!
//! The options can be created more ergonomically using the `Into` trait to automatically
//! convert a [`options::length::Bag`] into a [`DateTimeFormatterOptions::Length`].
//!
//! ```
//! use icu::calendar::Gregorian;
//! use icu::datetime::{options::length, TypedDateTimeFormatter, DateTimeFormatterOptions};
//! # let provider = icu_testdata::get_provider();
//! # let locale = icu::locid::locale!("en");
//! let options =
//!     length::Bag::from_date_time_style(length::Date::Medium, length::Time::Short).into();
//!
//! let dtf = TypedDateTimeFormatter::<Gregorian>::try_new(&provider, &locale.into(), &options);
//! ```
//!
//! At the moment, the crate provides only options using the [`Length`] bag, but in the future,
//! we expect to add more ways to customize the output, like skeletons, and components.
//!
//! *Notice:* Rust at the moment does not have a canonical way to represent date and time. We use
//! [`DateTime`] as an example of the data necessary for ICU [`TypedDateTimeFormatter`] to work, and
//! [we hope to work with the community](https://github.com/unicode-org/icu4x/blob/main/docs/research/datetime.md)
//! to develop core date and time APIs that will work as an input for this component. [`DateTime`] additionally
//! has support for non-Gregorian calendars, which this module will eventually be able to format.
//!
//! [data provider]: icu_provider
//! [`ICU4X`]: ../icu/index.html
//! [`Length`]: options::length
//! [`DateTime`]: icu_calendar::DateTime

// https://github.com/unicode-org/icu4x/blob/main/docs/process/boilerplate.md#library-annotations
#![cfg_attr(not(any(test, feature = "std")), no_std)]
#![cfg_attr(
    not(test),
    deny(
        clippy::indexing_slicing,
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        clippy::exhaustive_structs,
        clippy::exhaustive_enums,
        // TODO(#2266): enable missing_debug_implementations,
    )
)]
#![warn(missing_docs)]

extern crate alloc;

mod calendar;
mod datetime;
mod error;
pub mod fields;
mod format;
pub mod input;
pub mod mock;
pub mod options;
#[doc(hidden)]
pub mod pattern;
pub mod provider;
pub(crate) mod raw;
#[doc(hidden)]
#[allow(clippy::exhaustive_structs, clippy::exhaustive_enums)] // private-ish module
pub mod skeleton;
#[allow(missing_docs)] // TODO(#686) - Add missing docs.
pub mod time_zone;
mod zoned_datetime;

mod any;

pub use any::{DateTimeFormatter, ZonedDateTimeFormatter};
pub use calendar::CldrCalendar;
pub use datetime::{TimeFormatter, TypedDateFormatter, TypedDateTimeFormatter};
pub use error::DateTimeFormatterError;
pub use format::datetime::FormattedDateTime;
pub use format::time_zone::FormattedTimeZone;
pub use format::zoned_datetime::FormattedZonedDateTime;
pub use options::DateTimeFormatterOptions;
pub use time_zone::TimeZoneFormatter;
pub use time_zone::TimeZoneFormatterConfig;
pub use time_zone::TimeZoneFormatterOptions;
pub use zoned_datetime::TypedZonedDateTimeFormatter;
