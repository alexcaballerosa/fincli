use fincli::{cli::init_cli, config::get_config};
use rusqlite::Connection;

fn main() {
    let config = get_config().expect("Failed to read configuration.");

    let conn = Connection::open(config.database.path).expect("Failed to connect to database.");

    init_cli(conn);
}
