use std::fs::File;
use std::io;
use std::io::Write;

use anyhow::Result;

use clap::{arg, Command, Parser, Subcommand};
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

#[derive(Parser, Default)]
struct Show {
    /// The UUID of the entry to show
    uuid: String,
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
    println!("Enter '?' to print the list of available commands.");

    let stdin = io::stdin();
    let stdout = io::stdout();

    while true {
        let mut buffer = String::new();

        print!("> ");
        io::stdout().flush();
        stdin.read_line(&mut buffer)?;

        let line: String = buffer.replace(&"\n", &"");
        let line: &str = line.trim().as_ref();

        let args = shellwords::split(line)?;

        if args.is_empty() {
            continue;
        }

        let command_name = &args[0];
        let command_args = &args[1..];

        match command_name.as_ref() {
            "ls" => {
                let mut command = Command::new("")
                    .no_binary_name(true)
                    .arg(arg!(t: -t --tag <TAG> "list entries with a specific tag"));
                let parsing_result = command.clone().try_get_matches_from(command_args);
                match parsing_result {
                    Ok(command_args) => {
                        display_entries(
                            &db.root.children,
                            command_args.get_one::<String>("t").cloned(),
                        );
                    }
                    Err(e) => {
                        e.print();
                    }
                }
            }
            "show" => {
                if command_args.len() != 1 {
                    println!("Invalid number of arguments.")
                }
                let entry_uuid = command_args[0].clone();
                let found = show_entry(&db.root.children, &entry_uuid);
                if !found {
                    println!("Could not find entry {}", entry_uuid);
                }
            }
            "search" => {
                let mut command = Command::new("")
                    .no_binary_name(true)
                    .arg(arg!(<TERM> "term to search for"));
                let parsing_result = command.clone().try_get_matches_from(command_args);
                match parsing_result {
                    Ok(command_args) => {
                        search_entries(
                            &db.root.children,
                            command_args.get_one::<String>("TERM").unwrap(),
                        );
                    }
                    Err(e) => {
                        e.print();
                    }
                }
            }
            "add" => {}
            "edit" => {}
            "help" => {}
            "?" => {
                print_available_commands();
            }
            "exit" => {
                break;
            }
            _ => {
                println!("Invalid command {}", command_name);
            }
        }
        println!();
    }

    Ok(std::process::ExitCode::SUCCESS)
}

fn search_entries(nodes: &Vec<Node>, search_term: &str) {
    for node in nodes {
        match node {
            Node::Group(group) => {
                search_entries(&group.children, search_term);
            }
            Node::Entry(entry) => {
                if let Some(title) = entry.get_title() {
                    if title.contains(search_term) {
                        println!("{} {}", entry.get_uuid(), title);
                    }
                }
            }
        }
    }
}

fn display_entries(nodes: &Vec<Node>, tag_option: Option<String>) {
    for node in nodes {
        match node {
            Node::Group(group) => {}
            Node::Entry(entry) => {
                if let Some(tag) = &tag_option {
                    if entry.tags.contains(&tag) {
                        println!("{} {}", entry.get_uuid(), entry.get_title().unwrap());
                    }
                } else {
                    println!("{} {}", entry.get_uuid(), entry.get_title().unwrap());
                }
            }
        }
    }
}

fn show_entry(nodes: &Vec<Node>, uuid: &str) -> bool {
    for node in nodes {
        match node {
            Node::Group(group) => {
                let found = show_entry(&group.children, uuid);
                if found {
                    return true;
                }
            }
            Node::Entry(entry) => {
                if entry.get_uuid() == uuid {
                    println!("{} {}", entry.get_uuid(), entry.get_title().unwrap());
                    return true;
                }
            }
        }
    }
    false
}

fn print_available_commands() {
    println!("ls - List all the contacts");
    println!("search - Search for a contact");
    println!("add - Add a new contact");
    println!("show - Show a contact's information");
    println!("edit - Edit a contact");
    println!("help - Display the help for a command");
    println!("? - Print the list of available commands");
    println!("exit - Exit the application");
}
