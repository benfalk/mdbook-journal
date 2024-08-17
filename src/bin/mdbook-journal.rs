//
// Generate a MD
//
use anyhow::Result;
use clap::{Parser, Subcommand, ValueEnum};
use mdbook::preprocess::Preprocessor;
use mdbook_journal::mdbook::preprocessor::{fetch_context, SimpleDirPreprocessor};
use mdbook_journal::{cli_entry, CliLoader, EntryGenerationTrait, Journal};

fn main() -> Result<()> {
    let args = Cli::parse();

    match args.command.unwrap_or_default() {
        Command::Process => {
            let (ctx, book) = fetch_context(std::io::stdin())?;
            let config_file = ctx.root.join("book.toml");
            let journal = Journal::<CliLoader>::load(config_file)?;
            let processor = SimpleDirPreprocessor::new(journal);
            let book = processor.run(&ctx, book)?;
            serde_json::to_writer(std::io::stdout(), &book)?;
        }
        Command::Supports { .. } => {
            // Do nothing for now; we support all
            // render systems I'm pretty sure...
        }
        Command::New { topic, input } => {
            let config_file = std::fs::canonicalize(args.config)?;
            let journal = Journal::<CliLoader>::load(config_file)?;
            let topic = journal.with_topic(&topic)?;
            let entry = &topic.generate_entry(input.collect()?.as_ref())?;
            let path = journal.persist_entry(entry)?;
            println!("Entry Created: {}", path.display());
        }
        Command::Ls { topic } => {
            let config_file = std::fs::canonicalize(args.config)?;
            let journal = Journal::<CliLoader>::load(config_file)?;
            for entry in journal.entries_for_topic(&topic)? {
                if let Some(path) = entry.file_location() {
                    println!("{}", path.display());
                }
            }
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
    config: std::path::PathBuf,
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
        #[arg(short, long, value_enum, default_value = "interactive")]
        input: TopicInput,
    },
    /// List out topics
    Ls {
        /// Topic to list out
        topic: String,
    },
    /// (default) Process mdbook from stdin
    #[default]
    Process,
}

#[derive(Debug, Clone, Default, Copy, ValueEnum)]
enum TopicInput {
    #[default]
    Interactive,
    Json,
}

impl TopicInput {
    fn collect(&self) -> Result<Box<dyn EntryGenerationTrait>> {
        match self {
            Self::Interactive => Ok(Box::new(cli_entry::std_io())),
            Self::Json => {
                let json: serde_json::Value = serde_json::from_reader(std::io::stdin())?;
                Ok(Box::new(json))
            }
        }
    }
}
