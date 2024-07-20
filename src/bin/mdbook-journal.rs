//
// Generate a MD
//
use clap::{Parser, Subcommand};
use mdbook_journal::{cli_entry, CliLoader, Journal};
use std::path::PathBuf;

fn main() -> anyhow::Result<()> {
    let args = Cli::parse();
    let config_file = std::fs::canonicalize(args.config)?;

    match args.command.unwrap_or_default() {
        Command::Process => {
            todo!("Support mdBook Processing")
        }
        Command::Supports { .. } => {
            // Do nothing for now; we support all
            // render systems I'm pretty sure...
        }
        Command::New { topic } => {
            let journal = Journal::<CliLoader>::load(config_file)?;
            let topic = journal.with_topic(&topic)?;
            let entry = &topic.generate_entry(cli_entry::std_io())?;
            let path = journal.persist_entry(entry)?;
            println!("Entry Created: {}", path.display());
        }
    }

    Ok(())
}

#[derive(Debug, Parser, Clone)]
#[command(name = "mdbook-journal")]
#[command(about = "mdBook journaling system")]
#[command(version)]
struct Cli {
    #[arg(short, long, default_value = "book.toml")]
    config: PathBuf,
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Debug, Subcommand, Clone, Default)]
enum Command {
    /// Called by mdbook to determine render compatability
    #[command(arg_required_else_help = true)]
    Supports {
        /// renderer to check, such as "html"
        renderer: String,
    },
    /// Create a new topic entry
    New {
        /// topic to use
        topic: String,
    },
    /// (default) Process mdbook from stdin
    #[default]
    Process,
}
