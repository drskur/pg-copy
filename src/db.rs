use postgres::{Client, NoTls};
use std::fs::File;
use std::io::{BufWriter, BufReader};

pub fn create_client(database_url: &str) -> Client {
    Client::connect(database_url, NoTls)
        .unwrap_or_else(|_| panic!("cannot connect db: {}", database_url))
}

pub fn copy_out(client: &mut Client, table_name: &str, output_path: &str) -> anyhow::Result<()> {
    let query = format!("COPY {} TO STDOUT", table_name);
    let mut r = client.copy_out(query.as_str())?;
    let file = File::create(output_path)?;
    let mut w = BufWriter::new(file);

    println!("Copying out at {}", output_path);
    std::io::copy(&mut r, &mut w)?;

    Ok(())
}

pub fn copy_in(client: &mut Client, table_name: &str, input_path: &str) -> anyhow::Result<()> {
    let query = format!("COPY {} FROM STDIN", table_name);
    let mut w = client.copy_in(query.as_str())?;
    let file = File::open(input_path)?;
    let mut r = BufReader::new(file);

    println!("Copying in from {}", input_path);
    std::io::copy(&mut r, &mut w)?;

    Ok(())
}