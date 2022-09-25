use crc::{Crc, CRC_32_ISO_HDLC};
use std::fmt::Display;

use crate::chunk_type::ChunkType;
use crate::errors::PNGMEErrors::{InvalidChunkSize, InvalidChunkType, InvalidCrc};
use crate::errors::{Error, Result};

pub const CASTAGNOLI: Crc<u32> = Crc::<u32>::new(&CRC_32_ISO_HDLC);

pub struct Chunk {
    chunk_type: ChunkType,
    data: Vec<u8>,
}

impl Chunk {
    const LENGTH_BYTES: usize = 4;
    const CHUNK_TYPE_BYTES: usize = 4;
    const CRC_BYTES: usize = 4;

    const TOTAL_BYTES: usize = Chunk::LENGTH_BYTES + Chunk::CHUNK_TYPE_BYTES + Chunk::CRC_BYTES;

    fn new(chunk_type: ChunkType, data: Vec<u8>) -> Chunk {
        Chunk {
            chunk_type: chunk_type,
            data: data,
        }
    }

    fn length(&self) -> u32 {
        self.data.len() as u32
    }

    fn chunk_type(&self) -> &ChunkType {
        &self.chunk_type
    }

    fn data(&self) -> &[u8] {
        &self.data
    }

    fn crc(&self) -> u32 {
        let joined: Vec<u8> = self
            .chunk_type
            .bytes()
            .iter()
            .chain(self.data.iter())
            .copied()
            .collect();
        CASTAGNOLI.checksum(&joined)
    }

    fn data_as_string(&self) -> Result<String> {
        let string = std::str::from_utf8(&self.data).unwrap();
        Ok(string.to_string())
    }

    fn as_bytes(&self) -> Vec<u8> {
        self.length()
            .to_be_bytes()
            .iter()
            .chain(self.length().to_be_bytes().iter())
            .chain(self.chunk_type.bytes().iter())
            .chain(self.data.iter())
            .chain(self.crc().to_be_bytes().iter())
            .copied()
            .collect()
    }
}

impl TryFrom<&[u8]> for Chunk {
    type Error = Error;

    fn try_from(value: &[u8]) -> Result<Self> {
        if value.len() < Chunk::TOTAL_BYTES {
            return Err(InvalidChunkSize().into());
        }

        let (length_bytes, value) = value.split_at(Chunk::LENGTH_BYTES);
        let length = u32::from_be_bytes(length_bytes.try_into()?) as usize;

        let (chunk_type_bytes, value) = value.split_at(Chunk::CHUNK_TYPE_BYTES);
        let chunk_type_actual: [u8; 4] = chunk_type_bytes.try_into()?;
        let chunk_type = ChunkType::try_from(chunk_type_actual)?;

        if !chunk_type.is_valid() {
            return Err(InvalidChunkType(chunk_type.bytes()).into());
        }

        let (data_bytes, value) = value.split_at(length);

        let (crc_bytes, _v) = value.split_at(Chunk::CRC_BYTES);
        let crc = u32::from_be_bytes(crc_bytes.try_into()?);

        let chunk = Self {
            chunk_type,
            data: data_bytes.to_vec(),
        };

        if chunk.crc() != crc {
            return Err(Box::new(InvalidCrc(chunk.crc(), crc)));
        }

        Ok(chunk)
    }
}

impl Display for Chunk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string = &self.data_as_string().unwrap();
        write!(f, "{}", string)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chunk_type::ChunkType;
    use std::str::FromStr;

    fn testing_chunk() -> Chunk {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        Chunk::try_from(chunk_data.as_ref()).unwrap()
    }

    #[test]
    fn test_new_chunk() {
        let chunk_type = ChunkType::from_str("RuSt").unwrap();
        let data = "This is where your secret message will be!"
            .as_bytes()
            .to_vec();
        let chunk = Chunk::new(chunk_type, data);
        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_chunk_length() {
        let chunk = testing_chunk();
        assert_eq!(chunk.length(), 42);
    }

    #[test]
    fn test_chunk_type() {
        let chunk = testing_chunk();
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
    }

    #[test]
    fn test_chunk_string() {
        let chunk = testing_chunk();
        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");
        assert_eq!(chunk_string, expected_chunk_string);
    }

    #[test]
    fn test_chunk_crc() {
        let chunk = testing_chunk();
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_valid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref()).unwrap();

        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");

        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
        assert_eq!(chunk_string, expected_chunk_string);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_invalid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656333;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref());

        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_trait_impls() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk: Chunk = TryFrom::try_from(chunk_data.as_ref()).unwrap();

        let _chunk_string = format!("{}", chunk);
    }
}
