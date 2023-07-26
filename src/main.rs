use std::fs::File;
use std::io;
use std::io::Write;

use anyhow::Result;

use clap::{arg, Command, Parser};
use keepass::{
    db::{Entry, Node, Value},
    Database, DatabaseKey,
};

pub const NAME_TAG_NAME: &str = "Title";
pub const NICKNAME_TAG_NAME: &str = "Nickname";
pub const PHONE_NUMBER_TAG_NAME: &str = "PhoneNumber";
pub const ADDRESS_TAG_NAME: &str = "Address";
pub const EMAIL_TAG_NAME: &str = "Email";
pub const MATRIX_ID_TAG_NAME: &str = "MatrixID";
pub const BIRTH_DATE_TAG_NAME: &str = "BirthDate";
pub const NOTES_TAG_NAME: &str = "Notes";

/// Contact manager based on the KDBX4 encrypted database format
#[derive(Parser)]
#[clap(name = "keep-in-touch")]
#[clap(version = env!("CARGO_PKG_VERSION"))]
#[clap(about = "Contact manager based on the KDBX4 encrypted database format", long_about = None)]
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

    let mut database_file = File::open(&database_path)?;

    let password = rpassword::prompt_password("Password (or blank for none): ")
        .expect("Could not read password from TTY");

    // TODO support keyfile
    // TODO support yubikey
    //
    let mut db = Database::open(
        &mut database_file,
        DatabaseKey::new().with_password(&password),
    )?;
    println!("Enter '?' to print the list of available commands.");

    let stdin = io::stdin();

    loop {
        let mut buffer = String::new();

        print!("> ");
        io::stdout().flush()?;
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
                let command = Command::new("")
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
                        e.print()?;
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
                let command = Command::new("")
                    .no_binary_name(true)
                    .arg(arg!(<term> "term to search for"));
                let parsing_result = command.clone().try_get_matches_from(command_args);
                match parsing_result {
                    Ok(command_args) => {
                        search_entries(
                            &db.root.children,
                            command_args.get_one::<String>("term").unwrap(),
                        );
                    }
                    Err(e) => {
                        e.print()?;
                    }
                }
            }
            "add" => {
                let command = Command::new("")
                    .no_binary_name(true)
                    .arg(arg!(<name> "name of the new contact"));
                let parsing_result = command.clone().try_get_matches_from(command_args);
                match parsing_result {
                    Ok(command_args) => {
                        let name = command_args.get_one::<String>("name").unwrap();
                        let mut new_entry = Entry::new();
                        new_entry.fields.insert(
                            NAME_TAG_NAME.to_string(),
                            // FIXME should new values be protected by default?
                            Value::Unprotected(name.to_string()),
                        );
                        new_entry.update_history();
                        db.root.children.push(Node::Entry(new_entry));
                        let mut database_file = File::options().write(true).open(&database_path)?;
                        db.save(
                            &mut database_file,
                            DatabaseKey::new().with_password(&password),
                        )?;
                        print!("Database was saved.");
                    }
                    Err(e) => {
                        e.print()?;
                    }
                }
            }
            "edit" => {
                let command = Command::new("")
                    .no_binary_name(true)
                    .arg(arg!(<uuid> "uuid of the contact to edit"))
                    .arg(arg!(b: -b --birthdate <date> "birth date of the contact"))
                    .arg(arg!(a: -a --address <address> "address of the contact"))
                    .arg(arg!(m: -m --matrix <matrix_id> "matrix id of the contact"))
                    .arg(arg!(n: -n --nickname <nickname> "nickname of the contact"))
                    .arg(arg!(p: -p --phone <phone> "phone number of the contact"))
                    .arg(arg!(t: -t --tags <tags> "tags associated with the contact"))
                    .arg(arg!(e: -e --email <email> "email address of the contact"));
                let parsing_result = command.clone().try_get_matches_from(command_args);
                match parsing_result {
                    Ok(command_args) => {
                        let uuid = command_args.get_one::<String>("uuid").unwrap();
                        let entry = get_entry_by_uuid(&mut db.root.children, uuid)
                            .expect(format!("Could not find entry with uuid {}", uuid).as_ref());

                        if let Some(birth_date) = command_args.get_one::<String>("b") {
                            // TODO validate the date format.
                            entry.fields.insert(
                                BIRTH_DATE_TAG_NAME.to_string(),
                                Value::Unprotected(birth_date.to_string()),
                            );
                        }

                        if let Some(address) = command_args.get_one::<String>("a") {
                            // TODO validate the address format.
                            entry.fields.insert(
                                ADDRESS_TAG_NAME.to_string(),
                                Value::Unprotected(address.to_string()),
                            );
                        }

                        // TODO we should support adding multiple email addresses!
                        if let Some(email) = command_args.get_one::<String>("e") {
                            // TODO validate the email address format.
                            entry.fields.insert(
                                EMAIL_TAG_NAME.to_string(),
                                Value::Unprotected(email.to_string()),
                            );
                        }

                        // TODO we should support adding multiple phone numbers!
                        if let Some(phone_number) = command_args.get_one::<String>("p") {
                            // TODO validate the phone number format.
                            entry.fields.insert(
                                PHONE_NUMBER_TAG_NAME.to_string(),
                                Value::Unprotected(phone_number.to_string()),
                            );
                        }

                        if let Some(matrix_id) = command_args.get_one::<String>("m") {
                            // TODO validate the matrix id format.
                            entry.fields.insert(
                                MATRIX_ID_TAG_NAME.to_string(),
                                Value::Unprotected(matrix_id.to_string()),
                            );
                        }

                        if let Some(nickname) = command_args.get_one::<String>("n") {
                            entry.fields.insert(
                                NICKNAME_TAG_NAME.to_string(),
                                Value::Unprotected(nickname.to_string()),
                            );
                        }

                        if let Some(tags) = command_args.get_one::<String>("t") {
                            let mut new_tags: Vec<String> = vec![];
                            for tag in tags.split(",") {
                                new_tags.push(tag.to_string());
                            }
                            entry.tags = new_tags;
                        }

                        if entry.update_history() {
                            println!("The entry was modified. Saving the database.");
                            let mut database_file =
                                File::options().write(true).open(&database_path)?;
                            db.save(
                                &mut database_file,
                                DatabaseKey::new().with_password(&password),
                            )?;
                        } else {
                            println!("The entry was not modified.");
                        }
                    }
                    Err(e) => {
                        e.print()?;
                    }
                }
            }
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

