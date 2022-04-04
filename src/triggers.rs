use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(about, version, author)]
struct Value {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Triggers,
}

pub fn triggers() {
    let value = Value::parse();

    match &value.command {
        Commands::Triggers { } => {
            println!("Triggers called")
        },
    }
}