use crate::{DlsiteError, Result};

pub(crate) trait ToParseError<T> {
    fn to_parse_error(self, msg: &str) -> Result<T>;
}

impl<T> ToParseError<T> for Option<T> {
    fn to_parse_error(self, msg: &str) -> Result<T> {
        self.ok_or_else(|| DlsiteError::ParseError(msg.to_string()))
    }
}

impl<T, E> ToParseError<T> for std::result::Result<T, E> {
    fn to_parse_error(self, msg: &str) -> Result<T> {
        self.map_err(|_| DlsiteError::ParseError(msg.to_string()))
    }
}
