use serde_json::{value::Index, Value};

use crate::{Error, Result};

pub(crate) trait ValueExt {
    fn get_or_err<I>(&self, index: I) -> Result<&Value>
    where
        I: Index;
    fn as_str_or_err(&self) -> Result<&str>;
    fn as_array_or_err(&self) -> Result<&Vec<Value>>;
    fn as_u64_or_err(&self) -> Result<u64>;
    fn as_i64_or_err(&self) -> Result<i64>;
    fn as_f64_or_err(&self) -> Result<f64>;
    fn as_bool_or_err(&self) -> Result<bool>;
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

    fn as_array_or_err(&self) -> Result<&Vec<Value>> {
        self.as_array().ok_or(Error::BadRequest)
    }

    fn as_u64_or_err(&self) -> Result<u64> {
        self.as_u64().ok_or(Error::BadRequest)
    }

    fn as_i64_or_err(&self) -> Result<i64> {
        self.as_i64().ok_or(Error::BadRequest)
    }

    fn as_f64_or_err(&self) -> Result<f64> {
        self.as_f64().ok_or(Error::BadRequest)
    }

    fn as_bool_or_err(&self) -> Result<bool> {
        self.as_bool().ok_or(Error::BadRequest)
    }
}
