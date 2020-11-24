#[derive(Debug)]
pub enum Error {
    XmlError(xml::reader::Error),
    UnexpectedElement,
    MissingVersionAttr,
    InvalidVersionFormat,
    InvalidOutlineType,
    InvalidBitrateFormat,
    InvalidReliabilityFormat,
}

impl std::error::Error for Error {}

impl From<xml::reader::Error> for Error {
    fn from(error: xml::reader::Error) -> Self {
        Error::XmlError(error)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::XmlError(e) => write!(f, "{}", e),
            Error::UnexpectedElement => write!(f, "Unexpected element"),
            Error::MissingVersionAttr => write!(f, "Missing version attribute"),
            Error::InvalidVersionFormat => write!(f, "Invalid version format"),
            Error::InvalidOutlineType => write!(f, "Invalid outline type"),
            Error::InvalidBitrateFormat => write!(f, "Invalid bitrate format"),
            Error::InvalidReliabilityFormat => write!(f, "Invalid reliability format"),
        }
    }
}
