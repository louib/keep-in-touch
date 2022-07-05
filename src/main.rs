use std::fs::File;

use clap::{AppSettings, Parser, Subcommand};
use keepass::{Database, Entry, Node};

enum SupportedFormat {
    KDBX,
    VCARD,
}
impl SupportedFormat {
    pub fn from_string(format: &str) -> Option<SupportedFormat> {
        if format.to_lowercase() == "kdbx" {
            return Some(SupportedFormat::KDBX);
        }
        if format.to_lowercase() == "vcard" {
            return Some(SupportedFormat::VCARD);
        }
        None
    }
}

/// CLI tool to translate from kdbx to vcard, and vice versa.
#[derive(Parser)]
#[clap(name = "kp2vcard")]
#[clap(version = env!("CARGO_PKG_VERSION"))]
#[clap(about = "CLI tool to translate from kdbx to vcard, and vice versa.", long_about = None)]
struct KP2VCard {
    /// The path of the file to convert.
    path: String,
    /// The format to convert to. The extension of the path will be used
    /// to detect the format if a path is provided.
    format: Option<String>,
    /// Disables the password prompt on stdout.
    #[clap(long, short)]
    no_prompt: bool,
}

fn main() -> std::process::ExitCode {
    let args = KP2VCard::parse();

    let file_path = args.path;

    let mut source_format: SupportedFormat = SupportedFormat::KDBX;
    if let Some(format) = args.format {
        source_format = match SupportedFormat::from_string(&format) {
            Some(f) => f,
            None => {
                eprintln!("Invalid format {}.", format);
                return std::process::ExitCode::FAILURE;
            }
        };
    } else {
        let file_extension = file_path.split(".").last().unwrap();
        source_format = match SupportedFormat::from_string(&file_extension) {
            Some(f) => f,
            None => {
                eprintln!(
                    "Cannot detect file format from extension {}.",
                    file_extension
                );
                return std::process::ExitCode::FAILURE;
            }
        }
    }

    match source_format {
        SupportedFormat::KDBX => {
            let password = "temp_password";

            let db =
                match Database::open(&mut File::open(&file_path).unwrap(), Some(&password), None) {
                    Ok(db) => db,
                    Err(e) => {
                        eprintln!("Could not open database at {}: {}.", file_path, e);
                        return std::process::ExitCode::FAILURE;
                    }
                };

            println!(
                "There are {} entries in this database.",
                db.root.children.len()
            );

            for entry in db.root.children {
                let entry: Entry = match entry {
                    Node::Entry(e) => e,
                    Node::Group(_) => continue,
                };
                println!("Database entry {}.", entry.get_title().unwrap());
            }
        }
        SupportedFormat::VCARD => {}
    }

    std::process::ExitCode::SUCCESS
}
