use tiny_log_kv::kv_store::KvStore;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about = "Tiny Log-Structured KV Store", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Add or update a key-value pair
    Set {
        key: String,
        value: String,
    },
    /// Get the value for a key
    Get {
        key: String,
    },
    /// Delete a key
    Delete {
        key: String,
    },
}

fn main() {
    let cli = Cli::parse();
    let log_path = "store.log";

    let mut store = KvStore::open(log_path).expect("Failed to open the store!");

    match &cli.command {
        Command::Set { key, value } => {
            store.set(key.to_string(), value.to_string()).expect("Error in set");
            println!("OK");
        }
        Command::Get { key } => {
            match store.get(key) {
                Some(value) => println!("{value}"),
                None => eprintln!("Key not found"),
            }
        }
        Command::Delete { key } => {
            if store.delete(key).expect("Error in delete") {
                println!("Deleted");
            } else {
                println!("Key not found");
            }
        }
    }
}
