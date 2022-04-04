use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(about, version, author)]
struct Value {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Withdraw { account_id: i32 },
}

pub fn withdraw() {
    let value = Value::parse();

    match &value.command {
        Commands::Withdraw { account_id } => {
            println!("Withdraw called for {:?}", account_id)
        },
    }
}