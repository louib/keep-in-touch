use std::fs::File;

use anyhow::Result;

use clap::{AppSettings, Parser, Subcommand};
use keepass::{Database, Entry, Node};

/// Contact manager based on the KDBX4 encrypted database format
#[derive(Parser)]
#[clap(name = "keep-in-touch")]
#[clap(version = env!("CARGO_PKG_VERSION"))]
#[clap(about = "CLI tool to translate from kdbx to vcard, and vice versa.", long_about = None)]
struct KeepInTouch {
    /// The path of the database file.
    path: String,
    /// Disables the password prompt on stdout.
    #[clap(long, short)]
    no_prompt: bool,
}

fn main() -> Result<std::process::ExitCode> {
    let args = KeepInTouch::parse();

    let database_path = args.path;

    let mut database_data = File::open(database_path)?;

    let password = rpassword::prompt_password("Password (or blank for none): ")
        .expect("Could not read password from TTY");

    // TODO support keyfile
    // TODO support yubikey
    //
    let mut db = Database::open(&mut database_data, Some(&password), None)?;

    println!("Database was opened, {} entries.", db.root.children.len());

    Ok(std::process::ExitCode::SUCCESS)
}
