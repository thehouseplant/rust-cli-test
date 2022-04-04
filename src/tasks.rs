use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(about, version, author)]
struct Value {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Tasks,
}

pub fn tasks() {
    let value = Value::parse();

    match &value.command {
        Commands::Tasks { } => {
            println!("Tasks called")
        },
    }
}