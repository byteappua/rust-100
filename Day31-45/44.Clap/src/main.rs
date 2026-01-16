use clap::{Parser, Subcommand};

/// A simple CLI tool to demonstrate clap
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Optional name to greet
    #[arg(short, long, value_name = "NAME")]
    name: Option<String>,

    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Counts words in a string
    Count {
        /// The string to count words from
        #[arg(short, long)]
        input: String,
    },
    /// Echoes the input
    Echo {
        /// The message to echo
        #[arg(short, long)]
        message: String,
        /// Repeat the message n times
        #[arg(short, long, default_value_t = 1)]
        repeat: u8,
    },
}

fn main() {
    let cli = Cli::parse();

    // Check for debug flag
    match cli.debug {
        0 => {},
        1 => println!("Debug mode is on"),
        _ => println!("Debug level: {}", cli.debug),
    }

    if let Some(name) = cli.name {
        println!("Hello, {}!", name);
    }

    match &cli.command {
        Some(Commands::Count { input }) => {
            let count = input.split_whitespace().count();
            println!("Word count: {}", count);
        }
        Some(Commands::Echo { message, repeat }) => {
            for _ in 0..*repeat {
                println!("{}", message);
            }
        }
        None => {
            println!("No subcommand was used");
        }
    }
}
