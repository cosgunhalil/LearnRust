use rusqlite::{params, Connection, Result};

pub fn execute() -> Result<()> {
    // Open (or create) a SQLite database
    let conn = Connection::open("local_binary_data.db")?;

    // Create the table if it doesn't already exist
    conn.execute(
        "CREATE TABLE IF NOT EXISTS binary_data (
            id INTEGER PRIMARY KEY,
            data BLOB
        )",
        [],
    )?;

    // The string to be converted to binary and stored
    let input_string = "Sample data";
    let binary_data = input_string.as_bytes();

    // Write the binary data to the database
    write_data(&conn, binary_data)?;

    // Read the binary data from the database
    if let Some(retrieved_data) = read_data(&conn, 1)? {
        // Convert binary data back to a string
        let retrieved_string = String::from_utf8(retrieved_data).unwrap();
        println!("Retrieved string: {}", retrieved_string);
    } else {
        println!("No data found for the given ID.");
    }

    Ok(())
}

// Function to write binary data
fn write_data(conn: &Connection, data: &[u8]) -> Result<()> {
    conn.execute(
        "INSERT INTO binary_data (data) VALUES (?1)",
        params![data],
    )?;
    println!("Data written to database successfully.");
    Ok(())
}

// Function to read binary data
fn read_data(conn: &Connection, id: i32) -> Result<Option<Vec<u8>>> {
    let mut stmt = conn.prepare("SELECT data FROM binary_data WHERE id = ?1")?;
    let mut rows = stmt.query(params![id])?;

    if let Some(row) = rows.next()? {
        let binary_data: Vec<u8> = row.get(0)?;
        Ok(Some(binary_data))
    } else {
        Ok(None)
    }
}
