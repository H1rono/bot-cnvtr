use serde_json::{value::Index, Value};

use crate::{Error, Result};

pub trait ValueExt {
    fn get_or_err<I>(&self, index: I) -> Result<&Value>
    where
        I: Index;
}

impl ValueExt for Value {
    fn get_or_err<I>(&self, index: I) -> Result<&Value>
    where
        I: Index,
    {
        self.get(index).ok_or(Error::BadRequest)
    }
}
