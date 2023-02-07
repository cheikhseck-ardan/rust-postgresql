use postgres::{Client, NoTls, Error};
use std::env; // will be used to access env. variables.

fn main() -> Result<(), Error> {
	
	let con_string = env::var("SQL_STRING")
	.expect("$SQL_STRING is not set");
    let mut client = Client::connect(&con_string, NoTls)?;
    
    client.batch_execute("
        CREATE TABLE IF NOT EXISTS developer (
            id              SERIAL PRIMARY KEY,
            name            VARCHAR NOT NULL
        )
    ")?;

    let name = "Herbert W.";
	client.execute(
	    "INSERT INTO author (name) VALUES ($1)",
	    &[&name],
	)?;

	for row in client.query("SELECT id, name FROM author", &[])? {
	    let id: i32 = row.get(0);
	    let name: &str = row.get(1);

	    println!("found person: {} {}", id, name);
	}

    Ok(())

}
