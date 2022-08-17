use std::{
  ffi::NulError,
  fmt::{Display, Formatter},
  io,
};

use jsc_sys::JSValueRef;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum JscError {
  #[error("Convert to `CString` failed")]
  NulError,
  #[error("{0}")]
  JSCException(JSCException),
  #[error("{0}")]
  IO(io::Error),
}

impl From<NulError> for JscError {
  fn from(_: NulError) -> Self {
    Self::NulError
  }
}

impl From<io::Error> for JscError {
  fn from(err: io::Error) -> Self {
    Self::IO(err)
  }
}

impl From<JSValueRef> for JscError {
  fn from(err: JSValueRef) -> Self {
    Self::JSCException(JSCException { exception: err })
  }
}

#[derive(Debug)]
pub struct JSCException {
  pub exception: JSValueRef,
}

impl Display for JSCException {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "JSCException")
  }
}
