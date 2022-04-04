use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(about, version, author)]
struct Value {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Go { account_id: i32 },
}

pub fn go() {
    let value = Value::parse();

    match &value.command {
        Commands::Go { account_id } => {
            println!("Go called for {:?}", account_id)
        },
    }
}