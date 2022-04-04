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
    Update { account_id: i32 },
    Unregister { account_id: i32 },
    Withdraw { account_id: i32 },
    Status { account_id: i32 },
    Tasks,
    Triggers,
    Go { account_id: i32 },
    Daemon { near_env: String },
}

fn main() {
    let value = Value::parse();

    match &value.command {
        Commands::Register { account_id } => {
            println!("Register called for {:?}", account_id)
        },
        Commands::Update { account_id } => {
            println!("Update called for {:?}", account_id)
        },
        Commands::Unregister { account_id } => {
            println!("Unregister called for {:?}", account_id)
        },
        Commands::Withdraw { account_id } => {
            println!("Withdraw called for {:?}", account_id)
        },
        Commands::Status { account_id } => {
            println!("Status called for {:?}", account_id)
        },
        Commands::Tasks { } => {
            println!("Tasks called")
        },
        Commands::Triggers { } => {
            println!("Triggers called")
        },
        Commands::Go { account_id } => {
            println!("Go called for {:?}", account_id)
        },
        Commands::Daemon { near_env } => {
            println!("Daemon called for {:?}", near_env)
        },
    }
}