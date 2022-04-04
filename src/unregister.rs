use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(about, version, author)]
struct Value {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Unregister { account_id: i32 },
}

pub fn unregister() {
    let value = Value::parse();

    match &value.command {
        Commands::Unregister { account_id } => {
            println!("Unregister called for {:?}", account_id)
        },
    }
}