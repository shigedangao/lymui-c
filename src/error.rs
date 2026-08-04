use lymui::error::Error as LymuiError;
use std::borrow::Cow;

#[derive(Debug)]
pub enum Error<'a> {
    UnsupportedConversion(&'a str),
    DeserializeInputColor(&'a str),
    LymuiLib(Cow<'a, str>),
}

impl std::error::Error for Error<'_> {}

impl std::fmt::Display for Error<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnsupportedConversion(msg) => write!(f, "Unsupported conversion: {}", msg),
            Self::DeserializeInputColor(msg) => {
                write!(f, "The color passed as an input could not be read: {}", msg)
            }
            Self::LymuiLib(msg) => write!(f, "{}", msg),
        }
    }
}

impl From<LymuiError> for Error<'_> {
    fn from(err: LymuiError) -> Self {
        Self::LymuiLib(Cow::from(format!(
            "Underlying library error due to: {}",
            err
        )))
    }
}
