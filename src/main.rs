use std::fs::File;
use std::io;
use std::io::Write;

use anyhow::Result;

use clap::{AppSettings, Parser, Subcommand};
use keepass::{Database, Entry, Node, NodeRef};

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

    let stdin = io::stdin();
    let stdout = io::stdout();

    while true {
        let mut buffer = String::new();

        display_menu();
        print!("Enter your choice: ");
        io::stdout().flush();
        stdin.read_line(&mut buffer)?;

        let choice: String = buffer.replace(&"\n", &"");
        let choice: &str = choice.trim().as_ref();
        match choice {
            "ls" => {
                display_all_entries(&db.root.children);
            }
            "search" => {}
            "add" => {}
            "edit" => {}
            "help" => {}
            "exit" => {
                break;
            }
            _ => {
                println!("Invalid command {}", choice);
            }
        }
        println!()
    }

    Ok(std::process::ExitCode::SUCCESS)
}

fn display_all_entries(nodes: &Vec<Node>) {
    for node in nodes {
        match node {
            Node::Group(group) => {}
            Node::Entry(entry) => {
                println!("{} {}", entry.get_uuid(), entry.get_title().unwrap());
            }
        }
    }
}

fn display_menu() {
    println!("ls - List all the contacts");
    println!("search - Search for a contact");
    println!("add - Add a new contact");
    println!("edit - Edit a contact");
    println!("help - Display the help for a command");
    println!("exit - Exit the application");
}
