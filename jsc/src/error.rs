use std::ffi::NulError;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum JscError {
  #[error("Convert to `CString` failed")]
  NulError,
  #[error("Create ContextGroup failed")]
  CreateContextGroupError,
  #[error("Create GlobalContext failed")]
  CreateGlobalContextError,
  #[error("Create Class from ClassDefinition failed")]
  CreateClassError,
}

impl From<NulError> for JscError {
  fn from(_: NulError) -> Self {
    Self::NulError
  }
}
