use clap::{AppSettings, Parser, Subcommand};

enum SupportedFormat {
    KDBX,
    VCARD,
}
impl SupportedFormat {
    pub fn from_string(format: &str) -> Option<SupportedFormat> {
        if (format == "kdbx") {
            return Some(SupportedFormat::KDBX);
        }
        if (format == "vcard") {
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
        SupportedFormat::KDBX => {}
        SupportedFormat::VCARD => {}
    }

    std::process::ExitCode::SUCCESS
}
