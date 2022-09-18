use std::fmt;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum PNGMEErrors {
    InvalidChunkType([u8; 4]),
}

impl fmt::Display for PNGMEErrors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidChunkType(value) => {
                write!(f, "Invalid chunk type {:02x?}", value)
            }
        }
    }
}

impl std::error::Error for PNGMEErrors {}
