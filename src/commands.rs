use clap::Subcommand;
use std::convert::TryFrom;
use std::fs;
use std::str::FromStr;

use crate::chunk::Chunk;
use crate::chunk_type::ChunkType;
use crate::png::Png;
use crate::{Error, Result};

/// Encodes a message into a PNG file and saves the result
pub fn encode(
    file_path: String,
    chunk_type: String,
    message: String,
    output: Option<String>,
) -> Result<()> {
    todo!()
}

/// Searches for a message hidden in a PNG file and prints the message if one is found
pub fn decode(file_path: String, chunk_type: String) -> Result<()> {
    todo!()
}

/// Removes a chunk from a PNG file and saves the result
pub fn remove(file_path: String, chunk_type: String) -> Result<()> {
    todo!()
}

/// Prints all of the chunks in a PNG file
pub fn print_chunks(file_path: String) -> Result<()> {
    todo!()
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    // Encode message in PNG
    Encode {
        file_path: String,
        chunk_type: String,
        message: String,
        output: Option<String>,
    },
    // Decode message from PNG
    Decode {
        file_path: String,
        chunk_type: String,
    },
    // Remove message from PNG
    Remove {
        file_path: String,
        chunk_type: String,
    },
    // Print messages in PNG
    Print {
        file_path: String,
    },
}

impl Commands {
    pub fn run(self) -> Result<()> {
        match self {
            Commands::Encode {
                file_path,
                chunk_type,
                message,
                output,
            } => encode(file_path, chunk_type, message, output),
            Commands::Decode {
                file_path,
                chunk_type,
            } => decode(file_path, chunk_type),
            Commands::Remove {
                file_path,
                chunk_type,
            } => remove(file_path, chunk_type),
            Commands::Print { file_path } => print_chunks(file_path),
        }
    }
}