fn get_entry_by_uuid<'a>(nodes: &'a mut Vec<Node>, entry_uuid: &str) -> Option<&'a mut Entry> {
    for node in nodes {
        match node {
            Node::Group(group) => {
                if let Some(entry) = get_entry_by_uuid(&mut group.children, entry_uuid) {
                    return Some(entry);
                }
            }
            Node::Entry(entry) => {
                if entry.uuid.to_string() == entry_uuid {
                    return Some(entry);
                }
            }
        }
    }
    None
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
                if let Some(nickname) = entry.get(NICKNAME_TAG_NAME) {
                    if nickname.contains(search_term) {
                        println!("{} {}", entry.get_uuid(), nickname);
                    }
                }
            }
        }
    }
}

fn display_entries(nodes: &Vec<Node>, tag_option: Option<String>) {
    for node in nodes {
        match node {
            Node::Group(group) => display_entries(&group.children, tag_option.clone()),
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
                if entry.get_uuid().to_string() == uuid {
                    println!("UUID: {}", entry.get_uuid());
                    println!(
                        "Last Modification Time: {}",
                        entry.times.get_last_modification().unwrap()
                    );
                    println!("Name: {}", entry.get(NAME_TAG_NAME).unwrap());
                    if let Some(nickname) = entry.get(NICKNAME_TAG_NAME) {
                        println!("{}: {}", NICKNAME_TAG_NAME, nickname);
                    }
                    if let Some(phone_number) = entry.get(PHONE_NUMBER_TAG_NAME) {
                        println!("{}: {}", PHONE_NUMBER_TAG_NAME, phone_number);
                    }
                    if let Some(address) = entry.get(ADDRESS_TAG_NAME) {
                        println!("{}: {}", ADDRESS_TAG_NAME, address);
                    }
                    if let Some(email) = entry.get(EMAIL_TAG_NAME) {
                        println!("{}: {}", EMAIL_TAG_NAME, email);
                    }
                    if let Some(matrix_id) = entry.get(MATRIX_ID_TAG_NAME) {
                        println!("{}: {}", MATRIX_ID_TAG_NAME, matrix_id);
                    }
                    if let Some(birth_date) = entry.get(BIRTH_DATE_TAG_NAME) {
                        println!("{}: {}", BIRTH_DATE_TAG_NAME, birth_date);
                    }
                    if !entry.tags.is_empty() {
                        println!("Tags: {}", entry.tags.join(","));
                    }
                    if let Some(notes) = entry.get(NOTES_TAG_NAME) {
                        println!("--- {} ---", NOTES_TAG_NAME);
                        println!("{}", notes);
                        println!("----------");
                    }
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
