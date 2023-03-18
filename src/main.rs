use std::error::Error;

use base64::engine::general_purpose;
use base64::Engine;
use clap::{Parser, Subcommand};

const KEY: &str = "key";

/// A converter between Slay the Spire save format and plain JSON
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct CliArgs {
    /// Input file path
    #[arg(short, long, value_name = "FILE")]
    in_path: String,

    /// Output file path
    #[arg(short, long, value_name = "FILE")]
    out_path: String,

    /// What to do
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Convert the Slay the Spire save to JSON
    ToJson,
    /// Convert the JSON to a slay the spire save
    FromJson,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = CliArgs::parse();

    match args.command {
        Command::ToJson => save_to_json(&args),
        Command::FromJson => json_to_save(&args),
    }?;

    Ok(())
}

fn save_to_json(args: &CliArgs) -> Result<(), Box<dyn Error>> {
    let base64_data = std::fs::read_to_string(&args.in_path)?;

    let mut data = general_purpose::STANDARD.decode(base64_data)?;

    encode_or_decode(data.as_mut_slice(), KEY.as_bytes());

    std::fs::write(&args.out_path, &data)?;

    Ok(())
}

fn json_to_save(args: &CliArgs) -> Result<(), Box<dyn Error>> {
    let mut input_data = std::fs::read(&args.in_path)?;

    encode_or_decode(input_data.as_mut_slice(), KEY.as_bytes());

    let base64_data = general_purpose::STANDARD.encode(input_data);

    std::fs::write(&args.out_path, base64_data)?;

    Ok(())
}

fn encode_or_decode(data: &mut [u8], key: &[u8]) {
    for i in 0..data.len() {
        data[i] ^= key[i % key.len()];
    }
}
