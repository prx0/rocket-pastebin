use std::{fmt};
use std::borrow::Cow;
use rocket::request::FromParam;
use uuid::Uuid;

/// A _probably_ unique paste ID.
pub struct PasteId<'a>(Cow<'a, str>);

impl<'a> PasteId<'a> {
    /// Generate a _probably_ unique ID with `size` characters. For readability,
    /// the characters used are from the sets [0-9], [A-Z], [a-z]. The
    /// probability of a collision depends on the value of `size` and the number
    /// of IDs generated thus far.
    pub fn new() -> PasteId<'static> {
        let id = format!("{}", Uuid::new_v4());
        PasteId(Cow::Owned(id))
    }
}

impl<'a> fmt::Display for PasteId<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<'a> FromParam<'a> for PasteId<'a> {
    type Error = &'a str;

    fn from_param(param: &'a str) -> Result<Self, Self::Error> {
        match Uuid::parse_str(param) {
            Ok(id) => Ok(PasteId(format!("{}", id).into())),
            Err(e) => Err(param)
        }
    }
}