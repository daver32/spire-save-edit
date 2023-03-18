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

fn main() {
    let args = CliArgs::parse();

    match args.command {
        Command::ToJson => save_to_json(&args),
        Command::FromJson => json_to_save(&args),
    };
}

fn save_to_json(args: &CliArgs) {
    let base64_data = match std::fs::read_to_string(&args.in_path) {
        Ok(data) => data,
        Err(err) => {
            println!("Failed to read input file \"{}\" ({})", &args.in_path, err);
            return;
        },
    };

    let mut data = match general_purpose::STANDARD.decode(base64_data) {
        Ok(data) => data,
        Err(err) => {
            println!("Failed to decode base64 ({})", err);
            return;
        },
    };

    encode_or_decode(data.as_mut_slice(), KEY.as_bytes());

    if let Err(err) = std::fs::write(&args.out_path, &data) {
        println!("Failed to write to output file \"{}\", ({})", &args.out_path, err);
    }
}

fn json_to_save(args: &CliArgs) {
    let mut input_data = match std::fs::read(&args.in_path) {
        Ok(bytes) => bytes,
        Err(err) => {
            println!("Failed to read input file \"{}\" ({})", &args.in_path, err);
            return;
        },
    };

    encode_or_decode(input_data.as_mut_slice(), KEY.as_bytes());

    let base64_data = general_purpose::STANDARD.encode(input_data);

    if let Err(err) = std::fs::write(&args.out_path, base64_data) {
        println!("Failed to write to output file \"{}\", ({})", &args.out_path, err);
    }
}

fn encode_or_decode(data: &mut [u8], key: &[u8]) {
    for i in 0..data.len() {
        data[i] ^= key[i % key.len()];
    }
}
