use super::*;

use std::{ffi::OsString, fmt, option::NoneError};

#[derive(Clone, Debug, Fail, PartialEq, PartialOrd)]
#[allow(pub_enum_variant_names)]
pub enum Error {
    InvalidUtf8Arg(OsString),
    NoneError,
    NoRolls,
    InvalidRoll(u8),
    InvalidFrame(usize, Vec<u8>),
    TooManyRolls,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match *self {
            Error::InvalidUtf8Arg(ref os_string) => format!("{}: {:?}", MSG_ERR_INVALID_UTF8_ARG, os_string),
            Error::NoneError => MSG_ERR_NONE_ERROR.to_string(),
            Error::NoRolls => MSG_ERR_NO_ROLLS.to_string(),
            Error::InvalidRoll(v) => format!("{}: {}", MSG_ERR_INVALID_ROLL, v),
            Error::InvalidFrame(ref i, ref v) => format!("{}: [{}]: {:?}", MSG_ERR_INVALID_FRAME, i, v),
            Error::TooManyRolls => format!("{}", MSG_ERR_TOO_MANY_ROLLS),
        })
    }
}

impl From<OsString> for Error {
    fn from(err: OsString) -> Self {
        Error::InvalidUtf8Arg(err)
    }
}

impl From<NoneError> for Error {
    fn from(_: NoneError) -> Self {
        Error::NoneError
    }
}
