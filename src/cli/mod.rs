mod add;
mod database;

use self::add::{add_data, Add};
use self::database::{manage_db, Database};
use clap::{Parser, Subcommand};
use rusqlite::Connection;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Add new record (income/expense) to the database
    #[command(subcommand)]
    Add(Add),

    /// Manage database operations
    #[command(subcommand)]
    Database(Database),
}

pub fn init_cli(conn: Connection) {
    let cli = Cli::parse();

    match cli.command {
        Commands::Add(add) => add_data(add),
        Commands::Database(database) => manage_db(database, conn),
    }
}
