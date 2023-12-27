use serde_json::{value::Index, Value};

use crate::{Error, Result};

pub(crate) fn extract_header_value<'a, H, K, V>(headers: H, name: &str) -> Result<&'a [u8]>
where
    H: Iterator<Item = (&'a K, &'a V)>,
    K: AsRef<[u8]> + ?Sized + 'static,
    V: AsRef<[u8]> + ?Sized + 'static,
{
    use std::str::from_utf8;
    let name = name.to_lowercase();
    for (k, v) in headers {
        let Ok(key) = from_utf8(k.as_ref()) else {
            continue;
        };
        if key.to_lowercase() != name {
            continue;
        }
        return Ok(v.as_ref());
    }
    Err(Error::MissingField)
}

pub(crate) trait ValueExt {
    fn get_or_err<I>(&self, index: I) -> Result<&Value>
    where
        I: Index + ToString;
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
        I: Index + ToString,
    {
        self.get(index).ok_or(Error::MissingField)
    }

    fn as_str_or_err(&self) -> Result<&str> {
        self.as_str().ok_or(Error::WrongType)
    }

    fn as_array_or_err(&self) -> Result<&Vec<Value>> {
        self.as_array().ok_or(Error::WrongType)
    }

    fn as_u64_or_err(&self) -> Result<u64> {
        self.as_u64().ok_or(Error::WrongType)
    }

    fn as_i64_or_err(&self) -> Result<i64> {
        self.as_i64().ok_or(Error::WrongType)
    }

    fn as_f64_or_err(&self) -> Result<f64> {
        self.as_f64().ok_or(Error::WrongType)
    }

    fn as_bool_or_err(&self) -> Result<bool> {
        self.as_bool().ok_or(Error::WrongType)
    }
}
