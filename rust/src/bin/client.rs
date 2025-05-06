use std::env;

use rust::client::client::Client;


fn main() -> std::io::Result<()> {
    let mut addr = String::from("localhost:10001");
    let mut args = env::args();

    if args.len() > 1 {
        // Set the address from the command line argument
        addr = args.nth(1).unwrap();
    }
    let entries = Client::connect(&addr)?;
    //println!("{}", data);

    println!("List of neighbors:");

    let output = Client::format_entries(&entries);
    println!("{}", output);

    Ok(())
}


