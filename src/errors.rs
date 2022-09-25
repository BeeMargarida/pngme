use std::fmt;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum PNGMEErrors {
    InvalidChunkType([u8; 4]),
    InvalidChunkSize(),
    InvalidCrc(u32, u32)
}

impl fmt::Display for PNGMEErrors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidChunkType(value) => {
                write!(f, "Invalid chunk type {:02x?}", value)
            }
            Self::InvalidChunkSize() => {
                write!(f, "Invalid chunk size")
            }
            Self::InvalidCrc(actual, expected) => {
                write!(f, "Invalid Crc {}, actual is {}", expected, actual)
            }
        }
    }
}

impl std::error::Error for PNGMEErrors {}
