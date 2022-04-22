// Andrew Barthel
// Kraken Coding Challenge
// Utility Functions to aid in Transaction Engine

// Define lib module to use in main.rs.
pub mod lib {

    // Libraries.
    use std::error::Error;
    use serde::Deserialize;

    // Define valid data in CSV to read.
    #[derive(Deserialize)]
    pub struct Row<'a> {
        pub transaction_type: &'a str,
        pub client_id: u16,
        pub transaction_id: u32,
        pub amount: &'a str
    }

    // Define a valid transaction data struct.
    #[derive(Debug)]
    #[derive(Deserialize)]
    pub struct ApprovedTransaction {
        pub transaction_id: u32,
        pub amount: f32,
        pub in_dispute: bool
    }

    // Define a valid client data struct.
    #[derive(Debug)]
    #[derive(Deserialize)]
    pub struct Client {
        pub client_id: u16,
        pub current_transactions: Vec<ApprovedTransaction>,
        pub available: f32,
        pub held: f32,
        pub locked: bool
    }

    // Utility function to read and parse the CSV of transactions.
    pub fn read_csv(filename: String, mut clients: &mut Vec<Client>) -> Result<(), Box<dyn Error>> {
        let mut rdr = csv::Reader::from_path(filename)?;
        for result in rdr.records() {
            let record = result?;
            let row: Row = record.deserialize(None)?;
            process_record(row, &mut clients);
        }
        Ok(())
    }

    // Processes a valid line of data in CSV.
    // It will match data with currently created clients, if not matched - it creates new client.
    fn process_record(row: Row, clients: &mut Vec<Client>) {
        let mut client_found = false;
        let client_id = row.client_id;
        let transaction_id = row.transaction_id;
        let transaction_type = row.transaction_type;
        let mut amount = 0.0;
        if row.amount != "" {
            amount = row.amount.parse().unwrap();
        }
        // Case of client match.
        for mut client in clients.iter_mut() {
            if client.client_id == client_id {
                client_found = true;
                match transaction_type {
                    "deposit"=> process_deposit(row, &mut client),
                    "withdrawal"=> process_withdrawal(row, &mut client),
                    "dispute"=>process_dispute(row, &mut client),
                    "resolve"=>process_resolve(row, &mut client),
                    "chargeback"=>process_chargeback(row, &mut client),
                    _=>(),
                };
                break;
            }
        }
        // Case of a new client needing to be created.
        if !client_found {
            if transaction_type == "deposit"{
                let approved_trans = ApprovedTransaction{
                    transaction_id: transaction_id,
                    amount: amount,
                    in_dispute: false
                };
                let mut new_client = Client {
                    client_id: client_id,
                    available: amount,
                    held: 0.0,
                    locked: false,
                    current_transactions: Vec::new()
                };
                new_client.current_transactions.push(approved_trans);
                clients.push(new_client);
            }
        }
    }

    // Utility function to process a valid deposit transaction.
    fn process_deposit(row: Row, client: &mut Client) {
        if !client.locked {
            let approved_trans = ApprovedTransaction{
                transaction_id: row.transaction_id,
                amount: row.amount.parse().unwrap(),
                in_dispute: false
            };
            client.available += approved_trans.amount;
            client.current_transactions.push(approved_trans);
        }
    }

    // utility function to process a valid withdrawl transaction.
    fn process_withdrawal(row: Row, client: &mut Client) {
        if !client.locked {
            let amount = row.amount.parse().unwrap();
            if client.available >= amount {
                client.available -= amount;
            }
        }
    }

    // Utility function to process valid dispute transaction.
    fn process_dispute(row: Row, client: &mut Client) {
        if !client.locked {
            for trans in client.current_transactions.iter_mut() {
                if trans.transaction_id == row.transaction_id {
                    trans.in_dispute = true;
                    client.available -= trans.amount;
                    client.held += trans.amount;
                }
            }
        }
    }

    // Utility function to process valid resolved transaction.
    fn process_resolve(row: Row, client: &mut Client) {
        if !client.locked {
            for trans in client.current_transactions.iter_mut() {
                if trans.transaction_id == row.transaction_id && trans.in_dispute {
                    trans.in_dispute = false;
                    client.available += trans.amount;
                    client.held -= trans.amount;
                }
            }
        }
    }

    // Utility function to process valid chargeback transaction.
    fn process_chargeback(row: Row, client: &mut Client) {
        if !client.locked {
            for trans in client.current_transactions.iter_mut() {
                if trans.transaction_id == row.transaction_id && trans.in_dispute {
                    client.locked = true;
                    client.held -= trans.amount;
                }
            }
        }
    }
    
}
