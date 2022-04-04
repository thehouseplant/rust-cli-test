use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(about, version, author)]
struct Value {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Update { account_id: i32 },
}

pub fn update() {
    let value = Value::parse();

    match &value.command {
        Commands::Update { account_id } => {
            println!("Update called for {:?}", account_id)
        },
    }
}