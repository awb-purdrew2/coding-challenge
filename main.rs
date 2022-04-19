// Andrew Barthel
// Kraken Coding Challenge
// Driving/Main file for Transaction Engine

// Libraries
use std::env;
use std::process;
mod lib;
use lib::lib::Client;

// Main Function
fn main() {

    // Parse CLI arguments.
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let mut clients : Vec<Client> = Vec::new();

    // Read CSV and parse data.
    if let Err(err) = lib::lib::read_csv(filename.to_string(), &mut clients) {
        println!("Error reading CSV file: {}", err);
        process::exit(1);
    }

    // Print 'headers' of CSV output.
    println!("client, available, held, total, locked");

    // Print all CSV data with formatting defined in challenge handout.
    for client in clients {
        println!("{},{:.4},{:.4},{:.4},{}", client.client_id.to_string(), client.available, client.held, client.held + client.available, client.locked );
    }
    
}
