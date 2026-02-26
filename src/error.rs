use std::{
    fmt::Display,
    fmt::Formatter,
    error::Error,
};

#[derive(Debug)]
pub enum GlassError {
    PollingError,
    Other(Box<dyn Error + 'static>),
}

impl Display for GlassError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> { 
        match self {
            Self::PollingError => write!(f, "Error: Fatal Error while Polling the House directory."),
            Self::Other(err) => write!(f, "{}", err),
            unimplemented => { write!(f, "Unimplemented Error Message for {:?}", unimplemented) }
        }
    }
}
impl Error for GlassError {}

impl From<Box<dyn Error + 'static>> for GlassError {
    fn from(value: Box<dyn Error + 'static>) -> Self { todo!() }
}
