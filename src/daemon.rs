use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(about, version, author)]
struct Value {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Daemon { near_env: String },
}

pub fn daemon() {
    let value = Value::parse();

    match &value.command {
        Commands::Daemon { near_env } => {
            println!("Daemon called for {:?}", near_env)
        },
    }
}