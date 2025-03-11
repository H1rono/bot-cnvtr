use http::HeaderMap;
use serde_json::{Value, value::Index};

use domain::Failure;

pub(crate) fn extract_header_value<'a>(
    headers: &'a HeaderMap,
    name: &str,
) -> Result<&'a [u8], Failure> {
    headers
        .get(name)
        .map(http::HeaderValue::as_bytes)
        .ok_or_err()
}

pub(crate) trait OptionExt {
    type Inner;
    fn ok_or_err(self) -> Result<Self::Inner, Failure>;
}

impl<T> OptionExt for Option<T> {
    type Inner = T;

    fn ok_or_err(self) -> Result<Self::Inner, Failure> {
        self.ok_or_else(|| Failure::reject_bad_request("Received unexpected payload"))
    }
}

#[allow(dead_code)]
pub(crate) trait ValueExt {
    fn get_or_err<I>(&self, index: I) -> Result<&Value, Failure>
    where
        I: Index + ToString;
    fn as_str_or_err(&self) -> Result<&str, Failure>;
    fn as_array_or_err(&self) -> Result<&Vec<Value>, Failure>;
    fn as_u64_or_err(&self) -> Result<u64, Failure>;
    fn as_i64_or_err(&self) -> Result<i64, Failure>;
    fn as_f64_or_err(&self) -> Result<f64, Failure>;
    fn as_bool_or_err(&self) -> Result<bool, Failure>;
}

impl ValueExt for Value {
    fn get_or_err<I>(&self, index: I) -> Result<&Value, Failure>
    where
        I: Index + ToString,
    {
        self.get(index)
            .ok_or_else(|| Failure::reject_bad_request("Received unexpected payload"))
    }

    fn as_str_or_err(&self) -> Result<&str, Failure> {
        self.as_str()
            .ok_or_else(|| Failure::reject_bad_request("Received unexpected payload"))
    }

    fn as_array_or_err(&self) -> Result<&Vec<Value>, Failure> {
        self.as_array()
            .ok_or_else(|| Failure::reject_bad_request("Received unexpected payload"))
    }

    fn as_u64_or_err(&self) -> Result<u64, Failure> {
        self.as_u64()
            .ok_or_else(|| Failure::reject_bad_request("Received unexpected payload"))
    }

    fn as_i64_or_err(&self) -> Result<i64, Failure> {
        self.as_i64()
            .ok_or_else(|| Failure::reject_bad_request("Received unexpected payload"))
    }

    fn as_f64_or_err(&self) -> Result<f64, Failure> {
        self.as_f64()
            .ok_or_else(|| Failure::reject_bad_request("Received unexpected payload"))
    }

    fn as_bool_or_err(&self) -> Result<bool, Failure> {
        self.as_bool()
            .ok_or_else(|| Failure::reject_bad_request("Received unexpected payload"))
    }
}
