use postgres::{Client, NoTls, Error as PostgresError};

//set_database()?; println!("Database setup completed successfully");
const DB_URL: &str = "postgresql://postgres:2233@localhost:5432/postgres";

fn set_database() -> Result<(), PostgresError> {
    let mut client = Client::connect(DB_URL, NoTls)?;

    client.execute(
        "CREATE TABLE IF NOT EXISTS \"user\" (
        id SERIAL PRIMARY KEY,
        name VARCHAR NOT NULL,
        email VARCHAR NOT NULL
    )",
        &[],
    )?;

    Ok(())
}