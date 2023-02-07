use postgres::{Client, NoTls, Error};

fn main() -> Result<(), Error> {

    let mut client = Client::connect("postgresql://simuam:password@localhost:5432/auth", NoTls)?;
    
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
