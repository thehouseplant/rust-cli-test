use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(about, version, author)]
struct Value {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Register { account_id: i32 },
}

pub fn register() {
    let value = Value::parse();

    match &value.command {
        Commands::Register { account_id } => {
            println!("Register called for {:?}", account_id)
        },
    }
}