#![cfg_attr(feature="clippy", feature(plugin))] //nightly rustc required by `clippy`
#![cfg_attr(feature="clippy", plugin(clippy))]
#![feature(proc_macro, const_atomic_bool_new, galvanic_mock_integration)] //req'd by galvanic-mock, galvanic-test + galvanic-mock
#![allow(match_bool, redundant_closure /*galvanic*/, unnecessary_mut_passed /*galvanic*/)] //disable false positives
#![warn(cast_possible_truncation, cast_possible_wrap, cast_precision_loss, cast_sign_loss, empty_enum, enum_glob_use,
        fallible_impl_from, filter_map, if_not_else, int_plus_one, invalid_upcast_comparisons, maybe_infinite_iter,
        mem_forget, missing_debug_implementations, mut_mut, mutex_integer, nonminimal_bool, option_map_unwrap_or,
        option_map_unwrap_or_else, option_map_unwrap_or_else, option_unwrap_used, /*print_stdout,*/
        pub_enum_variant_names, range_plus_one, result_map_unwrap_or_else, result_unwrap_used, /*trivial_casts, //galvanic*/
        non_camel_case_types, stutter, trivial_numeric_casts, unicode_not_nfc, unseparated_literal_suffix,
        /*use_debug,*/ use_self, used_underscore_binding, unused_import_braces, /*unused_qualifications, //galvanic*/
        wrong_pub_self_convention)]
#![deny(overflowing_literals, unused_must_use)]

pub extern crate failure;
#[macro_use] extern crate failure_derive;
#[allow(useless_attribute, unused_imports)] #[macro_use] extern crate galvanic_assert;
#[allow(useless_attribute, unused_imports)] #[macro_use] extern crate galvanic_test;
extern crate galvanic_mock;

#[cfg(test)] mod unit_tests;
pub mod consts;
mod error;
pub mod concurrency_primitives;
pub mod arch;

pub use error::Error;
#[allow(unused_imports)] use failure::ResultExt;
#[allow(unused_imports)] use consts::*;
use concurrency_primitives::Singleton;
use arch::Info;

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone, PartialEq)]
pub struct App<T: Info> {
    arch_info: T,
}

impl<T: Info> App<T> {
    /// # Returns
    /// Application instance upon successful initialization.
    ///
    /// /// # Errors
    /// Returns an error in the event that any unhandled errors arise during initialization.  Prevents application from
    /// being instantiated more than once.
    /// Rather than returning general, type-erased `Box<std::error::Error>`s or downcastable `Fail::Error`s, `run()`
    /// returns a strongly typed `self::error::Error` which can be exhaustively matched or type-erased (coerced into a
    /// `Fail::Error`) at the caller's discretion.
    ///
    /// # Remarks
    /// Employs constructor dependency injection for loose coupling of the singleton management instance (this enables
    /// downgrading or upgrading the concurrency primitive to local thread or across multiple distinct systems, for
    /// example, without changing App's implementation at all) and the architecture's address width (again, for the
    /// benefits gained from loose coupling).  Notice that both dependencies can now be mocked, enabling for thorough
    /// unit testing.
    pub fn new<S: Singleton>(app_state: &S, arch_info: T) -> Result<Self> {
        match app_state.already_instantiated() {
            true => Err(Error::SingletonViolation),
            false => Ok(Self { arch_info }),
        }
    }

    /// # Returns
    /// Result of application execution.
    ///
    /// # Errors
    /// Returns an error in the event that any unhandled errors arise during execution.
    ///
    /// # Remarks
    /// This method is the library's primary entry point.
    pub fn run(&self) -> Result<String> {
        Ok(format!("Hello, {}-bit world!", self.arch_info.width()))
    }
}

