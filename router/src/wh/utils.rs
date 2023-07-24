use serde_json::{value::Index, Value};

use crate::{Error, Result};

pub(crate) trait ValueExt {
    fn get_or_err<I>(&self, index: I) -> Result<&Value>
    where
        I: Index;

    fn as_str_or_err(&self) -> Result<&str>;
}

impl ValueExt for Value {
    fn get_or_err<I>(&self, index: I) -> Result<&Value>
    where
        I: Index,
    {
        self.get(index).ok_or(Error::BadRequest)
    }

    fn as_str_or_err(&self) -> Result<&str> {
        self.as_str().ok_or(Error::BadRequest)
    }
}
