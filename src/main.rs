use clap::{AppSettings, Parser, Subcommand};

/// CLI tool to translate from kdbx to vcard, and vice versa.
#[derive(Parser)]
#[clap(name = "kp2vcard")]
#[clap(version = env!("CARGO_PKG_VERSION"))]
#[clap(about = "CLI tool to translate from kdbx to vcard, and vice versa.", long_about = None)]
struct KP2VCard {
    #[clap(subcommand)]
    command: SubCommand,
}

#[derive(Subcommand)]
enum SubCommand {
    /// Formats a Flatpak manifest.
    #[clap(setting(AppSettings::ArgRequiredElseHelp))]
    Convert {
        /// The path of the file to convert.
        path: String,
        /// The format to convert to. The extension of the path will be used
        /// to detect the format if a path is provided.
        format: Option<String>,
    },
}

fn main() -> std::process::ExitCode {
    let args = KP2VCard::parse();

    match &args.command {
        SubCommand::Convert { path, format } => {
            if path.ends_with(".kdbx") {
                return std::process::ExitCode::SUCCESS;
            }

            if path.ends_with(".vcard") {
                return std::process::ExitCode::SUCCESS;
            }

            eprintln!("Could not detect file format based on path extension.");
            return std::process::ExitCode::FAILURE;
        }
    }
    std::process::ExitCode::SUCCESS
}
