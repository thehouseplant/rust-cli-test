use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(about, version, author)]
struct Value {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Status { account_id: i32 },
}

pub fn status() {
    let value = Value::parse();

    match &value.command {
        Commands::Status { account_id } => {
            println!("Status called for {:?}", account_id)
        },
    }
}