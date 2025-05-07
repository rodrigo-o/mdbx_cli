use clap::Parser;
use std::path::PathBuf;
use libmdbx::{Database, DatabaseOptions, WriteMap};
use rustyline::Editor;
use mdbx_cli::commands;

/// Interactive MDBX CLI Tool
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Path to the MDBX database file
    #[arg(short, long, value_name = "FILE")]
    db_path: PathBuf,
}

pub fn execute_command(env: &Database<WriteMap>, command: &str) -> Result<String, Box<dyn std::error::Error>> {
    let trimmed_input = command.trim();
    if trimmed_input.is_empty() {
        return Ok(String::new());
    }

    let args = match shell_words::split(trimmed_input) {
        Ok(parsed_args) => parsed_args,
        Err(e) => return Ok(format!("Error parsing input: {}", e)),
    };

    if args.is_empty() {
        return Ok(String::new());
    }

    match args[0].as_str() {
        "list" => {
            if args.len() != 2 {
                return Ok("Usage: list_values <table>".to_string());
            }
            commands::list(env, &args[1])
        }
        "get" => {
            if args.len() != 3 {
                return Ok("Usage: get <table> <key>".to_string());
            }
            commands::get(env, &args[1], &args[2])
        }
        "put" => {
            if args.len() != 4 {
                return Ok("Usage: put <table> <key> <value>".to_string());
            }
            commands::put(env, &args[1], &args[2], &args[3])
        }
        "del" => {
            if args.len() != 3 {
                return Ok("Usage: del <table> <key>".to_string());
            }
            commands::del(env, &args[1], &args[2])
        }
        "list_tables" => {
            commands::list_tables(env)
        }
        "create_table" => {
            if args.len() != 2 {
                return Ok("Usage: create_table <table>".to_string());
            }
            commands::create_table(env, &args[1])
        }
        "empty_table" => {
            if args.len() != 2 {
                return Ok("Usage: empty_table <table>".to_string());
            }
            commands::empty_table(env, &args[1])
        }
        "help" => {
            let mut output = String::new();
            output.push_str("Available commands:\n");
            output.push_str("1. create_table <table> - Creates a new table in the database.\n");
            output.push_str("2. put <table> <key> <value> - Inserts a key-value pair into the specified table.\n");
            output.push_str("3. get <table> <key> - Retrieves the value associated with the specified key in the table.\n");
            output.push_str("4. del <table> <key> - Deletes the specified key from the table.\n");
            output.push_str("5. list <table> - Lists all key-value pairs in the specified table.\n");
            output.push_str("6. list_tables - Lists all tables in the database.\n");
            output.push_str("7. empty_table <table> - Clears all key-value pairs from the specified table.\n");
            output.push_str("8. help - Displays this help message.\n");
            Ok(output)
        }
        _ => Ok("Unknown command.".to_string()),
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the line editor
    let mut rl = Editor::<()>::new()?;
    // Load history; ignore error if file doesn't exist
    if rl.load_history("mdbx_history.txt").is_err() {}

    let args_cli = Cli::parse();

    // Open the MDBX environment
    let db_options = DatabaseOptions {
        max_tables: Some(10),
        ..Default::default()
    };

    let env: Database<WriteMap> = Database::open_with_options(&args_cli.db_path, db_options)?; // Specified WriteMap

    loop {
        // Read input from the user and process commands
        let line = rl.readline("mdbx> ");
        match line {
            Ok(input) => {
                let trimmed_input = input.trim();
                if trimmed_input.is_empty() {
                    continue;
                }
                rl.add_history_entry(trimmed_input);

                match execute_command(&env, trimmed_input) {
                    Ok(output) => {
                        println!("{}", output);
                    }
                    Err(e) => {
                        println!("Error executing command: {}", e);
                    }
                }
            }
            Err(_) => break,
        }
    }

    rl.save_history("mdbx_history.txt")?;
    Ok(())
}
