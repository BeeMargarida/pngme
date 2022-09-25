use std::fmt;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum PNGMEErrors {
    InvalidChunkType([u8; 4]),
    InvalidChunkSize(),
    InvalidCrc(u32, u32),
    InvalidHeader([u8; 8]),
    UnknownChunkType(),
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
            Self::InvalidHeader(header) => {
                write!(f, "Invalid header {:02x?}", header)
            }
            Self::UnknownChunkType() => {
                write!(f, "Unknow chunk type")
            }
        }
    }
}

impl std::error::Error for PNGMEErrors {}
