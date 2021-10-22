use std::{fmt};
use std::borrow::Cow;
use rocket::request::FromParam;
use rocket::form::FromForm;
use uuid::Uuid;

// PasteId use an UUID v4 to avoid collision and
// be unique
pub struct PasteId<'a>(Cow<'a, str>);

impl<'a> PasteId<'a> {
    // generate an UUID v4
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
            Err(_) => Err(param)
        }
    }
}

// Lang represent a GPL or DSL
pub struct Lang<'a>(Cow<'a, str>);

impl<'a> fmt::Display for Lang<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<'a> FromParam<'a> for Lang<'a> {
    type Error = &'a str;

    fn from_param(param: &'a str) -> Result<Self, Self::Error>{
        let supported_lang = [
            "c".to_string(), 
            "cpp".to_string(), 
            "csharp".to_string(), 
            "ruby".to_string(), 
            "js".to_string(),
            "xml".to_string(),
            "css".to_string(),
            "rust".to_string(),
        ].to_vec();

        match supported_lang.iter().find(|&s| *s == param) {
            Some(lang) => Ok(Lang(format!("language-{}", lang).into())),
            None => Ok(Lang("language-text".into()))
        }
    }
}

// Represent the form to send a new pastebin
#[derive(FromForm)]
pub struct SendPastebin {
    #[field(name = "content")]
    pub raw_content: String,

    #[field(name = "lang")]
    pub lang: String,
}