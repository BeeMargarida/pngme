use clap::Subcommand;
use std::fs;
use std::str::FromStr;

use crate::chunk::Chunk;
use crate::chunk_type::ChunkType;
use crate::png::Png;
use crate::Result;

/// Encodes a message into a PNG file and saves the result
pub fn encode(
    file_path: String,
    chunk_type: String,
    message: String,
    output: Option<String>,
) -> Result<()> {
    let mut png: Png = Png::from_file(&file_path).unwrap();

    let chunk_type = ChunkType::from_str(chunk_type.as_str()).unwrap();
    let message = message.bytes().collect();
    let message_chunk = Chunk::new(chunk_type, message);
    png.append_chunk(message_chunk);
    println!("{}", &png.as_bytes().len());

    let output_path = match output {
        Some(output) => output,
        None => file_path,
    };
    let result = fs::write(&output_path, png.as_bytes());
    if let Err(e) = result {
        println!("{:?}", e.to_string());
    }
    Ok(())
}

/// Searches for a message hidden in a PNG file and prints the message if one is found
pub fn decode(file_path: String, chunk_type: String) -> Result<()> {
    let png: Png = Png::from_file(&file_path).unwrap();
    let result = png.chunk_by_type(chunk_type.as_str());
    match result {
        Some(message) => println!("Message: {}", message.to_string()),
        None => println!("This PNG does not contain any message."),
    }
    Ok(())
}

/// Removes a chunk from a PNG file and saves the result
pub fn remove(file_path: String, chunk_type: String) -> Result<()> {
    let mut png: Png = Png::from_file(&file_path).unwrap();
    png.remove_chunk(chunk_type.as_str());
    let result = fs::write(&file_path, png.as_bytes());
    if let Err(e) = result {
        println!("{:?}", e.to_string());
    }
    Ok(())
}

/// Prints all of the chunks in a PNG file
pub fn print_chunks(file_path: String) -> Result<()> {
    let png: Png = Png::from_file(&file_path).unwrap();
    println!("{}", png.to_string());
    Ok(())
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
