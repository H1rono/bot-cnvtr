use http::HeaderMap;
use serde_json::{value::Index, Value};

use domain::Error;

pub(crate) fn extract_header_value<'a>(
    headers: &'a HeaderMap,
    name: &str,
) -> Result<&'a [u8], Error> {
    headers
        .get(name)
        .map(http::HeaderValue::as_bytes)
        .ok_or_err()
}

pub(crate) trait OptionExt {
    type Inner;
    fn ok_or_err(self) -> Result<Self::Inner, Error>;
}

impl<T> OptionExt for Option<T> {
    type Inner = T;

    fn ok_or_err(self) -> Result<Self::Inner, Error> {
        self.ok_or(Error::BadRequest)
    }
}

#[allow(dead_code)]
pub(crate) trait ValueExt {
    fn get_or_err<I>(&self, index: I) -> Result<&Value, Error>
    where
        I: Index + ToString;
    fn as_str_or_err(&self) -> Result<&str, Error>;
    fn as_array_or_err(&self) -> Result<&Vec<Value>, Error>;
    fn as_u64_or_err(&self) -> Result<u64, Error>;
    fn as_i64_or_err(&self) -> Result<i64, Error>;
    fn as_f64_or_err(&self) -> Result<f64, Error>;
    fn as_bool_or_err(&self) -> Result<bool, Error>;
}

impl ValueExt for Value {
    fn get_or_err<I>(&self, index: I) -> Result<&Value, Error>
    where
        I: Index + ToString,
    {
        self.get(index).ok_or(Error::BadRequest)
    }

    fn as_str_or_err(&self) -> Result<&str, Error> {
        self.as_str().ok_or(Error::BadRequest)
    }

    fn as_array_or_err(&self) -> Result<&Vec<Value>, Error> {
        self.as_array().ok_or(Error::BadRequest)
    }

    fn as_u64_or_err(&self) -> Result<u64, Error> {
        self.as_u64().ok_or(Error::BadRequest)
    }

    fn as_i64_or_err(&self) -> Result<i64, Error> {
        self.as_i64().ok_or(Error::BadRequest)
    }

    fn as_f64_or_err(&self) -> Result<f64, Error> {
        self.as_f64().ok_or(Error::BadRequest)
    }

    fn as_bool_or_err(&self) -> Result<bool, Error> {
        self.as_bool().ok_or(Error::BadRequest)
    }
}
