#![cfg_attr(feature = "clippy", feature(plugin))] // nightly rustc required by `clippy`
#![cfg_attr(feature = "clippy", plugin(clippy))]
#![allow(match_bool)] // disable false positives
#![warn(cast_possible_truncation, cast_possible_wrap, cast_precision_loss, cast_sign_loss, empty_enum, enum_glob_use,
fallible_impl_from, filter_map, if_not_else, int_plus_one, invalid_upcast_comparisons, maybe_infinite_iter,
mem_forget, missing_debug_implementations, mut_mut, mutex_integer, nonminimal_bool, option_map_unwrap_or,
option_map_unwrap_or_else, option_map_unwrap_or_else, option_unwrap_used, /*print_stdout,*/
pub_enum_variant_names, range_plus_one, redundant_closure, result_map_unwrap_or_else, result_unwrap_used,
trivial_casts, non_camel_case_types, stutter, trivial_numeric_casts, unicode_not_nfc,
unseparated_literal_suffix, /*use_debug,*/ use_self, used_underscore_binding, unused_import_braces,
unnecessary_mut_passed, unused_qualifications, wrong_pub_self_convention)]
#![deny(overflowing_literals, unused_must_use)]
#![feature(try_trait, integer_atomics, associated_type_defaults)]

#[macro_use]
extern crate failure;

mod consts;
pub mod di;
pub mod error;
pub mod greeter;
mod hello_world_greeter;
mod universal_width_provider;
mod width_provider;

use consts::*;
pub use error::Error;
use greeter::Greeter;
use hello_world_greeter::HelloWorldGreeter;
use universal_width_provider::UniversalWidthProvider;
use width_provider::WidthProvider;

pub type Result<T> = std::result::Result<T, Error>;